import pandas as pd
import numpy as np
import re
from sklearn.linear_model import LogisticRegression
from sklearn.ensemble import RandomForestClassifier
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics import f1_score, classification_report

BASE_DIR = "."

def load_code(file_path):
    try:
        with open(file_path, "r", errors="ignore") as f:
            return f.read()
    except:
        return ""

def extract_static(code):
    return [
        len(re.findall(r"\bunsafe\b", code)),
        len(re.findall(r"\*\s*mut|\*\s*const", code)),
        len(re.findall(r"'\w+", code)),
        len(re.findall(r"\bfor\b|\bwhile\b", code)),
        len(re.findall(r"\bfn\s+\w+", code)),
        len(re.findall(r"\bif\b|\bmatch\b", code)),
        code.count("\n") + 1,
        len(code),
    ]

def load_split(csv_path):
    df = pd.read_csv(csv_path)
    texts, labels, static = [], [], []
    skipped = 0
    for _, row in df.iterrows():
        code = load_code(row["file_path"])
        if not code:
            skipped += 1
            continue
        texts.append(code)
        labels.append(1 if row["label"] == "vulnerable" else 0)
        static.append(extract_static(code))
    print(f"  Loaded {len(texts)}, skipped {skipped}")
    return texts, np.array(labels), np.array(static)

print("Loading train...")
train_texts, y_train, train_static = load_split("crate_split_train.csv")
print("Loading test...")
test_texts,  y_test,  test_static  = load_split("crate_split_test.csv")

print(f"\nTrain: {len(y_train)} | Test: {len(y_test)}")
print(f"Train vuln/safe: {y_train.sum()} / {(y_train==0).sum()}")
print(f"Test  vuln/safe: {y_test.sum()}  / {(y_test==0).sum()}")

results = {}

# --- Model 1: TF-IDF + LR ---
print("\n[1/2] Training TF-IDF + LR...")
tfidf = TfidfVectorizer(max_features=10000, ngram_range=(1,2))
X_train_tfidf = tfidf.fit_transform(train_texts)
X_test_tfidf  = tfidf.transform(test_texts)
lr = LogisticRegression(max_iter=1000, random_state=42)
lr.fit(X_train_tfidf, y_train)
preds = lr.predict(X_test_tfidf)
f1 = f1_score(y_test, preds, average="binary")
results["TF-IDF + LR"] = f1
print(classification_report(y_test, preds, target_names=["safe","vulnerable"]))

# --- Model 2: Static RF ---
print("[2/2] Training Static-only RF...")
rf = RandomForestClassifier(n_estimators=100, random_state=42)
rf.fit(train_static, y_train)
preds = rf.predict(test_static)
f1 = f1_score(y_test, preds, average="binary")
results["Static RF"] = f1
print(classification_report(y_test, preds, target_names=["safe","vulnerable"]))

# --- Summary ---
print("\n" + "="*50)
print("RESULTS ON CRATE-LEVEL SPLIT (no leakage)")
print("="*50)
for model, f1 in results.items():
    print(f"{model:30s}  F1 = {f1:.4f}")

print("\nPrevious results (RUSTSEC-level split, with leakage):")
print(f"{'TF-IDF + LR':30s}  F1 = 0.9867")
print(f"{'Static RF':30s}  F1 = 0.7393")
print(f"{'CodeBERT':30s}  F1 = 1.0000  ← was leakage")

# Save results
with open("crate_split_results.txt", "w") as f:
    f.write("CRATE-LEVEL SPLIT RESULTS\n")
    f.write("="*50 + "\n")
    for model, score in results.items():
        f.write(f"{model}: {score:.4f}\n")
print("\nSaved to crate_split_results.txt")