#!/usr/bin/env python3
"""E4 clean baseline, E3 header injection, E7 cross-dataset transfer."""
import argparse, csv, json, os, sys
import numpy as np
from run_stripping_ladder import (load_samples, majority_baseline, metrics,
                                  run_tfidf, run_transformer, strip_comments)

csv.field_size_limit(sys.maxsize)

VULN_HEADER = """// REAL VULNERABLE CODE
// RustSec ID: {rustsec_id}
// CVE: {cve_id}
// CWE: {cwe_id}
// Crate: {crate}
// Type: {vuln_type}
// Severity: {severity}
// Source: {source_url}
// Extraction Method: {method}
// GitHub Repo: {repo}
// Commit: {commit}
// File: {filename}
"""

SAFE_HEADER = """// REAL FIXED CODE
// RustSec ID: {rustsec_id} (FIXED)
// Crate: {crate}
// Type: {vuln_type} (FIXED)
// Extraction Method: {method}
// GitHub Repo: {repo}
// Commit: {commit}
// File: {filename}
"""


def load_rustxec(pairs_dir, metadata_csv):
    meta = {}
    if os.path.exists(metadata_csv):
        for r in csv.DictReader(open(metadata_csv, newline="", encoding="utf-8")):
            meta[r["id"].strip()] = r
    labels_path = os.path.join(pairs_dir, "labels.csv")
    if not os.path.exists(labels_path):
        sys.exit(f"{labels_path} not found -- run build_rustxec_pairs.py first")
    pair_meta = {}
    pairs_csv = os.path.join(pairs_dir, "pairs.csv")
    if os.path.exists(pairs_csv):
        for r in csv.DictReader(open(pairs_csv, newline="", encoding="utf-8")):
            pair_meta[r["pair_id"]] = r
    out = []
    for r in csv.DictReader(open(labels_path, newline="", encoding="utf-8")):
        path = r["file"]
        if not os.path.exists(path):
            path = os.path.join(pairs_dir, "samples", r["sample_id"])
        if not os.path.exists(path):
            continue
        adv = r["rustsec_id"]
        pm = pair_meta.get(r["pair_id"], {})
        m = meta.get(adv, {})
        out.append({
            "path": path,
            "raw": open(path, encoding="utf-8", errors="replace").read(),
            "y": int(r["label"]),
            "group": adv,
            "crate": r["crate"],
            "overlap": int(r.get("v4_crate_overlap", 0)),
            "meta": {
                "rustsec_id": adv,
                "crate": r["crate"],
                "cve_id": (m.get("aliases") or "unknown")[:60],
                "cwe_id": (m.get("CWE") or "unknown")[:40],
                "vuln_type": (m.get("categories") or "unknown")[:40],
                "severity": (m.get("severity") or "medium"),
                "source_url": (m.get("url") or "unknown")[:90],
                "method": "commit_diff",
                "repo": pm.get("repo", "unknown"),
                "commit": pm.get("fix_sha", "unknown")[:40],
                "filename": os.path.basename(pm.get("path", r["sample_id"])),
            },
        })
    return out


def prep(samples, condition, inject):
    texts = []
    for s in samples:
        body = strip_comments(s["raw"]) if condition == "stripped" else s["raw"]
        if inject:
            tpl = VULN_HEADER if s["y"] == 1 else SAFE_HEADER
            body = tpl.format(**s["meta"]) + body
        texts.append(body)
    return texts


def evaluate(texts, y, groups, models, seeds, folds, tag, results, out, args):
    for model in models:
        for seed in seeds:
            key = f"{tag}|{model}|seed{seed}"
            print(f"  running {key}", flush=True)
            if model == "tfidf":
                oof, per_fold = run_tfidf(texts, y, groups, seed, folds)
            else:
                name = ("microsoft/codebert-base" if model == "codebert"
                        else "microsoft/graphcodebert-base")
                oof, per_fold = run_transformer(texts, y, groups, seed, folds,
                                                name, args.epochs,
                                                args.batch_size, args.max_length)
            with open(os.path.join(out, f"pred_{tag}_{model}_seed{seed}.csv"),
                      "w", newline="", encoding="utf-8") as fh:
                w = csv.writer(fh)
                w.writerow(["group", "y_true", "y_pred"])
                for g, t, p in zip(groups, y, oof):
                    w.writerow([g, t, int(p)])
            agg = {k: [f[k] for f in per_fold] for k in per_fold[0] if k != "collapsed"}
            summ = {k: {"mean": float(np.mean(v)), "std": float(np.std(v))}
                    for k, v in agg.items()}
            summ["collapsed_folds"] = sum(1 for f in per_fold if f["collapsed"])
            results[key] = summ
            print(f"    F1 {summ['f1']['mean']:.4f}+/-{summ['f1']['std']:.4f} | "
                  f"acc {summ['accuracy']['mean']:.4f} | "
                  f"MCC {summ['mcc']['mean']:+.4f} | "
                  f"collapsed {summ['collapsed_folds']}/{folds}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--pairs-dir", default="rustxec_pairs")
    ap.add_argument("--rustxec-metadata", default="RustXec/metadata.csv")
    ap.add_argument("--v4-dataset", default="dataset.csv")
    ap.add_argument("--root", default=".")
    ap.add_argument("--out", default="rustxec_results")
    ap.add_argument("--experiments", default="e4,e3")
    ap.add_argument("--models", default="tfidf")
    ap.add_argument("--seeds", default="42")
    ap.add_argument("--folds", type=int, default=5)
    ap.add_argument("--epochs", type=int, default=3)
    ap.add_argument("--batch-size", type=int, default=8)
    ap.add_argument("--max-length", type=int, default=256)
    args = ap.parse_args()

    os.makedirs(args.out, exist_ok=True)
    models = [m.strip() for m in args.models.split(",")]
    seeds = [int(s) for s in args.seeds.split(",")]
    exps = {e.strip() for e in args.experiments.split(",")}

    samples = load_rustxec(args.pairs_dir, args.rustxec_metadata)
    y = [s["y"] for s in samples]
    groups = [s["group"] for s in samples]
    crates = [s["crate"] for s in samples]
    print(f"RustXec: {len(samples)} files | {sum(y)} vulnerable | "
          f"{len(set(groups))} advisories | {len(set(crates))} crates")
    base = majority_baseline(y)
    print(f"baseline: all-positive F1={base['all_positive_f1']:.4f} "
          f"acc={base['all_positive_acc']:.4f}")
    results = {"baseline": base, "n": len(samples)}

    if "e4" in exps:
        print("\n=== E4  clean baseline (header-free, stripped) ===")
        texts = prep(samples, "stripped", inject=False)
        evaluate(texts, y, groups, models, seeds, args.folds,
                 "e4_advisory_split", results, args.out, args)
        if len(set(crates)) >= args.folds:
            print("  -- crate-disjoint split --")
            evaluate(texts, y, crates, models, seeds, args.folds,
                     "e4_crate_split", results, args.out, args)

    if "e3" in exps:
        print("\n=== E3  header injection (v4-style headers on clean pairs) ===")
        texts = prep(samples, "stripped", inject=True)
        print("  sample injected header:")
        print("   " + "\n   ".join(texts[0].splitlines()[:4]))
        evaluate(texts, y, groups, models, seeds, args.folds,
                 "e3_injected", results, args.out, args)

    if "e7" in exps:
        print("\n=== E7  transfer: train v4 -> test RustXec ===")
        v4 = load_samples(args.v4_dataset, args.root)
        keep = [s for s in samples if not s["overlap"]]
        dropped = len(samples) - len(keep)
        print(f"  dropped {dropped} RustXec files on shared crates; {len(keep)} remain")
        if keep:
            from sklearn.feature_extraction.text import TfidfVectorizer
            from sklearn.linear_model import LogisticRegression
            from sklearn.pipeline import make_pipeline
            yte = [s["y"] for s in keep]
            for cond in ("raw", "stripped"):
                Xtr = [strip_comments(s["raw"]) if cond == "stripped" else s["raw"] for s in v4]
                ytr = [s["y"] for s in v4]
                Xte = [strip_comments(s["raw"]) if cond == "stripped" else s["raw"] for s in keep]
                clf = make_pipeline(
                    TfidfVectorizer(ngram_range=(1, 2), max_features=5000),
                    LogisticRegression(C=1.0, max_iter=2000, random_state=42))
                clf.fit(Xtr, ytr)
                m = metrics(yte, clf.predict(Xte))
                results[f"e7_tfidf_train{cond}"] = m
                print(f"  train-v4({cond}) -> RustXec: F1 {m['f1']:.4f} | "
                      f"acc {m['accuracy']:.4f} | MCC {m['mcc']:+.4f} | "
                      f"collapsed {m['collapsed']}")
            results["e7_dropped_overlap_files"] = dropped

    json.dump(results, open(os.path.join(args.out, "results.json"), "w"), indent=2)
    lines = ["RustXec experiments (E4 / E3 / E7)", "=" * 60,
             f"files={results['n']}  all-positive baseline "
             f"F1={base['all_positive_f1']:.4f} acc={base['all_positive_acc']:.4f}", ""]
    for k, v in results.items():
        if isinstance(v, dict) and "f1" in v:
            f1 = v["f1"]
            if isinstance(f1, dict):
                lines.append(f"{k:<42} F1 {f1['mean']:.4f}+/-{f1['std']:.4f}  "
                             f"acc {v['accuracy']['mean']:.4f}  MCC {v['mcc']['mean']:+.4f}")
            else:
                lines.append(f"{k:<42} F1 {f1:.4f}  acc {v['accuracy']:.4f}  "
                             f"MCC {v['mcc']:+.4f}")
    lines += ["", "Reading it:",
              "  E4 well below 1.0 -> leakage-free performance; the contrast number.",
              "  E3 near 1.0       -> headers alone restore perfect scores: causal proof.",
              "  E7 at/below chance-> nothing learned on v4 transfers."]
    txt = "\n".join(lines)
    open(os.path.join(args.out, "summary.txt"), "w", encoding="utf-8").write(txt + "\n")
    print("\n" + txt)


if __name__ == "__main__":
    main()
