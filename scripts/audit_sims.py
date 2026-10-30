import json, difflib, statistics, collections

pairs = json.load(open('clean_pairs.json'))

def normalize(text):
    lines = [l.strip() for l in text.splitlines()
             if l.strip() and not l.strip().startswith('//')
             and not l.strip().startswith('/*')]
    return '\n'.join(lines)

stored, recomputed = [], []
for p in pairs:
    a = normalize(open(p['vuln_file'], errors='ignore').read())
    b = normalize(open(p['safe_file'], errors='ignore').read())
    recomputed.append(difflib.SequenceMatcher(None, a, b).ratio())
    stored.append(p['body_sim'])

n = len(stored)
ms, mr = sum(stored)/n, sum(recomputed)/n
cov = sum((s-ms)*(r-mr) for s, r in zip(stored, recomputed))/n
r = cov/(statistics.pstdev(stored)*statistics.pstdev(recomputed))
print(f'pearson r(stored, recomputed) = {r:.4f}')

def tier(s): return 'large' if s < 0.70 else 'moderate' if s <= 0.95 else 'tiny' if s <= 0.99 else 'header'
agree = sum(1 for s, rr in zip(stored, recomputed) if tier(s) == tier(rr))
print(f'tier agreement: {agree}/{n}')
print('stored tier counts:    ', dict(collections.Counter(tier(s) for s in stored)))
print('recomputed tier counts:', dict(collections.Counter(tier(rr) for rr in recomputed)))
print('recomputed mean +/- std: {:.3f} +/- {:.3f}'.format(mr, statistics.pstdev(recomputed)))
off = [(i, round(s,3), round(rr,3)) for i, (s, rr) in enumerate(zip(stored, recomputed)) if abs(s-rr) > 0.3]
print(f'pairs off by >0.3: {len(off)}   first 10: {off[:10]}')

# misalignment test: are the VALUES right but attached to the wrong pairs?
ss, rs = sorted(stored), sorted(recomputed)
d = sum(abs(a-b) for a, b in zip(ss, rs))/n
print(f'mean |sorted stored - sorted recomputed| = {d:.4f}  (small => values fine, assignment scrambled)')