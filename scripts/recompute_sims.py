import json, difflib, hashlib, statistics, collections, shutil
import pandas as pd

# same normalize() as build_ml_dataset_pub.py -- keeps the paper's analyses consistent
def normalize(text):
    lines = [l.strip() for l in text.splitlines()
             if l.strip() and not l.strip().startswith('//')
             and not l.strip().startswith('/*')]
    return '\n'.join(lines)

def sha256(t):
    return hashlib.sha256(t.encode('utf-8', errors='replace')).hexdigest()

df = pd.read_csv('dataset.csv')
pairs_out, sims = [], []
mismatch = 0
for rid, g in df.groupby('rustsec_id'):
    v = g[g['label'] == 'vulnerable']
    s = g[g['label'] == 'safe']
    if len(v) != 1 or len(s) != 1:
        print(f'  WARNING {rid}: {len(v)} vuln / {len(s)} safe files, skipping')
        continue
    vf, sf = v.iloc[0]['file_path'], s.iloc[0]['file_path']
    a = normalize(open(vf, errors='ignore').read())
    b = normalize(open(sf, errors='ignore').read())
    # DOCUMENTED MEASURE: character-level difflib.SequenceMatcher ratio,
    # autojunk disabled, computed on normalize()d text (whitespace stripped,
    # blank and comment lines removed)
    sim = difflib.SequenceMatcher(None, a, b, autojunk=False).ratio()
    # sanity: hash-identical pairs must score exactly 1.0
    if sha256(a) == sha256(b) and abs(sim - 1.0) > 1e-9:
        mismatch += 1
    method = v.iloc[0].get('extraction_method', v.iloc[0].get('method', 'unknown'))
    pairs_out.append({'rustsec_id': rid, 'method': method,
                      'body_sim': sim, 'vuln_file': vf, 'safe_file': sf})
    sims.append(sim)

n = len(sims)
n_identical = sum(1 for s in sims if s == 1.0)
print(f'\nPairs recomputed: {n}')
print(f'Hash/sim consistency violations: {mismatch} (must be 0)')
print(f'Pairs with sim exactly 1.0: {n_identical}  (Table II says 90 -- should match)')

def tier(s):
    return 'header(>0.99)' if s > 0.99 else 'tiny(0.95-0.99)' if s > 0.95 \
        else 'moderate(0.70-0.95)' if s >= 0.70 else 'large(<0.70)'
counts = collections.Counter(tier(s) for s in sims)
print('\n==== CORRECTED TIER COUNTS (all 537, for Table III) ====')
for k in ['header(>0.99)', 'tiny(0.95-0.99)', 'moderate(0.70-0.95)', 'large(<0.70)']:
    print(f'  {k}: {counts[k]}  ({100*counts[k]/n:.1f}%)')

hdr_methods = collections.Counter(p['method'] for p in pairs_out if p['body_sim'] > 0.99)
print(f'\nHeader-only tier by extraction method: {dict(hdr_methods)}')
print('  (paper currently claims these are exclusively crate_versions -- verify!)')

cleaned = [s for s in sims if s <= 0.99]
print(f'\nCleaned pair set (sim <= 0.99): {len(cleaned)} pairs, '
      f'mean {sum(cleaned)/len(cleaned):.3f} +/- {statistics.pstdev(cleaned):.3f}')

shutil.copy('clean_pairs.json', 'clean_pairs_BUGGY_backup.json')
with open('clean_pairs.json', 'w') as f:
    json.dump([p for p in pairs_out if p['body_sim'] <= 0.99], f, indent=2)
with open('all_pairs_sims_v2.json', 'w') as f:
    json.dump(pairs_out, f, indent=2)
print('\nWrote: clean_pairs.json (corrected cleaned set; buggy original backed up)')
print('       all_pairs_sims_v2.json (all 537 with corrected sims)')
