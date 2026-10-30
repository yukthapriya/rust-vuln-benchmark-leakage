#!/usr/bin/env python3
"""E1 (header-only control) + E2 (stripping ladder) on RustDataset_v4."""

import argparse, csv, json, os, re, sys
import numpy as np

csv.field_size_limit(sys.maxsize)


def split_header(text):
    """(header, body). Header = leading contiguous run of // lines at top."""
    lines = text.splitlines()
    i = 0
    while i < len(lines) and not lines[i].strip():
        i += 1
    start = i
    while i < len(lines) and lines[i].lstrip().startswith("//"):
        i += 1
    if i == start:
        return "", text
    return "\n".join(lines[start:i]), "\n".join(lines[i:])


def strip_comments(code):
    """Remove // and /* */ comments, respecting string/char literals."""
    out, i, n = [], 0, len(code)
    in_line = in_block = in_str = in_char = False
    raw_hashes = -1
    while i < n:
        c = code[i]
        nxt = code[i + 1] if i + 1 < n else ""
        if in_line:
            if c == "\n":
                in_line = False
                out.append(c)
            i += 1
            continue
        if in_block:
            if c == "*" and nxt == "/":
                in_block = False
                i += 2
                continue
            if c == "\n":
                out.append(c)
            i += 1
            continue
        if in_str:
            out.append(c)
            if raw_hashes >= 0:
                if c == '"' and code[i + 1:i + 1 + raw_hashes] == "#" * raw_hashes:
                    out.append(code[i + 1:i + 1 + raw_hashes])
                    i += 1 + raw_hashes
                    in_str = False
                    raw_hashes = -1
                    continue
            else:
                if c == "\\":
                    if i + 1 < n:
                        out.append(nxt)
                    i += 2
                    continue
                if c == '"':
                    in_str = False
            i += 1
            continue
        if in_char:
            out.append(c)
            if c == "\\":
                if i + 1 < n:
                    out.append(nxt)
                i += 2
                continue
            if c == "'":
                in_char = False
            i += 1
            continue
        m = re.match(r'r(#*)"', code[i:])
        if m:
            raw_hashes = len(m.group(1))
            out.append(m.group(0))
            i += len(m.group(0))
            in_str = True
            continue
        if c == "/" and nxt == "/":
            in_line = True
            i += 2
            continue
        if c == "/" and nxt == "*":
            in_block = True
            i += 2
            continue
        if c == '"':
            in_str = True
            out.append(c)
            i += 1
            continue
        if c == "'" and not re.match(r"'[a-zA-Z_][a-zA-Z0-9_]*\b(?!')", code[i:]):
            in_char = True
            out.append(c)
            i += 1
            continue
        out.append(c)
        i += 1
    text = "".join(out)
    return "\n".join(ln for ln in text.splitlines() if ln.strip())


def make_condition(text, cond):
    header, body = split_header(text)
    if cond == "raw":
        return text
    if cond == "deheadered":
        return body
    if cond == "stripped":
        return strip_comments(text)
    if cond == "header_only":
        return header
    raise ValueError(cond)


PATH_KEYS = ("path", "file", "filepath", "file_path", "filename")
LABEL_KEYS = ("label", "is_vulnerable", "vulnerable", "target", "y")
GROUP_KEYS = ("rustsec_id", "rustsec", "advisory", "advisory_id", "id")
CRATE_KEYS = ("crate", "crate_name", "package")


def pick(cols, keys):
    low = {c.lower(): c for c in cols}
    for k in keys:
        if k in low:
            return low[k]
    for c in cols:
        if any(k in c.lower() for k in keys):
            return c
    return None


def load_samples(csv_path, root):
    rows = list(csv.DictReader(open(csv_path, newline="", encoding="utf-8")))
    if not rows:
        sys.exit(f"{csv_path} is empty")
    cols = rows[0].keys()
    pc, lc = pick(cols, PATH_KEYS), pick(cols, LABEL_KEYS)
    gc, cc = pick(cols, GROUP_KEYS), pick(cols, CRATE_KEYS)
    print(f"columns -> path={pc}  label={lc}  group={gc}  crate={cc}")
    if not pc or not lc:
        sys.exit(f"No path/label column in {list(cols)}. Edit PATH_KEYS/LABEL_KEYS.")
    samples, missing = [], 0
    for r in rows:
        p = r[pc]
        if not os.path.isabs(p):
            p = os.path.join(root, p)
        if not os.path.exists(p):
            missing += 1
            continue
        raw = open(p, encoding="utf-8", errors="replace").read()
        lab = str(r[lc]).strip().lower()
        y = 1 if lab in ("1", "true", "vuln", "vulnerable", "yes") else 0
        grp = (r.get(gc) or "").strip() if gc else ""
        if not grp:
            m = re.search(r"RUSTSEC-\d{4}-\d{4}", raw)
            grp = m.group(0) if m else os.path.basename(p)
        samples.append({"path": p, "y": y, "group": grp,
                        "crate": (r.get(cc) or "").strip() if cc else "",
                        "raw": raw})
    if missing:
        print(f"WARNING: {missing} rows skipped, file not found")
    return samples


def metrics(y_true, y_pred):
    from sklearn.metrics import (accuracy_score, f1_score, matthews_corrcoef,
                                 precision_score, recall_score)
    return {
        "f1": f1_score(y_true, y_pred, zero_division=0),
        "accuracy": accuracy_score(y_true, y_pred),
        "mcc": matthews_corrcoef(y_true, y_pred) if len(set(y_pred)) > 1 else 0.0,
        "precision": precision_score(y_true, y_pred, zero_division=0),
        "recall": recall_score(y_true, y_pred, zero_division=0),
        "pred_pos_rate": float(np.mean(y_pred)),
        "collapsed": len(set(y_pred)) == 1,
    }


def majority_baseline(y):
    from sklearn.metrics import f1_score
    y = np.asarray(y)
    return {"all_positive_f1": f1_score(y, np.ones_like(y), zero_division=0),
            "all_positive_acc": float(np.mean(y == 1)),
            "positive_rate": float(np.mean(y))}


def run_tfidf(texts, y, groups, seed, folds):
    from sklearn.feature_extraction.text import TfidfVectorizer
    from sklearn.linear_model import LogisticRegression
    from sklearn.model_selection import GroupKFold
    from sklearn.pipeline import make_pipeline
    oof = np.zeros(len(y), dtype=int)
    per_fold = []
    for tr, te in GroupKFold(n_splits=folds).split(texts, y, groups):
        clf = make_pipeline(
            TfidfVectorizer(ngram_range=(1, 2), max_features=5000),
            LogisticRegression(C=1.0, max_iter=2000, random_state=seed))
        clf.fit([texts[i] for i in tr], [y[i] for i in tr])
        p = clf.predict([texts[i] for i in te])
        oof[te] = p
        per_fold.append(metrics([y[i] for i in te], p))
    return oof, per_fold


def run_transformer(texts, y, groups, seed, folds, model_name, epochs, bs, maxlen):
    import torch
    from sklearn.model_selection import GroupKFold
    from torch.utils.data import DataLoader, Dataset
    from transformers import (AutoModelForSequenceClassification, AutoTokenizer,
                              set_seed)
    set_seed(seed)
    dev = ("mps" if torch.backends.mps.is_available()
           else "cuda" if torch.cuda.is_available() else "cpu")
    print(f"    device={dev}")
    tok = AutoTokenizer.from_pretrained(model_name)

    class DS(Dataset):
        def __init__(self, t, l):
            self.enc = tok(t, truncation=True, padding="max_length",
                           max_length=maxlen, return_tensors="pt")
            self.l = torch.tensor(l)

        def __len__(self):
            return len(self.l)

        def __getitem__(self, i):
            d = {k: v[i] for k, v in self.enc.items()}
            d["labels"] = self.l[i]
            return d

    oof = np.zeros(len(y), dtype=int)
    per_fold = []
    for fi, (tr, te) in enumerate(GroupKFold(n_splits=folds).split(texts, y, groups), 1):
        model = AutoModelForSequenceClassification.from_pretrained(
            model_name, num_labels=2).to(dev)
        opt = torch.optim.AdamW(model.parameters(), lr=2e-5)
        dl = DataLoader(DS([texts[i] for i in tr], [y[i] for i in tr]),
                        batch_size=bs, shuffle=True)
        model.train()
        for ep in range(epochs):
            tot = 0.0
            for b in dl:
                b = {k: v.to(dev) for k, v in b.items()}
                out = model(**b)
                out.loss.backward()
                opt.step()
                opt.zero_grad()
                tot += out.loss.item()
            print(f"    fold {fi} epoch {ep+1} loss {tot/max(len(dl),1):.4f}", flush=True)
        model.eval()
        preds = []
        with torch.no_grad():
            for b in DataLoader(DS([texts[i] for i in te], [y[i] for i in te]),
                                batch_size=bs):
                b.pop("labels")
                b = {k: v.to(dev) for k, v in b.items()}
                preds += model(**b).logits.argmax(-1).cpu().tolist()
        oof[te] = preds
        per_fold.append(metrics([y[i] for i in te], preds))
        del model
    return oof, per_fold


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dataset", default="dataset.csv")
    ap.add_argument("--root", default=".")
    ap.add_argument("--out", default="ladder_results")
    ap.add_argument("--conditions", default="raw,deheadered,stripped,header_only")
    ap.add_argument("--models", default="tfidf")
    ap.add_argument("--seeds", default="42")
    ap.add_argument("--folds", type=int, default=5)
    ap.add_argument("--codebert-model", default="microsoft/codebert-base")
    ap.add_argument("--epochs", type=int, default=3)
    ap.add_argument("--batch-size", type=int, default=8)
    ap.add_argument("--max-length", type=int, default=256)
    args = ap.parse_args()

    os.makedirs(args.out, exist_ok=True)
    samples = load_samples(args.dataset, args.root)
    y = [s["y"] for s in samples]
    groups = [s["group"] for s in samples]
    print(f"loaded {len(samples)} files | {sum(y)} vulnerable | {len(set(groups))} groups")

    base = majority_baseline(y)
    print(f"majority baseline: all-positive F1={base['all_positive_f1']:.4f} "
          f"acc={base['all_positive_acc']:.4f}")

    n_hdr = sum(1 for s in samples if split_header(s["raw"])[0].strip())
    print(f"files with a detected leading comment header: {n_hdr}/{len(samples)}")
    if n_hdr < len(samples) * 0.9:
        print("  !! header detection <90% -- inspect split_header() before trusting header_only.")

    results = {"majority_baseline": base, "n": len(samples),
               "n_positive": int(sum(y)), "runs": {}}

    for cond in [c.strip() for c in args.conditions.split(",")]:
        texts = [make_condition(s["raw"], cond) for s in samples]
        empt = sum(1 for t in texts if not t.strip())
        chars = int(np.mean([len(t) for t in texts]))
        print(f"\n=== condition {cond}: mean {chars} chars, {empt} empty ===")
        if empt > len(texts) * 0.05:
            print("  !! many empty inputs -- check split_header()")
        for model in [m.strip() for m in args.models.split(",")]:
            for seed in [int(s) for s in args.seeds.split(",")]:
                key = f"{model}|{cond}|seed{seed}"
                print(f"  running {key}", flush=True)
                if model == "tfidf":
                    oof, per_fold = run_tfidf(texts, y, groups, seed, args.folds)
                elif model in ("codebert", "graphcodebert"):
                    name = (args.codebert_model if model == "codebert"
                            else "microsoft/graphcodebert-base")
                    oof, per_fold = run_transformer(texts, y, groups, seed, args.folds,
                                                    name, args.epochs, args.batch_size,
                                                    args.max_length)
                else:
                    print(f"  unknown model {model}, skipping")
                    continue
                pf = os.path.join(args.out, f"predictions_{model}_{cond}_seed{seed}.csv")
                with open(pf, "w", newline="", encoding="utf-8") as fh:
                    w = csv.writer(fh)
                    w.writerow(["path", "group", "crate", "y_true", "y_pred"])
                    for s, p in zip(samples, oof):
                        w.writerow([s["path"], s["group"], s["crate"], s["y"], int(p)])
                agg = {k: [f[k] for f in per_fold] for k in per_fold[0] if k != "collapsed"}
                summary = {k: {"mean": float(np.mean(v)), "std": float(np.std(v))}
                           for k, v in agg.items()}
                summary["collapsed_folds"] = sum(1 for f in per_fold if f["collapsed"])
                summary["oof"] = metrics(y, oof)
                results["runs"][key] = {"per_fold": per_fold, "summary": summary}
                print(f"    F1 {summary['f1']['mean']:.4f} +/- {summary['f1']['std']:.4f}"
                      f" | acc {summary['accuracy']['mean']:.4f}"
                      f" | MCC {summary['mcc']['mean']:.4f}"
                      f" | collapsed folds {summary['collapsed_folds']}/{args.folds}")
                json.dump(results, open(os.path.join(args.out, "results.json"), "w"),
                          indent=2)

    lines = ["Stripping ladder (E1 + E2)", "=" * 72,
             f"n={results['n']}  positives={results['n_positive']}  folds={args.folds}",
             f"all-positive baseline: F1={base['all_positive_f1']:.4f} "
             f"acc={base['all_positive_acc']:.4f}", "",
             f"{'model':<14}{'condition':<14}{'seed':<6}{'F1':<18}{'acc':<10}{'MCC':<10}{'collapsed'}"]
    for key, v in results["runs"].items():
        m, c, s = key.split("|")
        z = v["summary"]
        lines.append(f"{m:<14}{c:<14}{s.replace('seed',''):<6}"
                     f"{z['f1']['mean']:.4f}+/-{z['f1']['std']:.4f}   "
                     f"{z['accuracy']['mean']:.4f}    {z['mcc']['mean']:+.4f}    "
                     f"{z['collapsed_folds']}/{args.folds}")
    lines += ["", "Reading it:",
              "  header_only high      -> the header alone carries the label (E1).",
              "  deheadered ~ raw      -> non-header comments also leak; revise the claim.",
              "  deheadered ~ stripped -> the header is the whole channel.",
              "  collapsed folds       -> single-class output; read accuracy/MCC, not F1."]
    txt = "\n".join(lines)
    open(os.path.join(args.out, "summary.txt"), "w", encoding="utf-8").write(txt + "\n")
    print("\n" + txt)


if __name__ == "__main__":
    main()
