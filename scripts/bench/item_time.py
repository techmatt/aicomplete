"""Where a conductor item's wall time went: sims, builds, tests, the rest.

Usage: python scripts/bench/item_time.py ITEM [ITEM...] [--top N]
ITEM is a done item's name, e.g. 098_rules_r38_troops_dark.

Reads (never writes) the item's run.json under the conductor control dir and its
main-session transcript under ~/.claude/projects. The wall clock from the run's
`started` to the transcript's last line is partitioned: a foreground tool call
(tool_use -> tool_result) is classed by its command; a wait (wait_for, Monitor,
a foreground sleep loop) by what writes the log or marker it waits on (the
latest earlier command that redirects into it); every uncovered second is the
model's (thinking, writing). Overlapping calls go to the highest class
(sim > build > test > rest). Background jobs a wait proves finished also give
the time that ran hidden behind the model's work. Subagent transcripts are read
only to say whether they ran sims, builds or tests themselves.
"""
import glob
import json
import ntpath
import os
import re
import sys
from collections import defaultdict
from datetime import datetime

CONTROL = r"C:\Code\aicomplete-control\done\aicomplete"
PROJECTS = os.path.join(os.path.expanduser("~"), ".claude", "projects")
PROJECT_PREFIX = "C--Code-aicomplete-worktrees-aicomplete-"

SIM = re.compile(
    r"aic-sim(\.exe)?\s+(experiment|run|replay)\b|\bexperiment\s+\S*\.json|screens\.py|"
    r"per_decision\.py|speed\.py|play_read\.py|board_read\.py|use_read|_read\.py|same_play\.py")
BUILD = re.compile(r"cargo\s+(\+\S+\s+)?(build|install)\b|npm\s+run\s+(wasm|build)\b|wasm-pack|build-wasm")
TEST = re.compile(
    r"cargo\s+(test|test-all|clippy|nextest)\b|npm\s+run\s+(check|e2e|test)\b|docs\s+--check|playwright|e2e\.mjs|cards-fit|cards:fit")
RANK = {"sim": 3, "build": 2, "test": 1, "rest": 0}
SEP = re.compile(r";|&&|\|\||\n|\(|\)|(?<![>&|])&(?![>&])|\bdo\b|\bthen\b")


def ts(s):
    return datetime.fromisoformat(s.replace("Z", "+00:00")).timestamp()


def classify(text):
    for name, rx in (("sim", SIM), ("build", BUILD), ("test", TEST)):
        if rx.search(text):
            return name
    return None


def log_names(text):
    """Basenames of out/ logs and markers a command or wait names."""
    return set(m.group(1) for m in re.finditer(r"[\\/]?([\w.\-]+\.(?:log|done|err|txt|out))\b", text))


class Producers:
    """Commands seen so far, to find what writes a given log or marker."""

    def __init__(self):
        self.cmds = []  # (time, text)

    def add(self, t, text):
        self.cmds.append((t, text))

    def find(self, name, before):
        """(class, launch time) of the latest command redirecting into `name`."""
        rx = re.compile(r">>?\s*\"?[^\s\"]*?" + re.escape(name) + r"\b|Redirect\w*\s+\"?[^\"]*" + re.escape(name))
        for t, text in reversed(self.cmds):
            if t > before or not rx.search(text):
                continue
            segs = [s for s in SEP.split(text) if s and s.strip()]
            last = None
            for s in segs:
                c = classify(s)
                if name in s and rx.search(s):
                    if c:
                        return c, t
                    if last:  # a marker echoed after the work: the work before it
                        return last, t
                if c:
                    last = c
            return classify(text), t
        return None, None


def name_guess(name):
    n = name.lower()
    if re.search(r"build|wasm", n):
        return "build"
    if re.search(r"test|clippy|check|e2e", n):
        return "test"
    if re.search(r"speed|perdec|screen|run|use-|r\d\d|fresh|final|split|d0|d2|onoff|lib-", n):
        return "sim"
    return None


def load(item):
    run = json.load(open(os.path.join(CONTROL, item + ".run.json"), encoding="utf-8"))
    pdir = os.path.join(PROJECTS, PROJECT_PREFIX + item.replace("_", "-"))
    sid = run["session_id"]
    main = os.path.join(pdir, sid + ".jsonl")
    files = [main] if os.path.exists(main) else sorted(glob.glob(os.path.join(pdir, "*.jsonl")))
    recs = []
    for f in files:
        for line in open(f, encoding="utf-8"):
            r = json.loads(line)
            if r.get("timestamp") and not r.get("isSidechain"):
                recs.append(r)
    recs.sort(key=lambda r: r["timestamp"])
    subs = glob.glob(os.path.join(pdir, sid, "subagents", "**", "agent-*.jsonl"), recursive=True)
    return run, recs, subs, files


def tool_calls(recs):
    """Yield (tool_use time, result time, name, input, result text)."""
    uses, out = {}, []
    for r in recs:
        msg = r.get("message") or {}
        content = msg.get("content")
        if not isinstance(content, list):
            continue
        for b in content:
            if not isinstance(b, dict):
                continue
            if r["type"] == "assistant" and b.get("type") == "tool_use":
                uses[b["id"]] = (ts(r["timestamp"]), b["name"], b.get("input") or {})
            elif r["type"] == "user" and b.get("type") == "tool_result" and b.get("tool_use_id") in uses:
                t0, name, inp = uses.pop(b["tool_use_id"])
                out.append((t0, ts(r["timestamp"]), name, inp, json.dumps(b.get("content"))))
    for t0, name, inp in uses.values():  # never answered: open to the end
        out.append((t0, None, name, inp, ""))
    out.sort(key=lambda c: c[0])
    return out


def command_of(name, inp):
    if name in ("Bash", "PowerShell"):
        return inp.get("command", "")
    return ""


def analyse(item):
    run, recs, subs, files = load(item)
    start = max(ts(run["started"]), ts(recs[0]["timestamp"]))
    end = ts(recs[-1]["timestamp"])
    prod = Producers()
    intervals = []  # (t0, t1, class, label)
    bg_jobs = []  # (t0, t1, class)
    unclassified = defaultdict(float)
    asks = 0.0
    for t0, t1, name, inp, result in tool_calls(recs):
        t1 = end if t1 is None else t1
        t0, t1c = max(t0, start), min(t1, end)
        if t1c <= t0:
            cmd = command_of(name, inp)
            if cmd:
                prod.add(t0, cmd)
            continue
        dur = t1c - t0
        cmd = command_of(name, inp)
        cls, label = None, name
        if cmd:
            prod.add(t0, cmd)
            label = cmd.strip().splitlines()[0][:90] if cmd.strip() else name
            detached = bool(re.search(r"(?<![>&|])&\s*($|\n|\||;)", cmd)) or inp.get("run_in_background")
            cls = None if detached and dur < 30 else classify(cmd)
            if cls is None and dur > 30:
                for n in log_names(cmd):
                    c, _ = prod.find(n, t0 - 0.001)
                    cls = cls or c
        elif name in ("mcp__conductor__wait_for", "Monitor", "TaskOutput"):
            target = inp.get("log") or inp.get("path") or inp.get("command") or inp.get("task_id") or ""
            n = ntpath.basename(str(target))
            label = "wait " + n
            c, launched = prod.find(n, t0)
            cls = c or name_guess(n)
            if cls and launched and re.search(r"has the line|exists", result):
                bg_jobs.append((launched, t1c, cls))
        elif name == "mcp__conductor__ask":
            asks += dur
        if cls is None:
            cls = "rest"
            if dur > 20 and name not in ("Read", "Edit", "Write", "Grep", "Glob", "mcp__conductor__ask",
                                         "mcp__conductor__progress"):
                unclassified[label] += dur
        intervals.append((t0, t1c, cls, label))

    # Partition [start, end]: each instant to the highest class covering it.
    points = sorted({start, end} | {t for iv in intervals for t in iv[:2]})
    wall = defaultdict(float)
    model_spans = []  # spans no tool covered: the model's own time
    for a, b in zip(points, points[1:]):
        mid = (a + b) / 2
        covering = [iv[2] for iv in intervals if iv[0] <= mid < iv[1]]
        if covering:
            wall[max(covering, key=RANK.get)] += b - a
        else:
            wall["rest"] += b - a
            model_spans.append((a, b))
    wall["model"] = sum(b - a for a, b in model_spans)

    # Background work hidden behind model time, per class (union of jobs).
    hidden = defaultdict(float)
    for cls in ("sim", "build", "test"):
        jobs = sorted((a, b) for a, b, c in bg_jobs if c == cls)
        merged = []
        for a, b in jobs:
            if merged and a <= merged[-1][1]:
                merged[-1][1] = max(merged[-1][1], b)
            else:
                merged.append([a, b])
        for a, b in merged:
            for ma, mb in model_spans:
                hidden[cls] += max(0.0, min(b, mb) - max(a, ma))

    sub = defaultdict(float)
    for f in subs:
        srecs = [json.loads(line) for line in open(f, encoding="utf-8")]
        srecs = [r for r in srecs if r.get("timestamp")]
        for t0, t1, name, inp, _ in tool_calls(srecs):
            c = classify(command_of(name, inp))
            if c and t1:
                sub[c] += t1 - t0
    steps = defaultdict(list)  # waited-on durations per class, for a typical step's cost
    for a, b, c, _ in intervals:
        if c != "rest" and b - a >= 10:
            steps[c].append(b - a)
    return {"item": item, "wall": end - start, "split": wall, "hidden": hidden, "steps": steps,
            "unclassified": unclassified, "asks": asks, "sub": sub, "files": files,
            "start": start, "end": end}


def fmt(sec, total):
    return f"{sec / 60:.0f} ({100 * sec / total:.0f}%)" if total else "0"


def main(argv):
    top = 8
    if "--top" in argv:
        i = argv.index("--top")
        top = int(argv[i + 1])
        argv = argv[:i] + argv[i + 2:]
    if not argv:
        sys.exit(__doc__)
    rows = [analyse(item) for item in argv]
    print("| item | wall min | sims | builds | tests | rest (model) |")
    print("|---|---:|---:|---:|---:|---:|")
    tot = defaultdict(float)
    for r in rows:
        s, w = r["split"], r["wall"]
        print(f"| {r['item']} | {w / 60:.0f} | {fmt(s['sim'], w)} | {fmt(s['build'], w)} | "
              f"{fmt(s['test'], w)} | {fmt(s['rest'], w)} ({fmt(s['model'], w)}) |")
        tot["wall"] += w
        for k in ("sim", "build", "test", "rest", "model"):
            tot[k] += s[k]
    w = tot["wall"]
    print(f"| **total** | {w / 60:.0f} | {fmt(tot['sim'], w)} | {fmt(tot['build'], w)} | "
          f"{fmt(tot['test'], w)} | {fmt(tot['rest'], w)} ({fmt(tot['model'], w)}) |")
    print("\nMinutes (share of wall). Rest includes short tools; the model figure in brackets is time no tool call was open.")
    print("\n| item | hidden sim | hidden build | hidden test | ask wait | subagent sim / build / test |")
    print("|---|---:|---:|---:|---:|---|")
    for r in rows:
        h, sb = r["hidden"], r["sub"]
        print(f"| {r['item']} | {h['sim'] / 60:.1f} | {h['build'] / 60:.1f} | {h['test'] / 60:.1f} | "
              f"{r['asks'] / 60:.1f} | {sb['sim'] / 60:.1f} / {sb['build'] / 60:.1f} / {sb['test'] / 60:.1f} |")
    print("\nHidden: background jobs a wait proved finished, overlapping the model's own time (minutes).")
    print("\nA waited-on step (a call or wait of 10 s or more), all items:")
    for cls in ("sim", "build", "test"):
        d = sorted(x for r in rows for x in r["steps"][cls])
        if d:
            print(f"- {cls}: {len(d)} steps, median {d[len(d) // 2] / 60:.1f} min, longest {d[-1] / 60:.1f} min")
    unc = defaultdict(float)
    for r in rows:
        for k, v in r["unclassified"].items():
            unc[f"{r['item'][:3]}: {k}"] += v
    print(f"\nTop unclassified tool calls over 20 s (counted as rest), {sum(unc.values()) / 60:.1f} min in all:")
    for k, v in sorted(unc.items(), key=lambda kv: -kv[1])[:top]:
        print(f"- {v / 60:.1f} min  {k}")


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    main(sys.argv[1:])
