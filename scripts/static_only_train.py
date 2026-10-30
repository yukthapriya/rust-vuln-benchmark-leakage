import json
import pandas as pd
from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import classification_report
import joblib
import os

# ---------------------------------------------------------
# Load static features from Rust analyzer
# ---------------------------------------------------------
with open("static_features.json", "r") as f:
    static_data = json.load(f)

static_df = pd.DataFrame(static_data)

# Derive snippet_id from file_path
static_df["snippet_id"] = static_df["file_path"].apply(
    lambda x: os.path.splitext(os.path.basename(x))[0]
)

# ---------------------------------------------------------
# Load labels from dataset.csv
# ---------------------------------------------------------
labels_df = pd.read_csv("dataset.csv")

# Derive snippet_id from file_path in dataset.csv
labels_df["snippet_id"] = labels_df["file_path"].apply(
    lambda x: os.path.splitext(os.path.basename(x))[0]
)

labels_df = labels_df[["snippet_id", "label"]]

# ---------------------------------------------------------
# Merge static features with labels
# ---------------------------------------------------------
df = static_df.merge(labels_df, on="snippet_id", how="left")

# Create binary label
df["label_binary"] = df["label"].apply(lambda x: 1 if x == "vulnerable" else 0)

# ---------------------------------------------------------
# Select static feature columns
# ---------------------------------------------------------
STATIC_FEATURES = [
    "n_unsafe",
    "n_raw_ptr",
    "n_lifetimes",
    "n_functions",
    "n_loops",
    "n_branches",
    "n_lines",
    "n_chars"
]

X = df[STATIC_FEATURES]
y = df["label_binary"]

# ---------------------------------------------------------
# Train static-only model
# ---------------------------------------------------------
clf = RandomForestClassifier(
    n_estimators=300,
    max_depth=None,
    random_state=42
)

clf.fit(X, y)

# ---------------------------------------------------------
# Evaluate
# ---------------------------------------------------------
preds = clf.predict(X)
print("\n=== STATIC-ONLY MODEL PERFORMANCE ===")
print(classification_report(y, preds))

# ---------------------------------------------------------
# Save model
# ---------------------------------------------------------
joblib.dump(clf, "static_only_model.pkl")
print("\nSaved static-only model to static_only_model.pkl")
