import os
import csv
from collections import Counter, defaultdict

# Directory paths (relative or absolute; adjust as needed)
VULN_DIR = "vulnerable_rust_files"
SAFE_DIR = "non_vulnerable_rust_files"
CSV_PATH = "dataset.csv"

def read_csv_dataset(csv_path):
    rows = []
    with open(csv_path, encoding="utf-8") as f:
        reader = csv.DictReader(f)
        for row in reader:
            rows.append(row)
    return rows

def check_file_existence_and_size(rows):
    missing_files = []
    empty_files = []
    file_sizes = []
    line_counts = []

    for row in rows:
        file_path = row["file_path"]
        label = row["label"]
        # Decide directory based on label
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
        # count lines
        with open(path, encoding="utf-8", errors="ignore") as fd:
            lines = fd.readlines()
            line_counts.append(len(lines))

    return missing_files, empty_files, file_sizes, line_counts

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

def main():
    print("Loading dataset.csv ...")
    rows = read_csv_dataset(CSV_PATH)

    print(f"\nTotal entries in CSV: {len(rows)}")
    missing_files, empty_files, file_sizes, line_counts = check_file_existence_and_size(rows)

    # 1. Missing and empty files
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

    # 2. Summarize fields
    print("\n==== LABEL AND FIELD DISTRIBUTIONS ====")
    for field in ["label", "vulnerability_type", "severity", "extraction_method", "crate"]:
        histogram = summarize_field_distribution(rows, field)
        print_histogram(field, histogram)

    # 3. Unknown or empty important fields
    print("\n==== INCOMPLETE/UNKNOWN FIELD REPORT ====")
    important_fields = ["cve_id", "vulnerability_type", "severity", "commit_hash", "github_repo", "extraction_method"]
    issues = report_unknown_or_empty(rows, important_fields)
    for field, entries in issues.items():
        print(f"Field '{field}' has {len(entries)} unknown/empty values")

    print("\n==== DATA QUALITY CHECK COMPLETE ====")

if __name__ == "__main__":
    main()