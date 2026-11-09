"""Markdown table of sweep results from out/tune/log.jsonl.

    python scripts/tune/log_table.py TAG [TAG...]
"""

import json
import os
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')


def main():
    tags = set(sys.argv[1:])
    print("| tag | variant | agent | games | win % | rounds | ended |")
    print("|---|---|---|---|---|---|---|")
    with open(os.path.join("out", "tune", "log.jsonl"), encoding="utf-8") as f:
        for line in f:
            d = json.loads(line)
            if tags and d["tag"] not in tags:
                continue
            ended = ", ".join(f"{k} {v}" for k, v in sorted(d["ended"].items()))
            print(f"| {d['tag']} | {d['name']} | {d['agent']} | {d['games']} | "
                  f"{100 * d['win_rate']:.1f} | {d['rounds']:.2f} | {ended} |")


if __name__ == "__main__":
    main()
