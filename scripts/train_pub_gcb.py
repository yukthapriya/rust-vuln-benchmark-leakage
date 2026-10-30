"""
train_pub.py
============
Publication-grade training and evaluation for Rust vulnerability detection.

Runs ALL experiments needed for the paper:
  - 5-fold cross-validation (RUSTSEC-level stratified)
  - Fixed random seed = 42
  - Three models: TF-IDF baseline, ML-only (CodeBERT), Hybrid (CodeBERT + static)
  - On BOTH dirty and clean datasets (RQ2: impact of data quality)
  - Saves all results to results_pub_gcb.json and results_pub_gcb.txt

Models compared:
  1. TF-IDF + Logistic Regression (weak baseline)
  2. Static-only Random Forest (your Rust analyzer features)
  3. CodeBERT fine-tuned (ML-only)
  4. CodeBERT + static features (Hybrid)
"""

import os, json, pickle, random, warnings
import numpy as np
import pandas as pd
import torch
import torch.nn as nn
import torch.optim as optim
from transformers import AutoModel
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.linear_model import LogisticRegression
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import StratifiedKFold
from sklearn.metrics import (accuracy_score, precision_score,
                              recall_score, f1_score, classification_report)
from sklearn.preprocessing import StandardScaler
warnings.filterwarnings("ignore")

# ── reproducibility ───────────────────────────────────────────────────────────
SEED = 42
random.seed(SEED)
np.random.seed(SEED)
torch.manual_seed(SEED)
torch.backends.cudnn.deterministic = True

# ── config ────────────────────────────────────────────────────────────────────
MODEL_NAME = "microsoft/graphcodebert-base"
EPOCHS     = 3
BATCH_SIZE = 8
LR         = 2e-5
N_FOLDS    = 5

STATIC_FEATURES = [
    "n_unsafe", "n_raw_ptr", "n_lifetimes", "n_loops",
    "n_functions", "n_branches", "n_lines", "n_chars"
]

DATASETS = {
    "dirty": "ml_dataset_dirty_gcb",
    "clean": "ml_dataset_clean_gcb",
}

# ── model definitions ─────────────────────────────────────────────────────────

class MLOnlyModel(nn.Module):
    def __init__(self, model_name):
        super().__init__()
        self.encoder    = AutoModel.from_pretrained(model_name)
        self.classifier = nn.Linear(self.encoder.config.hidden_size, 2)

    def forward(self, ids, mask):
        out = self.encoder(input_ids=ids, attention_mask=mask)
        return self.classifier(out.last_hidden_state[:, 0, :])

class HybridModel(nn.Module):
    def __init__(self, model_name, static_dim, hidden_dim=128):
        super().__init__()
        self.encoder    = AutoModel.from_pretrained(model_name)
        cbert_dim       = self.encoder.config.hidden_size
        self.classifier = nn.Sequential(
            nn.Linear(cbert_dim + static_dim, hidden_dim),
            nn.ReLU(),
            nn.Dropout(0.1),
            nn.Linear(hidden_dim, 2)
        )

    def forward(self, ids, mask, static_feats):
        out     = self.encoder(input_ids=ids, attention_mask=mask)
        cls     = out.last_hidden_state[:, 0, :]
        combined = torch.cat([cls, static_feats], dim=1)
        return self.classifier(combined)

# ── training helpers ──────────────────────────────────────────────────────────

def train_torch(model, ids, masks, labels, static=None, epochs=EPOCHS):
    torch.manual_seed(SEED)
    criterion = nn.CrossEntropyLoss()
    optimizer = optim.Adam(model.parameters(), lr=LR)
    model.train()
    for epoch in range(epochs):
        perm       = torch.randperm(ids.size(0))
        total_loss = 0
        for i in range(0, ids.size(0), BATCH_SIZE):
            idx      = perm[i:i+BATCH_SIZE]
            b_ids    = ids[idx]
            b_mask   = masks[idx]
            b_y      = labels[idx]
            optimizer.zero_grad()
            if static is not None:
                logits = model(b_ids, b_mask, static[idx])
            else:
                logits = model(b_ids, b_mask)
            loss = criterion(logits, b_y)
            loss.backward()
            optimizer.step()
            total_loss += loss.item() * len(b_y)
        avg = total_loss / ids.size(0)
        print(f"      Epoch {epoch+1}/{epochs} loss={avg:.4f}")

def eval_torch(model, ids, masks, static=None):
    model.eval()
    with torch.no_grad():
        if static is not None:
            logits = model(ids, masks, static)
        else:
            logits = model(ids, masks)
    return torch.argmax(logits, dim=1).numpy()

def metrics(y_true, y_pred):
    return {
        "accuracy":  round(accuracy_score(y_true, y_pred),  4),
        "precision": round(precision_score(y_true, y_pred, zero_division=0), 4),
        "recall":    round(recall_score(y_true, y_pred, zero_division=0),    4),
        "f1":        round(f1_score(y_true, y_pred, zero_division=0),        4),
    }

def summarise(fold_results):
    keys = fold_results[0].keys()
    return {
        k: {
            "mean": round(float(np.mean([r[k] for r in fold_results])), 4),
            "std":  round(float(np.std( [r[k] for r in fold_results])), 4),
        }
        for k in keys
    }

# ── main experiment loop ──────────────────────────────────────────────────────

all_results = {}
log_lines   = []

def log(msg):
    print(msg)
    log_lines.append(msg)

for dataset_name, data_dir in DATASETS.items():

    log(f"\n{'='*60}")
    log(f"DATASET: {dataset_name.upper()}  ({data_dir}/)")
    log(f"{'='*60}")

    # load
    with open(f"{data_dir}/train.pkl", "rb") as f:
        train_df = pickle.load(f)
    with open(f"{data_dir}/test.pkl", "rb") as f:
        test_df  = pickle.load(f)

    # combine for CV (use all data, split by RUSTSEC)
    full_df = pd.concat([train_df, test_df], ignore_index=True)

    # binary label
    full_df["label_bin"] = (full_df["label"] == "vulnerable").astype(int)

    # RUSTSEC-level 5-fold
    rustsec_ids   = full_df["rustsec_id"].unique()
    np.random.shuffle(rustsec_ids)
    rustsec_folds = np.array_split(rustsec_ids, N_FOLDS)

    log(f"  Total samples: {len(full_df)}")
    log(f"  RUSTSEC IDs:   {len(rustsec_ids)}")
    log(f"  Folds:         {N_FOLDS} (RUSTSEC-level stratified)")

    dataset_results = {}

    # ── Model 1: TF-IDF + Logistic Regression (baseline) ────────────────────

    log(f"\n  --- Model 1: TF-IDF + Logistic Regression (baseline) ---")
    tfidf_folds = []
    for fold_i, test_rustsec in enumerate(rustsec_folds):
        train_rustsec = np.concatenate([rustsec_folds[j]
                                        for j in range(N_FOLDS) if j != fold_i])
        tr = full_df[full_df["rustsec_id"].isin(train_rustsec)]
        te = full_df[full_df["rustsec_id"].isin(test_rustsec)]

        vec = TfidfVectorizer(max_features=10000, sublinear_tf=True)
        X_tr = vec.fit_transform(tr["code"])
        X_te = vec.transform(te["code"])
        y_tr = tr["label_bin"].values
        y_te = te["label_bin"].values

        clf = LogisticRegression(max_iter=1000, random_state=SEED, C=1.0)
        clf.fit(X_tr, y_tr)
        preds = clf.predict(X_te)
        m = metrics(y_te, preds)
        tfidf_folds.append(m)
        log(f"    Fold {fold_i+1}: F1={m['f1']:.4f}  Acc={m['accuracy']:.4f}")

    dataset_results["tfidf_lr"] = summarise(tfidf_folds)
    s = dataset_results["tfidf_lr"]
    log(f"    MEAN: F1={s['f1']['mean']:.4f}±{s['f1']['std']:.4f}  "
        f"Acc={s['accuracy']['mean']:.4f}±{s['accuracy']['std']:.4f}")

    # ── Model 2: Static-only Random Forest ───────────────────────────────────

    log(f"\n  --- Model 2: Static-only Random Forest ---")
    static_folds = []
    for fold_i, test_rustsec in enumerate(rustsec_folds):
        train_rustsec = np.concatenate([rustsec_folds[j]
                                        for j in range(N_FOLDS) if j != fold_i])
        tr = full_df[full_df["rustsec_id"].isin(train_rustsec)]
        te = full_df[full_df["rustsec_id"].isin(test_rustsec)]

        scaler = StandardScaler()
        X_tr   = scaler.fit_transform(tr[STATIC_FEATURES].values.astype("float32"))
        X_te   = scaler.transform(te[STATIC_FEATURES].values.astype("float32"))
        y_tr   = tr["label_bin"].values
        y_te   = te["label_bin"].values

        clf = RandomForestClassifier(n_estimators=300, random_state=SEED)
        clf.fit(X_tr, y_tr)
        preds = clf.predict(X_te)
        m = metrics(y_te, preds)
        static_folds.append(m)
        log(f"    Fold {fold_i+1}: F1={m['f1']:.4f}  Acc={m['accuracy']:.4f}")

    dataset_results["static_rf"] = summarise(static_folds)
    s = dataset_results["static_rf"]
    log(f"    MEAN: F1={s['f1']['mean']:.4f}±{s['f1']['std']:.4f}  "
        f"Acc={s['accuracy']['mean']:.4f}±{s['accuracy']['std']:.4f}")

    # ── Model 3: CodeBERT (ML-only) ───────────────────────────────────────────

    log(f"\n  --- Model 3: CodeBERT ML-only ---")
    ml_folds = []
    for fold_i, test_rustsec in enumerate(rustsec_folds):
        log(f"    Fold {fold_i+1}/{N_FOLDS} ...")
        train_rustsec = np.concatenate([rustsec_folds[j]
                                        for j in range(N_FOLDS) if j != fold_i])
        tr = full_df[full_df["rustsec_id"].isin(train_rustsec)]
        te = full_df[full_df["rustsec_id"].isin(test_rustsec)]

        tr_ids    = torch.stack(tr["input_ids"].tolist())
        tr_mask   = torch.stack(tr["attention_mask"].tolist())
        tr_y      = torch.tensor(tr["label_bin"].values)
        te_ids    = torch.stack(te["input_ids"].tolist())
        te_mask   = torch.stack(te["attention_mask"].tolist())
        te_y      = te["label_bin"].values

        model = MLOnlyModel(MODEL_NAME)
        train_torch(model, tr_ids, tr_mask, tr_y)
        preds = eval_torch(model, te_ids, te_mask)
        m = metrics(te_y, preds)
        ml_folds.append(m)
        log(f"      F1={m['f1']:.4f}  Acc={m['accuracy']:.4f}  "
            f"Prec={m['precision']:.4f}  Rec={m['recall']:.4f}")

    dataset_results["codebert_ml"] = summarise(ml_folds)
    s = dataset_results["codebert_ml"]
    log(f"    MEAN: F1={s['f1']['mean']:.4f}±{s['f1']['std']:.4f}  "
        f"Acc={s['accuracy']['mean']:.4f}±{s['accuracy']['std']:.4f}")

    # ── Model 4: Hybrid (CodeBERT + static) ──────────────────────────────────

    log(f"\n  --- Model 4: Hybrid CodeBERT + Static ---")
    hybrid_folds = []
    for fold_i, test_rustsec in enumerate(rustsec_folds):
        log(f"    Fold {fold_i+1}/{N_FOLDS} ...")
        train_rustsec = np.concatenate([rustsec_folds[j]
                                        for j in range(N_FOLDS) if j != fold_i])
        tr = full_df[full_df["rustsec_id"].isin(train_rustsec)]
        te = full_df[full_df["rustsec_id"].isin(test_rustsec)]

        tr_ids    = torch.stack(tr["input_ids"].tolist())
        tr_mask   = torch.stack(tr["attention_mask"].tolist())
        tr_static = torch.tensor(tr[STATIC_FEATURES].values.astype("float32"))
        tr_y      = torch.tensor(tr["label_bin"].values)
        te_ids    = torch.stack(te["input_ids"].tolist())
        te_mask   = torch.stack(te["attention_mask"].tolist())
        te_static = torch.tensor(te[STATIC_FEATURES].values.astype("float32"))
        te_y      = te["label_bin"].values

        model = HybridModel(MODEL_NAME, static_dim=len(STATIC_FEATURES))
        train_torch(model, tr_ids, tr_mask, tr_y, static=tr_static)
        preds = eval_torch(model, te_ids, te_mask, static=te_static)
        m = metrics(te_y, preds)
        hybrid_folds.append(m)
        log(f"      F1={m['f1']:.4f}  Acc={m['accuracy']:.4f}  "
            f"Prec={m['precision']:.4f}  Rec={m['recall']:.4f}")

    dataset_results["hybrid"] = summarise(hybrid_folds)
    s = dataset_results["hybrid"]
    log(f"    MEAN: F1={s['f1']['mean']:.4f}±{s['f1']['std']:.4f}  "
        f"Acc={s['accuracy']['mean']:.4f}±{s['accuracy']['std']:.4f}")

    all_results[dataset_name] = dataset_results

# ── final comparison table ────────────────────────────────────────────────────

log(f"\n{'='*60}")
log("FINAL RESULTS SUMMARY")
log(f"{'='*60}")
log(f"{'Model':<25} {'Dataset':<8} {'F1 (mean±std)':<18} {'Acc (mean±std)':<18} {'Prec':<10} {'Rec':<10}")
log("-" * 90)

model_labels = {
    "tfidf_lr":   "TF-IDF + LR (baseline)",
    "static_rf":  "Static-only RF",
    "codebert_ml":"CodeBERT ML-only",
    "hybrid":     "Hybrid (CodeBERT+Static)",
}

for model_key, model_label in model_labels.items():
    for ds_name in ["dirty", "clean"]:
        r = all_results[ds_name][model_key]
        log(f"{model_label:<25} {ds_name:<8} "
            f"{r['f1']['mean']:.4f}±{r['f1']['std']:.4f}      "
            f"{r['accuracy']['mean']:.4f}±{r['accuracy']['std']:.4f}      "
            f"{r['precision']['mean']:.4f}     "
            f"{r['recall']['mean']:.4f}")

log(f"\nNote: All results are 5-fold CV with RUSTSEC-level splits, seed={SEED}")

# ── save results ──────────────────────────────────────────────────────────────

with open("results_pub_gcb.json", "w") as f:
    json.dump(all_results, f, indent=2)

with open("results_pub_gcb.txt", "w") as f:
    f.write("\n".join(log_lines))

print(f"\nResults saved to:")
print(f"  results_pub_gcb.json  — full structured results")
print(f"  results_pub_gcb.txt   — full log")
