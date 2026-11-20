"""297: writes smoke.md, the before-and-after tables, from the farmed reads.

    python experiments/results/297_overhaul_5_content_sweep/make_smoke.py R142_DIR R143_DIR

R142_DIR holds 295's kept `f295-r142.jsonl` and `f295-r142-greedy.jsonl` (seeds 50424-50471) and this item's
`f297-r142-fresh*.jsonl` (seeds 50640-50687, farmed at 2282b5387d); R143_DIR holds `f297-r143*.jsonl`.
"""
import contextlib
import io
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import smoke_table  # noqa: E402


def table(argv):
    out = io.StringIO()
    old = sys.argv
    sys.argv = ["smoke_table.py"] + argv
    try:
        with contextlib.redirect_stdout(out):
            smoke_table.main()
    finally:
        sys.argv = old
    return out.getvalue()


def main():
    a, b = sys.argv[1], sys.argv[2]
    same = [f"judge r142={a}/f295-r142.jsonl", f"judge r143={b}/f297-r143.jsonl",
            f"greedy r142={a}/f295-r142-greedy.jsonl", f"greedy r143={b}/f297-r143-greedy.jsonl"]
    fresh = [f"judge r142={a}/f297-r142-fresh.jsonl", f"judge r143={b}/f297-r143-fresh.jsonl",
             f"greedy r142={a}/f297-r142-fresh-greedy.jsonl", f"greedy r143={b}/f297-r143-fresh-greedy.jsonl"]
    text = "\n".join([
        "# 297: the smoke read, before and after",
        "",
        "Written by `make_smoke.py`. Two players, the three board pairs (Builder + Protector, Builder + Researcher,",
        "Protector + Researcher), 48 seeds a cell, d0-d3, the judge (`strategist`) and greedy. r142 is main at",
        "2282b5387d; r143 is this item's final build (02ee710e). \"Slot reached\" is the research row's slot at the",
        "game's end; `deck_empty` is the round cap, `research` slot 7, `discord_deck` a tip on an empty Discord deck.",
        "",
        "## Same seeds as 295's read (50424-50471), the three pairs pooled, 144 games a row",
        "",
        table(same),
        "## Fresh seeds (50640-50687), the three pairs pooled, 144 games a row",
        "",
        "The r142 side was farmed at 2282b5387d on the same seeds. The same-seed table above is the read this item's",
        "content was shaped on (the Botnet Conscripts condition, the troop lines that land by the players, Breach and",
        "Clear); this one saw no change after it.",
        "",
        table(fresh),
        "## By pair, same seeds",
        "",
        table(same + ["--pairs"]),
    ])
    (HERE / "smoke.md").write_text(text, encoding="utf-8", newline="\n")
    print("wrote smoke.md", len(text))


main()
