import glob, pandas as pd, numpy as np
from sklearn.model_selection import GroupKFold
from sklearn.metrics import f1_score, accuracy_score, matthews_corrcoef
for f in sorted(glob.glob("**/predictions_*.csv", recursive=True)):
    if "venv" in f: continue
    d = pd.read_csv(f); y, p = d["y_true"], d["y_pred"]
    pooled = f"pooled F1={f1_score(y,p,zero_division=0):.4f} acc={accuracy_score(y,p):.4f} MCC={matthews_corrcoef(y,p):+.4f}"
    g = []
    for _, te in GroupKFold(n_splits=5).split(d, y, d["group"]):
        s = d.iloc[te]
        g.append((f1_score(s.y_true,s.y_pred,zero_division=0), accuracy_score(s.y_true,s.y_pred),
                  matthews_corrcoef(s.y_true,s.y_pred), s.y_pred.nunique()==1))
    a = np.array([x[:3] for x in g])
    print(f"{f.split('/')[-1]:46s} {pooled} | per-fold F1={a[:,0].mean():.4f}±{a[:,0].std():.4f} "
          f"acc={a[:,1].mean():.4f}±{a[:,1].std():.4f} MCC={a[:,2].mean():+.4f} collapsed={sum(x[3] for x in g)}/5")
