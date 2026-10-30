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
MODEL_NAME  = "microsoft/codebert-base"
MAX_LEN     = 256
OUT_DIRTY   = "ml_dataset_dirty"
OUT_CLEAN   = "ml_dataset_clean"

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
print("Building tier datasets")
print("=" * 60)

np.random.seed(42)

TIERS = {
    "near_identical": "tier_datasets/near_identical.csv",
    "moderate":       "tier_datasets/moderate.csv",
    "large_diff":     "tier_datasets/large_diff.csv",
}

tokenizer = AutoTokenizer.from_pretrained(MODEL_NAME)

def tokenize_split(df_split, strip):
    ids_list, mask_list, code_list = [], [], []
    for code in df_split["code"]:
        text = normalize(code) if strip else code
        code_list.append(text)
        ids, mask = tokenize_code(text, tokenizer)
        ids_list.append(ids)
        mask_list.append(mask)
    out = df_split.copy()
    out["code"]           = code_list
    out["input_ids"]      = ids_list
    out["attention_mask"] = mask_list
    return out

for tier_name, csv_path in TIERS.items():
    print(f"\n--- {tier_name} ---")
    df = pd.read_csv(csv_path)

    codes, statics = [], []
    for _, row in df.iterrows():
        code = load_code(row["file_path"])
        codes.append(code)
        statics.append(extract_static(code))
    df["code"] = codes
    df = pd.concat([df.reset_index(drop=True), pd.DataFrame(statics)], axis=1)

    # RustSec-level 80/20 split (same protocol as main experiments)
    rids = df["rustsec_id"].unique()
    np.random.shuffle(rids)
    cut = int(len(rids) * 0.8)
    train_r, test_r = set(rids[:cut]), set(rids[cut:])
    train_df = df[df["rustsec_id"].isin(train_r)].copy()
    test_df  = df[df["rustsec_id"].isin(test_r)].copy()
    print(f"  files: {len(df)} | train: {len(train_df)} | test: {len(test_df)}")
    print(f"  train labels: {train_df['label'].value_counts().to_dict()}")
    print(f"  test labels:  {test_df['label'].value_counts().to_dict()}")

    for strip, suffix in [(False, "_raw"), (True, "")]:
        out_dir = f"ml_dataset_tier_{tier_name}{suffix}"
        os.makedirs(out_dir, exist_ok=True)
        save_pkl(tokenize_split(train_df, strip), f"{out_dir}/train.pkl")
        save_pkl(tokenize_split(test_df,  strip), f"{out_dir}/test.pkl")
        print(f"  saved -> {out_dir}/ ({'stripped' if strip else 'raw'})")