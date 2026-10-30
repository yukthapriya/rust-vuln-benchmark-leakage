# Benchmark Leakage in AI-Based Rust Vulnerability Detection

Artifact for the IEEE AIxSE 2026 paper "Benchmark Leakage in AI-Based Rust Vulnerability
Detection: An Empirical Audit of Code Embeddings, Static Analysis, and Dataset Quality."

## Layout
- `scripts/` – all code used in the paper
- `results/` – result files and per-fold predictions behind every table
- `data/` – the audited RLVD source files (1,074 files, 537 pairs) and metadata
- `requirements.txt` – exact package versions used

## Setup
    python3 -m venv venv && source venv/bin/activate
    pip install -r requirements.txt

**Use the pinned versions.** Every advisory group has exactly two files, so
`GroupKFold`'s fold assignment depends on numpy/scikit-learn tie-breaking. Other
versions produce different folds and different per-fold numbers.

## Paper → files
| Paper | Script(s) | Output |
|---|---|---|
| Fig. 3 (header templates; dataset generator) | `generate_rlvd_dataset.py` | `data/vulnerable_rust_files/`, `data/non_vulnerable_rust_files/` |
| Table I (composition), Table III (quality issues) | `data_quality_assessment.py`, `rust_data_quality_assessment.py`, `check_duplicates.py` | `results/quality_issues_summary.json`, `results/data_quality_assessment.txt` |
| Table IV (similarity tiers) | `audit_sims.py`, `recompute_sims.py`, `build_tier_datasets.py`, `build_tier_pkls.py` | `results/all_pairs_sims_v2.json`, `results/pair_similarity_results.txt` |
| Model comparison table | `build_ml_dataset_pub.py`, `train_pub.py`, `build_ml_dataset_gcb.py`, `train_pub_gcb.py`, `static_only_train.py`, hybrid scripts | `results/results_pub*.json`, `results/static_only_results.txt`, `results/hybrid_results.txt` |
| Table V (input ablation) | `run_stripping_ladder.py`, `run_seeds.sh` | `results/ladder_results/predictions_*.csv` |
| Crate-disjoint split | `crate_split.py`, `build_crate_split_pkl.py` | `results/crate_split_results.txt`, `results/codebert_crate_split_result.txt` |
| Control benchmark, header injection, transfer (Tables II, VI–VIII) | `build_rustxec_pairs.py`, `run_rustxec_experiments.py`, `rustxec_inspect.py` | `results/rustxec_pairs/`, `results/rustxec_results/` |
| Recompute tables from saved predictions | `recompute_ladder.py`, `verify_numbers.py` | – |

## Reproducing Table V without retraining
    cd results && python ../scripts/recompute_ladder.py

## Control benchmark
Derived from RustXec (Ji et al., MSR 2026). Function bodies remain under the
licenses of their upstream crates.
