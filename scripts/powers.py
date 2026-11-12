"""An AI board power's counters, as the engine names them (`ai::power_slug`, crates/aic-core/src/modules/ai.rs).

Since 198 they are keyed by the power's name: `ai.power.<board>.<slug>` (uses a round) and
`ai.dear.<board>.<slug>.<why>` (Peer Review: `ai.power.researcher.peer_review`), so a cut power no longer relabels
the rest. Runs written before 198 keyed them by the power's index in that revision's board
(`ai.power.<board>.<k>`); this module reads only the name keys.
"""
import re


def power_slug(name):
    """A power's name lowercased, each run of other characters one `_`."""
    return re.sub(r'[^a-z0-9]+', '_', name.lower()).strip('_')


def power_key(board, name):
    """The per-round use counter of board `board`'s power `name`."""
    return f"ai.power.{board}.{power_slug(name)}"


def dear_key(board, name, why):
    """The dear-power tally of board `board`'s power `name` for reason `why` (idle, no_tags, no_target, unpaid)."""
    return f"ai.dear.{board}.{power_slug(name)}.{why}"
