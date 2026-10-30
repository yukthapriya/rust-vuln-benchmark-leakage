#!/usr/bin/env python3
"""Build function-level vulnerable/fixed pairs from RustXec advisories."""
import argparse, csv, difflib, json, os, re, subprocess, sys
import urllib.request
from collections import Counter

csv.field_size_limit(sys.maxsize)

V4_CRATES = {"wasmtime","openssl","hyper","smallvec","tokio","pyo3","rustls",
             "zerocopy","slab","bumpalo","once_cell","ammonia","object_store"}
GIT_TIMEOUT = 600
NA = {"[N/A]","N/A","[]","","None","nan"}
PARSER = None


def load_parser():
    try:
        import tree_sitter_rust as tsrust
        from tree_sitter import Language, Parser
    except ImportError:
        sys.exit("pip install tree-sitter tree-sitter-rust")
    ptr = tsrust.language()
    lang = None
    for attempt in (lambda: Language(ptr), lambda: Language(ptr, "rust")):
        try:
            lang = attempt(); break
        except TypeError:
            continue
    if lang is None:
        sys.exit("could not construct Language object")
    try:
        return Parser(lang)
    except TypeError:
        p = Parser(); p.set_language(lang); return p


def _attrs_before(node, src):
    out, sib = [], node.prev_sibling
    while sib is not None and sib.type in ("attribute_item","line_comment","block_comment"):
        if sib.type == "attribute_item":
            out.append(src[sib.start_byte:sib.end_byte].decode("utf8","replace"))
        sib = sib.prev_sibling
    return " ".join(out)


def _qualified_name(node, src):
    nn = node.child_by_field_name("name")
    if nn is None: return None
    name = src[nn.start_byte:nn.end_byte].decode("utf8","replace")
    parts, cur = [], node.parent
    while cur is not None:
        if cur.type == "impl_item":
            t = cur.child_by_field_name("type")
            if t is not None:
                parts.append(src[t.start_byte:t.end_byte].decode("utf8","replace"))
        elif cur.type == "mod_item":
            n = cur.child_by_field_name("name")
            if n is not None:
                parts.append(src[n.start_byte:n.end_byte].decode("utf8","replace"))
        cur = cur.parent
    parts.reverse(); parts.append(name)
    return "::".join(parts)


def _in_test(node, src):
    if re.search(r"#\[\s*(tokio::)?test\s*\]|#\[\s*bench\s*\]", _attrs_before(node, src)):
        return True
    cur = node.parent
    while cur is not None:
        if cur.type == "mod_item" and "cfg(test)" in _attrs_before(cur, src).replace(" ",""):
            return True
        cur = cur.parent
    return False


def extract_functions(text):
    src = text.encode("utf8","replace")
    tree = PARSER.parse(src)
    found = {}
    def walk(n):
        if n.type == "function_item":
            qn = _qualified_name(n, src)
            if qn and not _in_test(n, src):
                found[qn] = (n.start_point[0]+1, n.end_point[0]+1,
                             src[n.start_byte:n.end_byte].decode("utf8","replace"))
        for c in n.children: walk(c)
    walk(tree.root_node)
    return found


def normalize(code):
    return "\n".join(s for s in (l.strip() for l in code.splitlines())
                     if s and not s.startswith("//"))


def body_similarity(a, b):
    na, nb = normalize(a), normalize(b)
    if not na and not nb: return 1.0
    return difflib.SequenceMatcher(None, na, nb, autojunk=False).ratio()


def run(cmd, cwd=None, check=True):
    r = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, timeout=GIT_TIMEOUT)
    if check and r.returncode != 0:
        raise RuntimeError(f"{' '.join(cmd[:4])} -> {r.stderr.strip()[:300]}")
    return r.stdout


def gh_api(url):
    req = urllib.request.Request(url, headers={"Accept":"application/vnd.github+json",
                                               "User-Agent":"rustxec-pair-builder"})
    tok = os.environ.get("GITHUB_TOKEN")
    if tok: req.add_header("Authorization", f"Bearer {tok}")
    with urllib.request.urlopen(req, timeout=60) as fh:
        return json.loads(fh.read().decode())


def classify_link(link):
    if "gitlab.com" in link: return None, "unsupported", "gitlab_host"
    m = re.search(r"github\.com/([^/]+)/([^/#?]+)", link)
    if not m: return None, "unsupported", "non_github_link"
    slug = f"{m.group(1)}/{m.group(2)}"
    if slug.endswith(".git"): slug = slug[:-4]
    mc = re.search(r"/commit/([0-9a-f]{7,40})", link)
    if mc: return slug, "commit", mc.group(1)
    mp = re.search(r"/pull/(\d+)", link)
    if mp: return slug, "pull", mp.group(1)
    return None, "unsupported", "no_commit_or_pr_in_link"


def resolve_revs(slug, kind, ref, repo_dir):
    os.makedirs(repo_dir, exist_ok=True)
    if not os.path.isdir(os.path.join(repo_dir, ".git")):
        run(["git","init","-q"], cwd=repo_dir)
        run(["git","remote","add","origin",f"https://github.com/{slug}.git"], cwd=repo_dir)
    if kind == "commit":
        run(["git","fetch","-q","--depth=2","origin",ref], cwd=repo_dir)
        fix = run(["git","rev-parse","FETCH_HEAD"], cwd=repo_dir).strip()
        base = run(["git","rev-parse",f"{fix}^"], cwd=repo_dir).strip()
        return base, fix
    commits = gh_api(f"https://api.github.com/repos/{slug}/pulls/{ref}/commits?per_page=100")
    if not commits: raise RuntimeError("pr_has_no_commits")
    parents = commits[0].get("parents") or []
    if not parents: raise RuntimeError("pr_first_commit_has_no_parent")
    base, fix = parents[0]["sha"], commits[-1]["sha"]
    for sha in (base, fix):
        run(["git","fetch","-q","--depth=1","origin",sha], cwd=repo_dir)
    return base, fix


HUNK_RE = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")


def changed_rust_lines(repo_dir, base, fix):
    diff = run(["git","diff","--unified=0","--no-renames",base,fix,"--","*.rs"], cwd=repo_dir)
    files, cur = {}, None
    for line in diff.splitlines():
        if line.startswith("+++ b/"):
            cur = line[6:]; files.setdefault(cur, (set(), set()))
        elif line.startswith("@@") and cur:
            m = HUNK_RE.match(line)
            if not m: continue
            o, ol, n, nl = int(m.group(1)), int(m.group(2) or 1), int(m.group(3)), int(m.group(4) or 1)
            old, new = files[cur]
            old.update(range(o, o+max(ol,1))); new.update(range(n, n+max(nl,1)))
    return files


def show(repo_dir, sha, path):
    r = subprocess.run(["git","show",f"{sha}:{path}"], cwd=repo_dir,
                       capture_output=True, text=True, timeout=GIT_TIMEOUT)
    return r.stdout if r.returncode == 0 else None


def is_test_path(p):
    low = p.lower()
    return any(s in low for s in ("/tests/","/test/","/benches/","/examples/","/fuzz/","tests.rs","/bench/"))


def main():
    global PARSER
    ap = argparse.ArgumentParser()
    ap.add_argument("--metadata", default="RustXec/metadata.csv")
    ap.add_argument("--out", default="rustxec_pairs")
    ap.add_argument("--cache", default="rustxec_repos")
    ap.add_argument("--max-per-advisory", type=int, default=3)
    ap.add_argument("--limit", type=int, default=0)
    args = ap.parse_args()

    PARSER = load_parser()
    samples_dir = os.path.join(args.out, "samples")
    os.makedirs(samples_dir, exist_ok=True); os.makedirs(args.cache, exist_ok=True)

    rows = list(csv.DictReader(open(args.metadata, newline="", encoding="utf-8")))
    if args.limit: rows = rows[:args.limit]

    pairs, labels, exclusions = [], [], []
    reasons = Counter()

    def drop(adv, crate, reason, detail=""):
        reasons[reason] += 1
        exclusions.append({"rustsec_id":adv,"crate":crate,"reason":reason,"detail":str(detail)[:300]})

    for i, r in enumerate(rows, 1):
        adv, crate = r["id"].strip(), r["package"].strip()
        raw = (r.get("fix commit links") or "").strip()
        links = [l.strip() for l in raw.split("\n") if l.strip() and l.strip() not in NA]
        print(f"[{i}/{len(rows)}] {adv} ({crate})", flush=True)
        if not links:
            drop(adv, crate, "no_fix_link"); continue
        emitted, last_err = 0, None
        for link in links:
            if emitted: break
            slug, kind, ref = classify_link(link)
            if slug is None:
                last_err = ("unsupported_link", ref); continue
            repo_dir = os.path.join(args.cache, slug.replace("/","__"))
            try:
                base, fix = resolve_revs(slug, kind, ref, repo_dir)
            except Exception as e:
                last_err = ("rev_resolution_failed", e); continue
            try:
                changed = changed_rust_lines(repo_dir, base, fix)
            except Exception as e:
                last_err = ("diff_failed", e); continue
            if not changed:
                last_err = ("no_rust_files_changed", link); continue
            candidates = []
            for path, (old_lines, new_lines) in changed.items():
                if is_test_path(path): continue
                so, sn = show(repo_dir, base, path), show(repo_dir, fix, path)
                if so is None or sn is None: continue
                try:
                    fo, fn = extract_functions(so), extract_functions(sn)
                except Exception:
                    continue
                to = {n for n,(s,e,_) in fo.items() if any(s<=L<=e for L in old_lines)}
                tn = {n for n,(s,e,_) in fn.items() if any(s<=L<=e for L in new_lines)}
                for name in sorted(to | tn):
                    if name not in fo or name not in fn:
                        drop(adv, crate, "function_missing_at_one_revision", f"{path}::{name}"); continue
                    vb, sb = fo[name][2], fn[name][2]
                    if normalize(vb) == normalize(sb):
                        drop(adv, crate, "identical_after_normalization", f"{path}::{name}"); continue
                    nch = sum(1 for L in old_lines if fo[name][0] <= L <= fo[name][1])
                    candidates.append((nch, path, name, vb, sb))
            if not candidates:
                last_err = ("no_usable_function_pairs", link); continue
            candidates.sort(key=lambda c: (-c[0], c[1], c[2]))
            kept = candidates[:args.max_per_advisory]
            for x in candidates[args.max_per_advisory:]:
                drop(adv, crate, "over_per_advisory_cap", f"{x[1]}::{x[2]}")
            for k, (_, path, name, vb, sb) in enumerate(kept):
                sid = f"{adv}__{k}"
                vf = os.path.join(samples_dir, f"{sid}__vuln.rs")
                sf = os.path.join(samples_dir, f"{sid}__safe.rs")
                open(vf,"w",encoding="utf-8").write(vb)
                open(sf,"w",encoding="utf-8").write(sb)
                overlap = crate in V4_CRATES
                pairs.append({"pair_id":sid,"rustsec_id":adv,"crate":crate,"repo":slug,
                              "base_sha":base,"fix_sha":fix,"path":path,"function":name,
                              "body_similarity":round(body_similarity(vb,sb),6),
                              "vuln_lines":len(vb.splitlines()),"safe_lines":len(sb.splitlines()),
                              "v4_crate_overlap":int(overlap)})
                for f, lab in ((vf,1),(sf,0)):
                    labels.append({"sample_id":os.path.basename(f),"file":f,"label":lab,
                                   "rustsec_id":adv,"crate":crate,"pair_id":sid,
                                   "v4_crate_overlap":int(overlap)})
                emitted += 1
        if not emitted:
            reason, detail = last_err or ("unknown_failure","")
            drop(adv, crate, reason, detail)

    def dump(name, recs, fields):
        with open(os.path.join(args.out,name),"w",newline="",encoding="utf-8") as fh:
            w = csv.DictWriter(fh, fieldnames=fields); w.writeheader(); w.writerows(recs)

    if pairs: dump("pairs.csv", pairs, list(pairs[0].keys()))
    if labels: dump("labels.csv", labels, list(labels[0].keys()))
    if exclusions: dump("exclusions.csv", exclusions, ["rustsec_id","crate","reason","detail"])

    advs = {p["rustsec_id"] for p in pairs}
    crates = {p["crate"] for p in pairs}
    sims = [p["body_similarity"] for p in pairs]
    ov = [p for p in pairs if p["v4_crate_overlap"]]
    L = ["RustXec function-level pair construction","="*46,
         f"advisories in metadata    : {len(rows)}",
         f"advisories yielding pairs : {len(advs)}",
         f"pairs emitted             : {len(pairs)}",
         f"files emitted             : {len(labels)}",
         f"distinct crates           : {len(crates)}",
         f"per-advisory cap          : {args.max_per_advisory}","",
         "body similarity within pairs"]
    if sims:
        s = sorted(sims)
        L += [f"  mean   {sum(sims)/len(sims):.4f}",
              f"  median {s[len(s)//2]:.4f}",
              f"  min {s[0]:.4f}  max {s[-1]:.4f}",
              f"  >0.99     {sum(1 for x in sims if x>0.99)}",
              f"  0.95-0.99 {sum(1 for x in sims if 0.95<x<=0.99)}",
              f"  0.70-0.95 {sum(1 for x in sims if 0.70<x<=0.95)}",
              f"  <=0.70    {sum(1 for x in sims if x<=0.70)}"]
    L += ["", f"pairs on crates shared with v4: {len(ov)} "
              f"({len({p['crate'] for p in ov})} crates) -- exclude from transfer test",
          "", "exclusion reasons"]
    for k, v in reasons.most_common(): L.append(f"  {k:38s} {v}")
    txt = "\n".join(L)
    open(os.path.join(args.out,"summary.txt"),"w",encoding="utf-8").write(txt+"\n")
    print("\n"+txt)


if __name__ == "__main__":
    main()
