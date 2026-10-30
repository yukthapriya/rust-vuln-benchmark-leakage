import os
import csv
import json
import requests
import time
import zipfile
import io
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Tuple
import re
import base64

class RealWorldRustVulnerabilityDatasetGenerator:
    """
    Generates a ground-truth Rust vulnerability dataset using ACTUAL code from:
    1. GitHub commits that fixed vulnerabilities
    2. RustSec advisory references
    3. Crate repository historical versions
    """
    
    def __init__(self, base_path: str, github_token: Optional[str] = None):
        self.base_path = Path(base_path)
        self.vulnerable_dir = self.base_path / "vulnerable_rust_files"
        self.non_vulnerable_dir = self.base_path / "non_vulnerable_rust_files"
        
        self.vulnerable_dir.mkdir(parents=True, exist_ok=True)
        self.non_vulnerable_dir.mkdir(parents=True, exist_ok=True)
        
        self.dataset_csv = self.base_path / "ground_truth_dataset.csv"
        self.log_file = self.base_path / "generation_log.txt"
        self.advisories_cache = self.base_path / "rustsec_advisories_cache.json"
        self.skipped_log = self.base_path / "skipped_advisories.txt"
        self.success_log = self.base_path / "successful_extractions.txt"
        
        self.dataset_entries = []
        self.vuln_file_counter = 0
        self.safe_file_counter = 0
        self.skipped_count = 0
        self.skipped_reasons = {}
        self.github_token = github_token
        
        # CSV headers
        self.csv_headers = [
            'file_path',
            'label',
            'rustsec_id',
            'cve_id',
            'cwe_id',
            'vulnerability_type',
            'severity',
            'crate',
            'source_url',
            'commit_hash',
            'github_repo',
            'extraction_method'
        ]
        
        self.log(f"Initialized real-world vulnerability dataset generator at {self.base_path}")
        if github_token:
            self.log("GitHub token provided - API rate limits will be higher")
        else:
            self.log("WARNING: No GitHub token - limited to 60 requests/hour")
    
    def log(self, message: str):
        """Log messages to file and console"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        log_message = f"[{timestamp}] {message}"
        print(log_message)
        with open(self.log_file, 'a', encoding='utf-8') as f:
            f.write(log_message + '\n')
    
    def log_success(self, rustsec_id: str, method: str, details: str):
        """Log successful code extractions"""
        with open(self.success_log, 'a', encoding='utf-8') as f:
            f.write(f"{rustsec_id} | {method} | {details}\n")
    
    def log_skipped(self, rustsec_id: str, reason: str):
        """Log skipped advisories"""
        self.skipped_count += 1
        self.skipped_reasons[reason] = self.skipped_reasons.get(reason, 0) + 1
        with open(self.skipped_log, 'a', encoding='utf-8') as f:
            f.write(f"{rustsec_id}: {reason}\n")
    
    def fetch_all_rustsec_advisories(self) -> List[Dict]:
        """Fetch all RustSec advisories from OSV API"""
        self.log("Fetching RustSec advisories from OSV API...")
        
        if self.advisories_cache.exists():
            cache_age = time.time() - self.advisories_cache.stat().st_mtime
            if cache_age < 86400:
                self.log("Loading advisories from cache...")
                with open(self.advisories_cache, 'r', encoding='utf-8') as f:
                    return json.load(f)
        
        all_advisories = []
        
        try:
            self.log("Downloading OSV database for crates.io...")
            response = requests.get(
                "https://osv-vulnerabilities.storage.googleapis.com/crates.io/all.zip",
                timeout=120
            )
            
            if response.status_code == 200:
                self.log("Extracting advisories from ZIP...")
                zip_file = zipfile.ZipFile(io.BytesIO(response.content))
                
                for filename in zip_file.namelist():
                    if filename.endswith('.json'):
                        with zip_file.open(filename) as f:
                            try:
                                advisory = json.loads(f.read())
                                if advisory.get('id', '').startswith('RUSTSEC-'):
                                    all_advisories.append(advisory)
                            except json.JSONDecodeError:
                                continue
                
                self.log(f"Successfully fetched {len(all_advisories)} RUSTSEC advisories")
                
                with open(self.advisories_cache, 'w', encoding='utf-8') as f:
                    json.dump(all_advisories, f, indent=2)
                
                return all_advisories
            else:
                self.log(f"Failed to download OSV database: HTTP {response.status_code}")
                
        except Exception as e:
            self.log(f"Error fetching advisories: {str(e)}")
        
        return []
    
    def extract_github_info(self, url: str) -> Optional[Tuple[str, str, Optional[str]]]:
        """Extract owner, repo, and commit from GitHub URL"""
        patterns = [
            r'github\.com/([^/]+)/([^/]+)/commit/([a-f0-9]+)',
            r'github\.com/([^/]+)/([^/]+)/pull/(\d+)',
            r'github\.com/([^/]+)/([^/]+)',
        ]
        
        for pattern in patterns:
            match = re.search(pattern, url)
            if match:
                owner = match.group(1)
                repo = match.group(2).replace('.git', '')
                commit = match.group(3) if len(match.groups()) > 2 else None
                return owner, repo, commit
        
        return None
    
    def github_api_request(self, url: str) -> Optional[Dict]:
        """Make GitHub API request with authentication"""
        headers = {'Accept': 'application/vnd.github.v3+json'}
        if self.github_token:
            headers['Authorization'] = f'token {self.github_token}'
        
        try:
            response = requests.get(url, headers=headers, timeout=30)
            if response.status_code == 200:
                return response.json()
            elif response.status_code == 403:
                self.log(f"GitHub API rate limit exceeded")
                return None
            else:
                self.log(f"GitHub API error: {response.status_code}")
                return None
        except Exception as e:
            self.log(f"GitHub API request failed: {str(e)}")
            return None
    
    def fetch_commit_diff(self, owner: str, repo: str, commit_hash: str) -> Optional[List[Dict]]:
        """Fetch commit diff from GitHub to get vulnerable and fixed code"""
        url = f"https://api.github.com/repos/{owner}/{repo}/commits/{commit_hash}"
        commit_data = self.github_api_request(url)
        
        if not commit_data:
            return None
        
        files = commit_data.get('files', [])
        rust_files = []
        
        for file in files:
            filename = file.get('filename', '')
            if filename.endswith('.rs'):
                file_info = {
                    'filename': filename,
                    'status': file.get('status'),
                    'patch': file.get('patch', ''),
                    'additions': file.get('additions', 0),
                    'deletions': file.get('deletions', 0),
                    'raw_url': file.get('raw_url', '')
                }
                rust_files.append(file_info)
        
        return rust_files if rust_files else None
    
    def extract_code_from_patch(self, patch: str) -> Tuple[Optional[str], Optional[str]]:
        """Extract vulnerable (removed) and fixed (added) code from patch"""
        vulnerable_lines = []
        fixed_lines = []
        
        for line in patch.split('\n'):
            if line.startswith('-') and not line.startswith('---'):
                vulnerable_lines.append(line[1:])
            elif line.startswith('+') and not line.startswith('+++'):
                fixed_lines.append(line[1:])
            elif not line.startswith('@@') and not line.startswith('\\'):
                # Context lines - add to both
                vulnerable_lines.append(line)
                fixed_lines.append(line)
        
        vulnerable_code = '\n'.join(vulnerable_lines).strip()
        fixed_code = '\n'.join(fixed_lines).strip()
        
        # Only return if we have substantial code
        if len(vulnerable_code) > 50 and len(fixed_code) > 50:
            return vulnerable_code, fixed_code
        
        return None, None
    
    def fetch_file_at_commit(self, owner: str, repo: str, commit_hash: str, filepath: str) -> Optional[str]:
        """Fetch full file content at specific commit"""
        url = f"https://api.github.com/repos/{owner}/{repo}/contents/{filepath}?ref={commit_hash}"
        data = self.github_api_request(url)
        
        if data and 'content' in data:
            try:
                content = base64.b64decode(data['content']).decode('utf-8')
                return content
            except Exception as e:
                self.log(f"Error decoding file content: {str(e)}")
        
        return None
    
    def fetch_crate_source(self, crate_name: str, version: str) -> Optional[str]:
        """Fetch source code from crates.io"""
        try:
            url = f"https://crates.io/api/v1/crates/{crate_name}/{version}/download"
            response = requests.get(url, timeout=30)
            
            if response.status_code == 200:
                # Extract .crate file (it's a tar.gz)
                import tarfile
                tar = tarfile.open(fileobj=io.BytesIO(response.content), mode='r:gz')
                
                # Find main lib.rs or main.rs
                rust_files = []
                for member in tar.getmembers():
                    if member.name.endswith('.rs'):
                        rust_files.append(member)
                
                # Prioritize lib.rs or src/lib.rs
                for member in rust_files:
                    if 'lib.rs' in member.name or 'main.rs' in member.name:
                        f = tar.extractfile(member)
                        if f:
                            content = f.read().decode('utf-8', errors='ignore')
                            return content
                
                # If no lib.rs, return first .rs file
                if rust_files:
                    f = tar.extractfile(rust_files[0])
                    if f:
                        content = f.read().decode('utf-8', errors='ignore')
                        return content
                        
        except Exception as e:
            self.log(f"Error fetching crate source: {str(e)}")
        
        return None
    
    def extract_real_vulnerable_code(self, advisory: Dict) -> Optional[Dict]:
        """Extract real vulnerable code from various sources"""
        rustsec_id = advisory.get('id', '')
        references = advisory.get('references', [])
        
        # Try to find GitHub commit/PR references
        github_urls = []
        for ref in references:
            url = ref.get('url', '')
            if 'github.com' in url and ('commit' in url or 'pull' in url):
                github_urls.append(url)
        
        # Method 1: Extract from GitHub commit diffs
        for url in github_urls:
            github_info = self.extract_github_info(url)
            if not github_info:
                continue
            
            owner, repo, commit = github_info
            if not commit:
                continue
            
            self.log(f"  Fetching commit {commit[:7]} from {owner}/{repo}")
            
            rust_files = self.fetch_commit_diff(owner, repo, commit)
            if rust_files:
                for file_info in rust_files:
                    vulnerable_code, fixed_code = self.extract_code_from_patch(file_info['patch'])
                    
                    if vulnerable_code and fixed_code:
                        self.log_success(rustsec_id, "commit_diff", f"{owner}/{repo}@{commit[:7]}")
                        return {
                            'vulnerable_code': vulnerable_code,
                            'fixed_code': fixed_code,
                            'method': 'commit_diff',
                            'commit_hash': commit,
                            'github_repo': f"{owner}/{repo}",
                            'filename': file_info['filename']
                        }
        
        # Method 2: Try to fetch from crates.io versions
        affected = advisory.get('affected', [])
        if affected:
            package_info = affected[0].get('package', {})
            crate_name = package_info.get('name', '')
            
            # Get vulnerable and fixed versions
            ranges = affected[0].get('ranges', [])
            vulnerable_version = None
            fixed_version = None
            
            for range_entry in ranges:
                events = range_entry.get('events', [])
                for event in events:
                    if 'introduced' in event and event['introduced'] != '0':
                        vulnerable_version = event['introduced']
                    if 'fixed' in event:
                        fixed_version = event['fixed']
            
            if crate_name and vulnerable_version and fixed_version:
                self.log(f"  Fetching crate {crate_name} v{vulnerable_version} and v{fixed_version}")
                
                vuln_source = self.fetch_crate_source(crate_name, vulnerable_version)
                fixed_source = self.fetch_crate_source(crate_name, fixed_version)
                
                if vuln_source and fixed_source:
                    self.log_success(rustsec_id, "crate_versions", f"{crate_name} {vulnerable_version}->{fixed_version}")
                    return {
                        'vulnerable_code': vuln_source,
                        'fixed_code': fixed_source,
                        'method': 'crate_versions',
                        'commit_hash': '',
                        'github_repo': crate_name,
                        'filename': 'lib.rs'
                    }
        
        # Method 3: Try to find repository and fetch at different commits
        repo_url = None
        for ref in references:
            url = ref.get('url', '')
            if 'github.com' in url and 'commit' not in url and 'pull' not in url:
                repo_url = url
                break
        
        if repo_url:
            github_info = self.extract_github_info(repo_url)
            if github_info:
                owner, repo, _ = github_info
                
                # Try to find relevant commits
                commits_url = f"https://api.github.com/repos/{owner}/{repo}/commits?per_page=50"
                commits_data = self.github_api_request(commits_url)
                
                if commits_data:
                    for commit in commits_data:
                        commit_msg = commit.get('commit', {}).get('message', '').lower()
                        if any(word in commit_msg for word in ['fix', 'security', 'vulnerability', 'cve']):
                            commit_sha = commit.get('sha')
                            rust_files = self.fetch_commit_diff(owner, repo, commit_sha)
                            
                            if rust_files:
                                for file_info in rust_files:
                                    vulnerable_code, fixed_code = self.extract_code_from_patch(file_info['patch'])
                                    
                                    if vulnerable_code and fixed_code:
                                        self.log_success(rustsec_id, "repo_search", f"{owner}/{repo}@{commit_sha[:7]}")
                                        return {
                                            'vulnerable_code': vulnerable_code,
                                            'fixed_code': fixed_code,
                                            'method': 'repo_search',
                                            'commit_hash': commit_sha,
                                            'github_repo': f"{owner}/{repo}",
                                            'filename': file_info['filename']
                                        }
        
        return None
    
    def classify_vulnerability_type(self, advisory: Dict) -> str:
        """Classify vulnerability type"""
        database_specific = advisory.get('database_specific', {})
        cwes = database_specific.get('cwes', [])
        categories = database_specific.get('categories', [])
        
        cwe_mapping = {
            'CWE-119': 'buffer_overflow',
            'CWE-120': 'buffer_overflow',
            'CWE-416': 'use_after_free',
            'CWE-415': 'double_free',
            'CWE-362': 'race_condition',
            'CWE-190': 'integer_overflow',
            'CWE-476': 'null_pointer_dereference',
            'CWE-502': 'deserialization',
            'CWE-400': 'denial_of_service',
            'CWE-327': 'weak_cryptography',
        }
        
        for cwe in cwes:
            if cwe in cwe_mapping:
                return cwe_mapping[cwe]
        
        if categories:
            return categories[0].lower().replace('-', '_')
        
        summary = advisory.get('summary', '').lower()
        if 'memory' in summary:
            return 'memory_safety'
        elif 'overflow' in summary:
            return 'overflow'
        elif 'race' in summary:
            return 'race_condition'
        
        return 'unknown'
    
    def save_rust_file(self, code: str, metadata: str, is_vulnerable: bool) -> Path:
        """Save Rust code with metadata header"""
        if is_vulnerable:
            self.vuln_file_counter += 1
            filename = f"vuln_{self.vuln_file_counter:05d}.rs"
            filepath = self.vulnerable_dir / filename
        else:
            self.safe_file_counter += 1
            filename = f"safe_{self.safe_file_counter:05d}.rs"
            filepath = self.non_vulnerable_dir / filename
        
        full_content = metadata + "\n\n" + code
        
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(full_content)
        
        return filepath
    
    def process_advisory(self, advisory: Dict) -> bool:
        """Process a single advisory and extract real code"""
        rustsec_id = advisory.get('id', '')
        
        # Validate basic requirements
        affected = advisory.get('affected', [])
        if not affected:
            self.log_skipped(rustsec_id, "No affected packages")
            return False
        
        crate_name = affected[0].get('package', {}).get('name', '')
        if not crate_name:
            self.log_skipped(rustsec_id, "No crate name")
            return False
        
        self.log(f"Processing {rustsec_id} ({crate_name})...")
        
        # Extract real vulnerable code
        code_data = self.extract_real_vulnerable_code(advisory)
        
        if not code_data:
            self.log_skipped(rustsec_id, "Could not extract real code")
            return False
        
        # Get metadata
        cve_id = next((a for a in advisory.get('aliases', []) if a.startswith('CVE-')), '')
        database_specific = advisory.get('database_specific', {})
        cwes = database_specific.get('cwes', [])
        cwe_id = cwes[0] if cwes else ''
        
        vuln_type = self.classify_vulnerability_type(advisory)
        
        severity_entries = advisory.get('severity', [])
        severity = 'medium'
        for sev in severity_entries:
            if sev.get('type') == 'CVSS_V3':
                score_str = sev.get('score', '')
                try:
                    score = float(score_str.split()[0])
                    if score >= 9.0:
                        severity = 'critical'
                    elif score >= 7.0:
                        severity = 'high'
                    elif score >= 4.0:
                        severity = 'medium'
                    else:
                        severity = 'low'
                except:
                    pass
        
        references = advisory.get('references', [])
        source_url = references[0].get('url', '') if references else f"https://rustsec.org/advisories/{rustsec_id}"
        
        # Create metadata headers
        vuln_metadata = f'''// REAL VULNERABLE CODE
// RustSec ID: {rustsec_id}
// CVE: {cve_id if cve_id else "N/A"}
// CWE: {cwe_id if cwe_id else "N/A"}
// Crate: {crate_name}
// Type: {vuln_type}
// Severity: {severity}
// Source: {source_url}
// Extraction Method: {code_data['method']}
// GitHub Repo: {code_data['github_repo']}
// Commit: {code_data['commit_hash'][:7] if code_data['commit_hash'] else "N/A"}
// File: {code_data['filename']}
'''
        
        safe_metadata = f'''// REAL FIXED CODE
// RustSec ID: {rustsec_id} (FIXED)
// Crate: {crate_name}
// Type: {vuln_type} (FIXED)
// Extraction Method: {code_data['method']}
// GitHub Repo: {code_data['github_repo']}
// Commit: {code_data['commit_hash'][:7] if code_data['commit_hash'] else "N/A"}
// File: {code_data['filename']}
'''
        
        # Save files
        vuln_filepath = self.save_rust_file(code_data['vulnerable_code'], vuln_metadata, is_vulnerable=True)
        safe_filepath = self.save_rust_file(code_data['fixed_code'], safe_metadata, is_vulnerable=False)
        
        # Create dataset entries
        vuln_entry = {
            'file_path': str(vuln_filepath.relative_to(self.base_path)).replace('\\', '/'),
            'label': 'vulnerable',
            'rustsec_id': rustsec_id,
            'cve_id': cve_id,
            'cwe_id': cwe_id,
            'vulnerability_type': vuln_type,
            'severity': severity,
            'crate': crate_name,
            'source_url': source_url,
            'commit_hash': code_data['commit_hash'][:7] if code_data['commit_hash'] else '',
            'github_repo': code_data['github_repo'],
            'extraction_method': code_data['method']
        }
        
        safe_entry = {
            'file_path': str(safe_filepath.relative_to(self.base_path)).replace('\\', '/'),
            'label': 'safe',
            'rustsec_id': rustsec_id,
            'cve_id': cve_id,
            'cwe_id': cwe_id,
            'vulnerability_type': vuln_type,
            'severity': severity,
            'crate': crate_name,
            'source_url': source_url,
            'commit_hash': code_data['commit_hash'][:7] if code_data['commit_hash'] else '',
            'github_repo': code_data['github_repo'],
            'extraction_method': code_data['method']
        }
        
        self.dataset_entries.append(vuln_entry)
        self.dataset_entries.append(safe_entry)
        
        self.log(f"  SUCCESS: Extracted real code via {code_data['method']}")
        
        # Add delay to respect GitHub rate limits
        time.sleep(1)
        
        return True
    
    def save_dataset_to_csv(self):
        """Save dataset to CSV"""
        self.log(f"Saving dataset to {self.dataset_csv}")
        
        with open(self.dataset_csv, 'w', newline='', encoding='utf-8') as csvfile:
            writer = csv.DictWriter(csvfile, fieldnames=self.csv_headers)
            writer.writeheader()
            writer.writerows(self.dataset_entries)
        
        self.log(f"Dataset saved with {len(self.dataset_entries)} entries")
    
    def generate_summary_report(self):
        """Generate summary report"""
        self.log("Generating summary report...")
        
        report_file = self.base_path / "dataset_summary.txt"
        
        total_samples = len(self.dataset_entries)
        vulnerable_count = sum(1 for e in self.dataset_entries if e['label'] == 'vulnerable')
        safe_count = sum(1 for e in self.dataset_entries if e['label'] == 'safe')
        
        extraction_methods = {}
        vuln_types = {}
        severity_counts = {}
        repos = {}
        
        for entry in self.dataset_entries:
            if entry['label'] == 'vulnerable':
                method = entry['extraction_method']
                extraction_methods[method] = extraction_methods.get(method, 0) + 1
                
                vuln_type = entry['vulnerability_type']
                vuln_types[vuln_type] = vuln_types.get(vuln_type, 0) + 1
                
                severity = entry['severity']
                severity_counts[severity] = severity_counts.get(severity, 0) + 1
                
                repo = entry['github_repo']
                if repo:
                    repos[repo] = repos.get(repo, 0) + 1
        
        with open(report_file, 'w', encoding='utf-8') as f:
            f.write("=" * 80 + "\n")
            f.write("REAL-WORLD RUST VULNERABILITY DATASET SUMMARY\n")
            f.write("=" * 80 + "\n\n")
            
            f.write(f"Generation Date: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n")
            f.write(f"Source: Real code from GitHub commits and crates.io\n\n")
            
            f.write("-" * 80 + "\n")
            f.write("DATASET STATISTICS\n")
            f.write("-" * 80 + "\n")
            f.write(f"Total Samples:           {total_samples}\n")
            f.write(f"Vulnerable Samples:      {vulnerable_count}\n")
            f.write(f"Safe Samples:            {safe_count}\n")
            f.write(f"Unique Repositories:     {len(repos)}\n")
            f.write(f"Skipped Advisories:      {self.skipped_count}\n\n")
            
            if extraction_methods:
                f.write("-" * 80 + "\n")
                f.write("EXTRACTION METHODS\n")
                f.write("-" * 80 + "\n")
                for method, count in sorted(extraction_methods.items(), key=lambda x: x[1], reverse=True):
                    percentage = (count / vulnerable_count) * 100 if vulnerable_count > 0 else 0
                    f.write(f"{method:25s}: {count:4d} ({percentage:5.1f}%)\n")
                f.write("\n")
            
            if vuln_types:
                f.write("-" * 80 + "\n")
                f.write("VULNERABILITY TYPE DISTRIBUTION\n")
                f.write("-" * 80 + "\n")
                for vuln_type, count in sorted(vuln_types.items(), key=lambda x: x[1], reverse=True):
                    percentage = (count / vulnerable_count) * 100 if vulnerable_count > 0 else 0
                    f.write(f"{vuln_type:35s}: {count:4d} ({percentage:5.1f}%)\n")
                f.write("\n")
            
            if severity_counts:
                f.write("-" * 80 + "\n")
                f.write("SEVERITY DISTRIBUTION\n")
                f.write("-" * 80 + "\n")
                for severity in ['critical', 'high', 'medium', 'low']:
                    count = severity_counts.get(severity, 0)
                    percentage = (count / vulnerable_count) * 100 if vulnerable_count > 0 else 0
                    f.write(f"{severity:15s}: {count:4d} ({percentage:5.1f}%)\n")
                f.write("\n")
            
            if repos:
                f.write("-" * 80 + "\n")
                f.write("TOP 20 SOURCE REPOSITORIES\n")
                f.write("-" * 80 + "\n")
                for repo, count in sorted(repos.items(), key=lambda x: x[1], reverse=True)[:20]:
                    f.write(f"{repo:50s}: {count:4d}\n")
                f.write("\n")
            
            if self.skipped_reasons:
                f.write("-" * 80 + "\n")
                f.write("SKIP REASONS\n")
                f.write("-" * 80 + "\n")
                for reason, count in sorted(self.skipped_reasons.items(), key=lambda x: x[1], reverse=True):
                    f.write(f"{reason:45s}: {count:4d}\n")
                f.write("\n")
            
            f.write("=" * 80 + "\n")
            f.write("DATASET CHARACTERISTICS\n")
            f.write("=" * 80 + "\n")
            f.write("- ALL CODE IS REAL: Extracted from actual commits and releases\n")
            f.write("- commit_diff: Before/after code from vulnerability fix commits\n")
            f.write("- crate_versions: Vulnerable vs fixed crate versions\n")
            f.write("- repo_search: Found by searching commit history\n")
            f.write("- Each vulnerable sample has corresponding fixed version\n")
            f.write("- Commit hashes and GitHub repos provided for verification\n")
            f.write("=" * 80 + "\n")
        
        self.log(f"Summary report saved to {report_file}")
    
    def run(self, max_samples: Optional[int] = None):
        """Main execution"""
        self.log("=" * 80)
        self.log("STARTING REAL-WORLD DATASET GENERATION")
        self.log("=" * 80)
        
        try:
            advisories = self.fetch_all_rustsec_advisories()
            self.log(f"Retrieved {len(advisories)} advisories from OSV")
            
            if not advisories:
                self.log("ERROR: No advisories fetched. Cannot proceed.")
                return
            
            # Process advisories
            self.log("Extracting real vulnerable code from advisories...")
            processed = 0
            target = max_samples if max_samples else len(advisories)
            
            for i, advisory in enumerate(advisories, 1):
                if max_samples and processed >= max_samples:
                    self.log(f"Reached target of {max_samples} samples")
                    break
                
                rustsec_id = advisory.get('id', f'UNKNOWN-{i}')
                
                if i % 50 == 0:
                    self.log(f"Progress: {i}/{len(advisories)} advisories checked, {processed} successfully extracted")
                
                try:
                    success = self.process_advisory(advisory)
                    if success:
                        processed += 1
                except Exception as e:
                    self.log(f"Error processing {rustsec_id}: {str(e)}")
                    self.log_skipped(rustsec_id, f"Processing error: {str(e)}")
                    continue
            
            # Save dataset
            self.save_dataset_to_csv()
            
            # Generate summary
            self.generate_summary_report()
            
            self.log("=" * 80)
            self.log("DATASET GENERATION COMPLETE")
            self.log(f"Total samples generated: {len(self.dataset_entries)}")
            self.log(f"  - Vulnerable: {self.vuln_file_counter}")
            self.log(f"  - Safe: {self.safe_file_counter}")
            self.log(f"Successfully processed: {processed}/{len(advisories)} advisories")
            self.log(f"Skipped advisories: {self.skipped_count}")
            self.log(f"\nOutput files:")
            self.log(f"  - Vulnerable code: {self.vulnerable_dir}")
            self.log(f"  - Safe code: {self.non_vulnerable_dir}")
            self.log(f"  - CSV dataset: {self.dataset_csv}")
            self.log(f"  - Summary: {self.base_path / 'dataset_summary.txt'}")
            self.log(f"  - Success log: {self.success_log}")
            self.log(f"  - Skipped log: {self.skipped_log}")
            self.log("=" * 80)
            
        except Exception as e:
            self.log(f"CRITICAL ERROR: {str(e)}")
            import traceback
            self.log(traceback.format_exc())
            raise


def main():
    """Main entry point"""
    import argparse
    
    parser = argparse.ArgumentParser(description='Generate real-world Rust vulnerability dataset')
    parser.add_argument('--path', type=str, required=True, help='Base path for dataset')
    parser.add_argument('--github-token', type=str, help='GitHub personal access token (recommended)')
    parser.add_argument('--max-samples', type=int, help='Maximum number of samples to collect')
    
    args = parser.parse_args()
    
    print("=" * 80)
    print("REAL-WORLD RUST VULNERABILITY DATASET GENERATOR")
    print("=" * 80)
    print(f"\nBase path: {args.path}")
    print("\nThis tool extracts REAL vulnerable code from:")
    print("  1. GitHub commit diffs (before/after vulnerability fixes)")
    print("  2. Crates.io vulnerable and fixed versions")
    print("  3. Repository commit history searches")
    print("\nFeatures:")
    print("  * 100% REAL CODE - No templates or synthetic examples")
    print("  * Commit hashes and GitHub repos for full verification")
    print("  * Both vulnerable and fixed versions extracted")
    print("  * Metadata includes extraction method and source")
    
    if args.github_token:
        print("\n  + GitHub token provided - Higher rate limits (5000 req/hour)")
    else:
        print("\n  ! WARNING: No GitHub token - Limited to 60 requests/hour")
        print("  ! Get token at: https://github.com/settings/tokens")
        print("  ! Use: --github-token YOUR_TOKEN")
    
    if args.max_samples:
        print(f"\n  * Target: {args.max_samples} samples")
    else:
        print("\n  * Target: ALL advisories (may take several hours)")
    
    print("\n" + "=" * 80)
    
    response = input("\nProceed with real code extraction? (yes/no): ")
    if response.lower() not in ['yes', 'y']:
        print("Generation cancelled.")
        return
    
    print("\nInitializing generator...")
    generator = RealWorldRustVulnerabilityDatasetGenerator(args.path, args.github_token)
    
    print("\nStarting extraction process...")
    print("This will take time as we fetch real code from GitHub and crates.io...")
    generator.run(max_samples=args.max_samples)
    
    print("\n" + "=" * 80)
    print("+ Real-world dataset generation completed!")
    print("=" * 80)
    print(f"\nResults saved to: {args.path}")
    print("\nDataset files:")
    print("  * ground_truth_dataset.csv - Main dataset with metadata")
    print("  * vulnerable_rust_files/ - Real vulnerable code samples")
    print("  * non_vulnerable_rust_files/ - Real fixed code samples")
    print("  * dataset_summary.txt - Statistics and breakdown")
    print("  * successful_extractions.txt - Log of successful extractions")
    print("  * skipped_advisories.txt - Advisories without extractable code")
    print("\nVerification:")
    print("  * Each file has commit hash and GitHub repo in header")
    print("  * Use commit hash to verify code on GitHub")
    print("  * extraction_method column shows how code was obtained")
    print("=" * 80)


if __name__ == "__main__":
    import sys
    
    # ========== CONFIGURATION - EDIT THESE VALUES ==========
    # Set your base path here
    BASE_PATH = r"C:\Users\sbosh01\Downloads\Spring 2025\Research\Rust Project\Rustv2"
    
    # Optional: Add your GitHub token for higher rate limits
    # Get token at: https://github.com/settings/tokens
    # Permissions needed: public_repo (read-only)
    GITHUB_TOKEN = os.environ.get('GITHUB_TOKEN', '')
    
    # Optional: Set max samples (None = extract all available)
    MAX_SAMPLES = 1000  # Change to None for all, or set a number like 50, 100, etc.
    # =======================================================
    
    if len(sys.argv) > 1:
        # Use argparse if command-line arguments provided
        main()
    else:
        # Direct execution with configured values
        print("=" * 80)
        print("REAL-WORLD RUST VULNERABILITY DATASET GENERATOR")
        print("=" * 80)
        print(f"\nBase path: {BASE_PATH}")
        print("\nThis tool extracts REAL vulnerable code from:")
        print("  1. GitHub commit diffs (before/after vulnerability fixes)")
        print("  2. Crates.io vulnerable and fixed versions")
        print("  3. Repository commit history searches")
        print("\nFeatures:")
        print("  * 100% REAL CODE - No templates or synthetic examples")
        print("  * Commit hashes and GitHub repos for full verification")
        print("  * Both vulnerable and fixed versions extracted")
        print("  * Metadata includes extraction method and source")
        
        if GITHUB_TOKEN:
            print("\n  + GitHub token configured - Higher rate limits (5000 req/hour)")
        else:
            print("\n  ! WARNING: No GitHub token - Limited to 60 requests/hour")
            print("  ! Get token at: https://github.com/settings/tokens")
            print("  ! Edit GITHUB_TOKEN in script to add your token")
        
        if MAX_SAMPLES:
            print(f"\n  * Target: {MAX_SAMPLES} samples")
        else:
            print("\n  * Target: ALL advisories (may take several hours)")
        
        print("\n" + "=" * 80)
        
        response = input("\nProceed with real code extraction? (yes/no): ")
        if response.lower() not in ['yes', 'y']:
            print("Generation cancelled.")
            sys.exit(0)
        
        print("\nInitializing generator...")
        generator = RealWorldRustVulnerabilityDatasetGenerator(BASE_PATH, GITHUB_TOKEN)
        
        print("\nStarting extraction process...")
        print("This will take time as we fetch real code from GitHub and crates.io...")
        generator.run(max_samples=MAX_SAMPLES)
        
        print("\n" + "=" * 80)
        print("+ Real-world dataset generation completed!")
        print("=" * 80)
        print(f"\nResults saved to: {BASE_PATH}")
        print("\nDataset files:")
        print("  * ground_truth_dataset.csv - Main dataset with metadata")
        print("  * vulnerable_rust_files/ - Real vulnerable code samples")
        print("  * non_vulnerable_rust_files/ - Real fixed code samples")
        print("  * dataset_summary.txt - Statistics and breakdown")
        print("  * successful_extractions.txt - Log of successful extractions")
        print("  * skipped_advisories.txt - Advisories without extractable code")
        print("\nVerification:")
        print("  * Each file has commit hash and GitHub repo in header")
        print("  * Use commit hash to verify code on GitHub")
        print("  * extraction_method column shows how code was obtained")
        print("=" * 80)