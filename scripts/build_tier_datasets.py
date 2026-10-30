# build_tier_datasets.py
import json
import pandas as pd
import os

df = pd.read_csv("dataset.csv")
with open("clean_pairs.json") as f:
    pairs = json.load(f)

print(f"Total pairs in clean_pairs.json: {len(pairs)}")
sims = [p["body_sim"] for p in pairs]
print(f"Mean body_sim: {sum(sims)/len(sims):.3f}")

tiers = {
    "tier1_header_only": lambda s: s > 0.99,           # expect ~37
    "tier2_tiny":        lambda s: 0.95 < s <= 0.99,   # expect ~28
    "tier3_moderate":    lambda s: 0.70 <= s <= 0.95,  # expect ~133
    "tier4_large":       lambda s: s < 0.70,           # expect ~339
}

# For training: merge tiers 1+2 (too small individually for stable CV)
train_groups = {
    "near_identical": lambda s: s > 0.95,
    "moderate":       lambda s: 0.70 <= s <= 0.95,
    "large_diff":     lambda s: s < 0.70,
}

os.makedirs("tier_datasets", exist_ok=True)

print("\n==== TIER COUNTS (for paper quality table) ====")
for name, cond in tiers.items():
    n = sum(1 for p in pairs if cond(p["body_sim"]))
    print(f"{name}: {n} pairs")

print("\n==== TRAINING DATASETS ====")
for name, cond in train_groups.items():
    selected = [p for p in pairs if cond(p["body_sim"])]
    files = set()
    for p in selected:
        files.add(p["vuln_file"])
        files.add(p["safe_file"])
    tier_df = df[df["file_path"].isin(files)].copy()
    out = f"tier_datasets/{name}.csv"
    tier_df.to_csv(out, index=False)
    v = (tier_df["label"] == "vulnerable").sum()
    s = (tier_df["label"] == "safe").sum()
    print(f"{name}: {len(selected)} pairs, {len(tier_df)} files "
          f"({v} vuln / {s} safe) -> {out}")