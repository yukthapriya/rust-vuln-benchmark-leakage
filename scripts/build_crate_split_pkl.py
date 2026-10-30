import os
import json
import pickle
import re
import pandas as pd
from transformers import AutoTokenizer

MODEL_NAME = "microsoft/codebert-base"
MAX_LEN    = 256
OUTPUT_DIR = "ml_crate_split_dataset"
os.makedirs(OUTPUT_DIR, exist_ok=True)

def extract_static_features(code):
    return {
        "n_unsafe":    len(re.findall(r"\bunsafe\b", code)),
        "n_raw_ptr":   len(re.findall(r"\*\s*mut|\*\s*const", code)),
        "n_lifetimes": len(re.findall(r"'\w+", code)),
        "n_loops":     len(re.findall(r"\bfor\b|\bwhile\b", code)),
        "n_functions": len(re.findall(r"\bfn\s+\w+", code)),
        "n_branches":  len(re.findall(r"\bif\b|\bmatch\b", code)),
        "n_lines":     code.count("\n") + 1,
        "n_chars":     len(code),
    }

print("Loading tokenizer...")
tokenizer = AutoTokenizer.from_pretrained(MODEL_NAME)

def process_csv(csv_path, split_name):
    print(f"\nProcessing {split_name} from {csv_path}...")
    df = pd.read_csv(csv_path)
    print(f"  Rows: {len(df)}")

    codes, static_feats, input_ids_list, mask_list = [], [], [], []

    for i, row in df.iterrows():
        with open(row["file_path"], "r", encoding="utf-8", errors="ignore") as f:
            code = f.read()
        codes.append(code)
        static_feats.append(extract_static_features(code))

        enc = tokenizer(
            code,
            truncation=True,
            padding="max_length",
            max_length=MAX_LEN,
            return_tensors="pt"
        )
        input_ids_list.append(enc["input_ids"].squeeze(0))
        mask_list.append(enc["attention_mask"].squeeze(0))

        if (i+1) % 100 == 0:
            print(f"  Tokenized {i+1}/{len(df)}")

    df["code"]             = codes
    df["input_ids"]        = input_ids_list
    df["attention_mask"]   = mask_list
    static_df              = pd.DataFrame(static_feats)
    for col in static_df.columns:
        df[col] = static_df[col].values

    out_path = os.path.join(OUTPUT_DIR, f"{split_name}.pkl")
    with open(out_path, "wb") as f:
        pickle.dump(df, f)
    print(f"  Saved to {out_path}")
    print(f"  vuln/safe: {(df.label=='vulnerable').sum()} / {(df.label=='safe').sum()}")
    return df

train_df = process_csv("crate_split_train.csv", "train")
test_df  = process_csv("crate_split_test.csv",  "test")

metadata = {
    "model_name": MODEL_NAME,
    "max_length": MAX_LEN,
    "num_train":  len(train_df),
    "num_test":   len(test_df),
    "split":      "crate-level"
}
with open(os.path.join(OUTPUT_DIR, "metadata.json"), "w") as f:
    json.dump(metadata, f, indent=4)

print("\n==== DONE ====")
print(f"Train: {len(train_df)} | Test: {len(test_df)}")
print(f"Saved to {OUTPUT_DIR}/")
