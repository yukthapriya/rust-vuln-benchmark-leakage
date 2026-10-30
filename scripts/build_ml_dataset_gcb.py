"""
build_ml_dataset_pub.py
=======================
 Rust vulnerability detection.

Key fixes over original:
  1. Removes 122 cross-label exact duplicates before anything else
  2. Splits at RUSTSEC ID level (not file level) to prevent data leakage
  3. Produces BOTH dirty and clean datasets for RQ2 (impact of quality issues)
  4. Fixed random seed = 42 everywhere
  5. Saves full provenance metadata
"""

import os, re, json, pickle, hashlib, random
import numpy as np
import pandas as pd
from transformers import AutoTokenizer

# ── reproducibility ───────────────────────────────────────────────────────────
SEED = 42
random.seed(SEED)
np.random.seed(SEED)

# ── config ────────────────────────────────────────────────────────────────────
CSV_PATH    = "dataset.csv"
MODEL_NAME  = "microsoft/graphcodebert-base"
MAX_LEN     = 256
OUT_DIRTY   = "ml_dataset_dirty_gcb"
OUT_CLEAN   = "ml_dataset_clean_gcb"

os.makedirs(OUT_DIRTY, exist_ok=True)
os.makedirs(OUT_CLEAN, exist_ok=True)

STATIC_FEATURES = [
    "n_unsafe", "n_raw_ptr", "n_lifetimes", "n_loops",
    "n_functions", "n_branches", "n_lines", "n_chars"
]

# ── helpers ───────────────────────────────────────────────────────────────────

def load_code(path):
    with open(path, "r", encoding="utf-8", errors="ignore") as f:
        return f.read()

def normalize(text):
    lines = [l.strip() for l in text.splitlines()
             if l.strip() and not l.strip().startswith("//")
             and not l.strip().startswith("/*")]
    return "\n".join(lines)

def sha256(text):
    return hashlib.sha256(text.encode("utf-8", errors="replace")).hexdigest()

def extract_static(code):
    return {
        "n_unsafe":   len(re.findall(r"\bunsafe\b", code)),
        "n_raw_ptr":  len(re.findall(r"\*\s*mut|\*\s*const", code)),
        "n_lifetimes":len(re.findall(r"'\w+", code)),
        "n_loops":    len(re.findall(r"\bfor\b|\bwhile\b", code)),
        "n_functions":len(re.findall(r"\bfn\s+\w+", code)),
        "n_branches": len(re.findall(r"\bif\b|\bmatch\b", code)),
        "n_lines":    code.count("\n") + 1,
        "n_chars":    len(code),
    }

def tokenize_code(code, tokenizer):
    enc = tokenizer(
        code, truncation=True, padding="max_length",
        max_length=MAX_LEN, return_tensors="pt"
    )
    return enc["input_ids"].squeeze(0), enc["attention_mask"].squeeze(0)

def save_pkl(obj, path):
    with open(path, "wb") as f:
        pickle.dump(obj, f)

# ── load dataset ──────────────────────────────────────────────────────────────

print("=" * 60)
print("Step 1 — Loading dataset")
print("=" * 60)

df = pd.read_csv(CSV_PATH)
print(f"  Loaded {len(df)} rows")

# load code + static features
codes, statics = [], []
for _, row in df.iterrows():
    code = load_code(row["file_path"])
    codes.append(code)
    statics.append(extract_static(code))

df["code"] = codes
df["norm_hash"] = [sha256(normalize(c)) for c in codes]
static_df = pd.DataFrame(statics)
df = pd.concat([df.reset_index(drop=True), static_df], axis=1)

print(f"  Code loaded and static features extracted for all {len(df)} files")

# ── identify quality issues ───────────────────────────────────────────────────

print("\nStep 2 — Identifying quality issues")

# cross-label exact duplicates
hash_label = df.groupby("norm_hash")["label"].nunique()
cross_label_hashes = set(hash_label[hash_label > 1].index)
cross_label_mask = df["norm_hash"].isin(cross_label_hashes)
n_cross = cross_label_mask.sum()
print(f"  Cross-label exact duplicates: {n_cross} files ({len(cross_label_hashes)} groups)")

# same-class exact duplicates
dup_mask = df.duplicated(subset=["norm_hash", "label"], keep="first")
n_dup = dup_mask.sum()
print(f"  Same-class redundant duplicates: {n_dup} files")

# trivial files
trivial_mask = df["n_lines"] < 20
print(f"  Trivial files (<20 lines): {trivial_mask.sum()}")

# rustsec pairs with identical content
rustsec_pairs = df.groupby("rustsec_id")["norm_hash"].nunique()
identical_pairs = (rustsec_pairs == 1).sum()
print(f"  RUSTSEC pairs with identical vuln/safe content: {identical_pairs}")

# ── build DIRTY dataset (original, no cleaning) ───────────────────────────────

print("\nStep 3 — Building DIRTY dataset (original, no cleaning)")

df_dirty = df.copy()
print(f"  Dirty dataset size: {len(df_dirty)}")

# RUSTSEC-level split — all files from same RUSTSEC go to same split
rustsec_ids = df_dirty["rustsec_id"].unique()
np.random.shuffle(rustsec_ids)
split_point = int(len(rustsec_ids) * 0.8)
train_rustsec = set(rustsec_ids[:split_point])
test_rustsec  = set(rustsec_ids[split_point:])

train_dirty = df_dirty[df_dirty["rustsec_id"].isin(train_rustsec)].copy()
test_dirty  = df_dirty[df_dirty["rustsec_id"].isin(test_rustsec)].copy()

print(f"  Train: {len(train_dirty)} | Test: {len(test_dirty)}")
print(f"  Train RUSTSEC IDs: {len(train_rustsec)} | Test RUSTSEC IDs: {len(test_rustsec)}")

# ── build CLEAN dataset ───────────────────────────────────────────────────────

print("\nStep 4 — Building CLEAN dataset")

# Step 4a: remove cross-label exact duplicates
df_clean = df[~cross_label_mask].copy()
print(f"  After removing cross-label duplicates: {len(df_clean)} files")

# Step 4b: remove same-class redundant duplicates
df_clean = df_clean[~df_clean.duplicated(subset=["norm_hash", "label"], keep="first")].copy()
print(f"  After removing same-class duplicates: {len(df_clean)} files")

# Step 4c: remove trivial files
df_clean = df_clean[df_clean["n_lines"] >= 20].copy()
print(f"  After removing trivial files (<20 lines): {len(df_clean)} files")

# RUSTSEC-level split on clean data
rustsec_ids_clean = df_clean["rustsec_id"].unique()
np.random.shuffle(rustsec_ids_clean)
split_point_clean = int(len(rustsec_ids_clean) * 0.8)
train_rustsec_clean = set(rustsec_ids_clean[:split_point_clean])
test_rustsec_clean  = set(rustsec_ids_clean[split_point_clean:])

train_clean = df_clean[df_clean["rustsec_id"].isin(train_rustsec_clean)].copy()
test_clean  = df_clean[df_clean["rustsec_id"].isin(test_rustsec_clean)].copy()

print(f"  Train: {len(train_clean)} | Test: {len(test_clean)}")
print(f"  Train label dist: {train_clean['label'].value_counts().to_dict()}")
print(f"  Test label dist:  {test_clean['label'].value_counts().to_dict()}")

# ── tokenize ──────────────────────────────────────────────────────────────────

print("\nStep 5 — Tokenizing (CodeBERT)")
tokenizer = AutoTokenizer.from_pretrained(MODEL_NAME)

def add_tokens(df_split):
    ids_list, mask_list, stripped_list = [], [], []
    for code in df_split["code"]:
        stripped = normalize(code)
        stripped_list.append(stripped)
        ids, mask = tokenize_code(stripped, tokenizer)
        ids_list.append(ids)
        mask_list.append(mask)
    df_split = df_split.copy()
    df_split["code"]           = stripped_list
    df_split["input_ids"]      = ids_list
    df_split["attention_mask"] = mask_list
    return df_split

print("  Tokenizing dirty train/test...")
train_dirty = add_tokens(train_dirty)
test_dirty  = add_tokens(test_dirty)

print("  Tokenizing clean train/test...")
train_clean = add_tokens(train_clean)
test_clean  = add_tokens(test_clean)

# ── save ──────────────────────────────────────────────────────────────────────

print("\nStep 6 — Saving datasets")

save_pkl(train_dirty, f"{OUT_DIRTY}/train.pkl")
save_pkl(test_dirty,  f"{OUT_DIRTY}/test.pkl")
save_pkl(train_clean, f"{OUT_CLEAN}/train.pkl")
save_pkl(test_clean,  f"{OUT_CLEAN}/test.pkl")

def save_meta(out_dir, train, test, label):
    meta = {
        "dataset_version": label,
        "model_name": MODEL_NAME,
        "max_length": MAX_LEN,
        "seed": SEED,
        "split_strategy": "rustsec_id_level",
        "num_train": len(train),
        "num_test":  len(test),
        "train_label_dist": train["label"].value_counts().to_dict(),
        "test_label_dist":  test["label"].value_counts().to_dict(),
        "static_features": STATIC_FEATURES,
    }
    with open(f"{out_dir}/metadata.json", "w") as f:
        json.dump(meta, f, indent=2)

save_meta(OUT_DIRTY, train_dirty, test_dirty, "dirty_original")
save_meta(OUT_CLEAN, train_clean, test_clean, "clean_deduplicated")

# ── quality issue summary ─────────────────────────────────────────────────────

quality_report = {
    "total_original": len(df),
    "cross_label_exact_duplicates": int(n_cross),
    "same_class_redundant_duplicates": int(n_dup),
    "trivial_files_under_20_lines": int(trivial_mask.sum()),
    "rustsec_pairs_identical_content": int(identical_pairs),
    "dirty_train": len(train_dirty),
    "dirty_test":  len(test_dirty),
    "clean_after_removal": len(df_clean),
    "clean_train": len(train_clean),
    "clean_test":  len(test_clean),
}
with open("quality_issues_summary.json", "w") as f:
    json.dump(quality_report, f, indent=2)

print("\n" + "=" * 60)
print("DATASET BUILD COMPLETE")
print("=" * 60)
print(f"  Dirty  → {OUT_DIRTY}/  (train={len(train_dirty)}, test={len(test_dirty)})")
print(f"  Clean  → {OUT_CLEAN}/  (train={len(train_clean)}, test={len(test_clean)})")
print(f"  Quality summary → quality_issues_summary.json")
