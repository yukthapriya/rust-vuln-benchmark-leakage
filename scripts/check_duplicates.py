import pandas as pd
import json
import os

print("\n=== Checking dataset.csv for duplicates ===")
df = pd.read_csv("dataset.csv")

# Check duplicate file paths
dup_paths = df["file_path"].duplicated().sum()
print(f"Duplicate file paths: {dup_paths}")

# Derive snippet_id
df["snippet_id"] = df["file_path"].apply(lambda x: os.path.splitext(os.path.basename(x))[0])
dup_snippets = df["snippet_id"].duplicated().sum()
print(f"Duplicate snippet_ids: {dup_snippets}")

print("\n=== Checking static_features.json for duplicates ===")
with open("static_features.json") as f:
    data = json.load(f)

static_df = pd.DataFrame(data)
static_df["snippet_id"] = static_df["file_path"].apply(lambda x: os.path.splitext(os.path.basename(x))[0])

dup_static_snippets = static_df["snippet_id"].duplicated().sum()
print(f"Duplicate snippet_ids in static features: {dup_static_snippets}")

print("\n=== Summary ===")
if dup_paths == 0 and dup_snippets == 0 and dup_static_snippets == 0:
    print("No duplicates found in dataset.csv or static_features.json.")
else:
    print("Duplicates detected. Check above counts.")
