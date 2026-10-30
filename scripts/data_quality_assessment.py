import os
import csv
import re
import hashlib
from collections import Counter, defaultdict

# Directory paths (relative or absolute; adjust as needed)
VULN_DIR = "vulnerable_rust_files"
SAFE_DIR = "non_vulnerable_rust_files"
CSV_PATH = "dataset.csv"


# ============================================================
# CSV LOADING
# ============================================================

def read_csv_dataset(csv_path):
    rows = []
    with open(csv_path, encoding="utf-8") as f:
        reader = csv.DictReader(f)
        for row in reader:
            rows.append(row)
    return rows


# ============================================================
# FILE EXISTENCE & SIZE CHECKS
# ============================================================

def check_file_existence_and_size(rows):
    missing_files = []
    empty_files = []
    file_sizes = []
    line_counts = []

    for row in rows:
        file_path = row["file_path"]

        # Determine correct path
        if file_path.startswith("vulnerable_rust_files/"):
            path = file_path
        elif file_path.startswith("non_vulnerable_rust_files/"):
            path = file_path
        else:
            path = file_path  # fallback

        if not os.path.isfile(path):
            missing_files.append(path)
            continue

        size = os.path.getsize(path)
        file_sizes.append(size)

        if size == 0:
            empty_files.append(path)
            continue

        # Count lines
        with open(path, encoding="utf-8", errors="ignore") as fd:
            lines = fd.readlines()
            line_counts.append(len(lines))

    return missing_files, empty_files, file_sizes, line_counts


# ============================================================
# FIELD DISTRIBUTIONS
# ============================================================

def summarize_field_distribution(rows, field):
    count = Counter(r[field] if r[field].strip() else "<EMPTY>" for r in rows)
    return count


def report_unknown_or_empty(rows, important_fields):
    issues = defaultdict(list)
    for i, row in enumerate(rows):
        for field in important_fields:
            val = row.get(field, '').strip().lower()
            if val in ("", "unknown", "n/a"):
                issues[field].append((i, row["file_path"]))
    return issues


def print_histogram(name, counts):
    print(f"\n{name} distribution:")
    for key, val in counts.most_common():
        print(f"- {key}: {val}")
    print(f"Total unique: {len(counts)}")


# ============================================================
# SEMANTIC QUALITY CHECKS
# ============================================================

def detect_semantic_issues(rows):
    trivial = []
    oversized = []
    duplicates = defaultdict(list)
    ffi_heavy = []
    unsafe_heavy = []

    FFI_RE = re.compile(r'extern\s+"C"|std::ffi|CString|CStr|\*mut|\*const')
    UNSAFE_RE = re.compile(r"\bunsafe\b")

    hash_map = {}

    for row in rows:
        path = row["file_path"]
        if not os.path.isfile(path):
            continue

        with open(path, "r", encoding="utf-8", errors="ignore") as f:
            code = f.read()

        # trivial / oversized
        n_lines = code.count("\n") + 1
        if n_lines < 20:
            trivial.append(path)
        if n_lines > 2000:
            oversized.append(path)

        # duplicates
        h = hashlib.sha256(code.encode("utf-8")).hexdigest()
        hash_map.setdefault(h, []).append(path)

        # FFI-heavy
        if FFI_RE.search(code):
            ffi_heavy.append(path)

        # unsafe-heavy
        if len(UNSAFE_RE.findall(code)) > 5:
            unsafe_heavy.append(path)

    # collect duplicates
    for h, paths in hash_map.items():
        if len(paths) > 1:
            duplicates[h] = paths

    return trivial, oversized, duplicates, ffi_heavy, unsafe_heavy


# ============================================================
# STATIC ANALYSIS FEATURE EXTRACTION
# ============================================================

def extract_static_features(code):
    return {
        "n_unsafe": len(re.findall(r"\bunsafe\b", code)),
        "n_raw_ptr": len(re.findall(r"\*\s*mut|\*\s*const", code)),
        "n_lifetimes": len(re.findall(r"'\w+", code)),
        "n_loops": len(re.findall(r"\bfor\b|\bwhile\b", code)),
        "n_functions": len(re.findall(r"\bfn\s+\w+", code)),
        "n_branches": len(re.findall(r"\bif\b|\bmatch\b", code)),
        "n_lines": code.count("\n") + 1,
        "n_chars": len(code),
    }


def compute_static_features(rows):
    features = []
    for row in rows:
        path = row["file_path"]
        if not os.path.isfile(path):
            continue

        with open(path, "r", encoding="utf-8", errors="ignore") as f:
            code = f.read()

        feats = extract_static_features(code)
        feats["file_path"] = path
        features.append(feats)

    return features


# ============================================================
# MAIN
# ============================================================

def main():
    print("Loading dataset.csv ...")
    rows = read_csv_dataset(CSV_PATH)

    print(f"\nTotal entries in CSV: {len(rows)}")
    missing_files, empty_files, file_sizes, line_counts = check_file_existence_and_size(rows)

    # --------------------------------------------------------
    # FILE EXISTENCE & SIZE
    # --------------------------------------------------------
    print("\n==== FILE EXISTENCE & SIZE ====")
    print(f"Missing files: {len(missing_files)}")
    if missing_files:
        print("Examples:", missing_files[:5])

    print(f"Empty files: {len(empty_files)}")
    if empty_files:
        print("Examples:", empty_files[:5])

    print(f"Mean file size: {sum(file_sizes)/len(file_sizes):.1f} bytes" if file_sizes else "No files found")
    print(f"Mean line count: {sum(line_counts)/len(line_counts):.1f}" if line_counts else "No files found")
    print(f"Max/Min file size: {max(file_sizes) if file_sizes else 'N/A'}/{min(file_sizes) if file_sizes else 'N/A'} bytes")
    print(f"Max/Min line count: {max(line_counts) if line_counts else 'N/A'}/{min(line_counts) if line_counts else 'N/A'}")

    # --------------------------------------------------------
    # FIELD DISTRIBUTIONS
    # --------------------------------------------------------
    print("\n==== LABEL AND FIELD DISTRIBUTIONS ====")
    for field in ["label", "vulnerability_type", "severity", "extraction_method", "crate"]:
        histogram = summarize_field_distribution(rows, field)
        print_histogram(field, histogram)

    # --------------------------------------------------------
    # UNKNOWN / EMPTY FIELDS
    # --------------------------------------------------------
    print("\n==== INCOMPLETE/UNKNOWN FIELD REPORT ====")
    important_fields = ["cve_id", "vulnerability_type", "severity", "commit_hash", "github_repo", "extraction_method"]
    issues = report_unknown_or_empty(rows, important_fields)
    for field, entries in issues.items():
        print(f"Field '{field}' has {len(entries)} unknown/empty values")

    # --------------------------------------------------------
    # SEMANTIC QUALITY CHECKS
    # --------------------------------------------------------
    print("\n==== SEMANTIC QUALITY CHECKS ====")
    trivial, oversized, duplicates, ffi_heavy, unsafe_heavy = detect_semantic_issues(rows)

    print(f"Trivial files (<20 lines): {len(trivial)}")
    print(f"Oversized files (>2000 lines): {len(oversized)}")
    print(f"Duplicate code groups: {len(duplicates)}")
    print(f"FFI-heavy files: {len(ffi_heavy)}")
    print(f"Unsafe-heavy files: {len(unsafe_heavy)}")

    # --------------------------------------------------------
    # STATIC ANALYSIS FEATURES
    # --------------------------------------------------------
    print("\n==== STATIC ANALYSIS FEATURES ====")
    static_feats = compute_static_features(rows)
    print(f"Computed static features for {len(static_feats)} files")

    print("\n==== DATA QUALITY CHECK COMPLETE ====")


if __name__ == "__main__":
    main()
