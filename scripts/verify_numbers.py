import csv, glob, json, os, re, sys
from collections import Counter, defaultdict
import numpy as np
from sklearn.metrics import (accuracy_score, confusion_matrix, f1_score,
                             matthews_corrcoef)
sys.path.insert(0, '.')
from run_stripping_ladder import load_samples, strip_comments

print("=== 1. AGGREGATION: pooled vs per-fold-mean ===")
print("(figure uses pooled; table used per-fold. Compare.)")
for path in sorted(glob.glob("ladder_results/predictions_*.csv")):
    rows = list(csv.DictReader(open(path)))
    yt = np.array([int(r["y_true"]) for r in rows])
    yp = np.array([int(r["y_pred"]) for r in rows])
    print("  %-52s pooled acc=%.4f mcc=%+.4f" %
          (os.path.basename(path), accuracy_score(yt, yp),
           matthews_corrcoef(yt, yp) if len(set(yp)) > 1 else 0.0))

print("\n=== 2/3. THE 42 EMPTY-AFTER-STRIPPING FILES ===")
s = load_samples('dataset.csv', '.')
empty = [x for x in s if not strip_comments(x['raw']).strip()]
print("  count:", len(empty))
g = Counter(x['group'] for x in empty)
print("  distinct advisories:", len(g))
complete = [k for k, v in g.items() if v == 2]
print("  advisories with BOTH files empty (true pairs):", len(complete))
print("  advisories with only ONE file empty:", sum(1 for v in g.values() if v == 1))

if os.path.exists('all_pairs_sims_v2.json'):
    sims = json.load(open('all_pairs_sims_v2.json'))
    print("  all_pairs_sims_v2.json type:", type(sims).__name__,
          "len:", len(sims))
    print("  first entry:", list(sims.items())[0] if isinstance(sims, dict)
          else sims[0])

print("\n=== 5. E7 CONFUSION MATRICES ===")
if os.path.exists('rustxec_results/results.json'):
    r = json.load(open('rustxec_results/results.json'))
    for k, v in r.items():
        if k.startswith('e7') and isinstance(v, dict):
            print(" ", k, json.dumps(v))
else:
    print("  rustxec_results/results.json not found")

print("\n=== 6. CODEBERT SEED AGGREGATION ===")
for cond in ("raw", "deheadered", "stripped", "header_only"):
    f1s, accs, mccs = [], [], []
    for seed in (42, 1337, 2026):
        p = f"ladder_results/predictions_codebert_{cond}_seed{seed}.csv"
        if not os.path.exists(p):
            continue
        rows = list(csv.DictReader(open(p)))
        yt = np.array([int(x["y_true"]) for x in rows])
        yp = np.array([int(x["y_pred"]) for x in rows])
        f1s.append(f1_score(yt, yp, zero_division=0))
        accs.append(accuracy_score(yt, yp))
        mccs.append(matthews_corrcoef(yt, yp) if len(set(yp)) > 1 else 0.0)
    if f1s:
        print("  %-12s n=%d  F1 %.4f+/-%.4f  acc %.4f+/-%.4f  MCC %+.4f+/-%.4f"
              % (cond, len(f1s), np.mean(f1s), np.std(f1s),
                 np.mean(accs), np.std(accs), np.mean(mccs), np.std(mccs)))
