import csv, sys
csv.field_size_limit(sys.maxsize)
rows = list(csv.DictReader(open('RustXec/metadata.csv', newline='', encoding='utf-8')))
cols = rows[0].keys()
BAD = {'[N/A]', 'N/A', '[]', '', 'None'}

print('ROWS:', len(rows))
print()
print('=== COVERAGE ===')
for c in cols:
    n = sum(1 for r in rows if (r[c] or '').strip() not in BAD)
    print('%-28s %3d/%d' % (c, n, len(rows)))
print()
for i in (0, 1, 50):
    print('=== RECORD %d ===' % i)
    for c in cols:
        print('  %-24s: %s' % (c, (rows[i][c] or '').replace('\n', ' | ')[:220]))
    print()
print('=== fix commit links (8) ===')
for r in rows[:8]:
    print(' ', (r['fix commit links'] or '').replace('\n', ' | ')[:200])
print()
print('=== affected.functions (8) ===')
for r in rows[:8]:
    print(' ', (r['affected.functions'] or '').replace('\n', ' | ')[:200])
print()
print('=== severity ===')
from collections import Counter
for k, v in Counter((r['severity'] or '').strip() for r in rows).most_common():
    print('  %-20s %d' % (k, v))
