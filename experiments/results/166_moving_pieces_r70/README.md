# 166: moving pieces (r70)

Matt's rulings (design doc *Moving pieces*, 2026-11-09; direction locked, numbers and prices candidates): pieces on
the board move. **Gather up to N** pieces into a hex in your reach, each from a hex next to it; **Push up to N**
pieces from a hex in your reach, each to a hex next to it, the player choosing where. Pieces: ambassadors (any
seat's), agents (the adversary's) and troops; botnets and corruption markers don't move. Kettle Operation and Lure
Network become gather-then-strike; Liberate may spend the ambassadors standing on the city; printed text calls the
adversary's pieces agents. `RULES_REVISION` 69 → 70, bumped from the constant (the prompt also said r70).

Scripts: [rates166.py](rates166.py) (drafted / played per card from games.jsonl lines), [flips166.py](flips166.py)
(the seeds two agents of one run split on, with their move counters), [rename166.py](rename166.py) (the printed
rename, rerunnable), [notes166.py](notes166.py) (the pricing notes), [cards166.overlay](cards166.overlay) (every
card edit; `apply_overlay.py`).

## The rules (r70)

- **Ops** (`op.rs`; resolved in `effects/moves.rs`): `gather { max, pieces, strike }` (replacing 158's
  `gather { troops, strike }`), `push { max, pieces }`. A target is a land hex in reach with a piece the op names
  next to it (gather) or on it (push). **Arrivals** (`moves::arrive`): an agent onto a standing ambassador cancels
  both (any seat's: `player_ambassador`), an ambassador onto agents cancels one, a firewalled hex takes no agent,
  troops just move. No piece moves onto ocean.
- **Batches (an engine choice, flagged):** after the hex, each pick is a batch from one hex ("2 agents from X",
  "3 troops to Y": 1 or as many as may go), repeated until the card's N or "stop". The rules allow every split; a
  batch only lets one answer move a stack. Why: the agents' search scores a card after 2-3 prompts, so one piece a
  prompt read a Kettle Operation as paying 18 credits for one removal (first build: Kettle 13% played when drafted,
  Checkpoint 0-6%, Honeypot 0%).
- **Strike** (Kettle Operation, Lure Network's Action, Honeypot Relay): after the hex, no more prompts: up to N
  adversary pieces next to it come off, troops first, then agents in hex order (each removal is a gain, so there is
  nothing to choose but where). It never lands what it gathers, so it never raises a Liberate bill.
- **Liberate** (`board::pay_liberate`): the ambassadors standing on the site pay first, the acting seat's, then the
  others' in seat order; the supply pays the rest (`liberate_supply_cost`, the action's target price). Applied to
  stations and factories too (the one site model); the ruling names the city.
- **Names:** "adversary ambassador" → "agent" in faces, content, drafts, house terms, `docs/rules/`, the player aid,
  harms.md, ops.md, the engine's prompts and labels, and the web panel (`rename166.py`); the Security token prints
  **Operatives** (`face::token`, `TokenKind::amount`; the code name `agents` stays). House terms: Ambassador (any
  player's), Agent, Gather up to N, Push up to N.

## Cards and the board power

Chosen from r69's play data on the first read's baseline side (`out/r69`, 288 games when chosen): cards seldom played when
drafted, or whose line the ledger rates low, and whose name fits a move. Each replaces its weakest line.

| Card | Was | Now | Played when drafted, confirm read: r69 → r70 |
|---|---|---|---|
| Kettle Operation (t2 event, 18 cr, 2 ins) | gather all at a corrupted city, strike 6 there | gather up to 6 agents or troops into a hex in reach and remove them | 27% → 37% |
| Lure Network (t2 building, 12 cr) | Action: gather at a corrupted city, strike 3 | Action: gather up to 3 agents and remove them | 49% → 41% |
| Checkpoint Network (t1 building, 12 → 7 cr) | Action, pay 2 cr: remove 2 agents in a country in reach | push up to 3 agents or troops; Action: push up to 2 | 1% → **3%** |
| Exodus Corridors (t2 program, 14 cr, 2 ins) | each ambassador steps 1 | gather up to 6 ambassadors | 42% → 50% |
| Honeypot Relay (t1 building, 12 cr) | draw 2 | draw 1; gather up to 2 agents and remove them | 54% → 47% |
| Rally the Allies (t2 event, 14 cr) | gain 5; move up to 3 of yours onto one piece | gain 5; gather up to 3 ambassadors | 79% → 81% |
| The Protector's **Patrol** (new power 7) | - | 1 energy: push up to 3 agents or troops | 1.2-1.4 uses a Protector game |

One board power: the Protector's (it holds ground; a push suits it). The Builder's and Researcher's powers are
economy; a move there would be a second identity.

### Give / get

(values.json table: printed = credits + 2.5 an insight, get = the effect ledger's reasoned value; anchors:
card-guidelines.md, a move 2 a piece, a removal 5, 4 an insight of keep and play.)

| Card | Table pay / get (ratio) | Anchors pay / get (ratio) | Gain the seat sees |
|---|---|---|---|
| Kettle Operation | 23.0 / 54.2 (2.36) | 34 / 38 (1.12) | six removals near a held hex, 2 Sway |
| Lure Network | 14.5 / 17.1 (1.18) | 24 / 10 + 15 a use | a removal engine from round 5 |
| Checkpoint Network | 9.5 / 13.0 (1.37) | 15 / 20 (1.33) | agents off a site before a Corrupt line |
| Exodus Corridors | 19.0 / 25.9 (1.36) | 30 / 29.5 (0.98) | a Liberate bill met from the board |
| Honeypot Relay | 14.5 / 17.7 (1.22) | 20 / 30.7 (1.54) | a draw, two removals paying its passive 3 credits |
| Rally the Allies | 16.5 / 39.4 (2.39) | unchanged price; gather 3 (6) for consolidate 3 | ambassadors onto a Liberate |
| Patrol (power) | 1 energy (4.5) / 4.2 | 4.5 / 6 (1.33) | the same, once a round |

Honeypot Relay at 1.54 at the anchors sits on the tier-1 ceiling (about 1.5); Exodus Corridors at 0.98 is just
under a fair 1.0 (a gather of 6 at 2 a piece). Both are flagged for Matt rather than repriced again.

### The tier-2 ceiling

The ledger now lists tier-2 cards over 2.5 at values.json (`outliers.md`, *Tier-2 cards over the ceiling*). Five
were over at r69's prices; each rose to the ceiling, credits rounded up: Mass Defection 8 → 11 (3.12 → 2.42), Open
Source Uprising 8 → 9 (2.63 → 2.39), Surgical Strike 14 → 15 (2.56 → 2.39), Open-Weights Release 10 → 12 (2.70 →
2.40), Collateralized Grid 10 → 11 (2.63 → 2.44). Rally the Allies (2.41) and Kettle Operation as rewritten (2.36)
were under. The last two are not r69 end-game cards; the done-when's "every tier-2 card" takes them in.

## The move heuristics (`agents/moves.rs`, dial `move_rules`)

Three sentence-sized rules on Gather and Push prompts; anything else, and every strike, is left to the search:

- **push:** push agents off the next Corrupt site (on it, or next to it when the coming line gathers from next door;
  `plan::next_corrupt_n`) onto empty land with no clean site next to it;
- **cancel:** gather (or, with no such land, push) agents onto a *spare* ambassador, one with no node, no player's
  site and no adversary site under it, as many as stand there (`moves::arrive` cancels them one for one);
- **liberate:** gather ambassadors that hold nothing onto an adversary site, when a seat in reach can then pay its
  Liberate.

Reviewed against the engine rules (the arrival rule, the Corrupt plan, `liberate_cost`) before the screens; message
7528 asked why cancel never fires: no card gathers agents without a strike (a strike removes them, it never lands
them), so cancel can answer only a push, and only with no empty land next door.

| Screen (d2, 3 pairs, on vs off) | on | off | paired | fires an on-game: push, cancel, liberate |
|---|---|---|---|---|
| v1, 36800-815 (any standing ambassador) | 83.3 | 85.4 | -2.1 | 1.64, 0.12, 0.18 |
| v1 after "stop" became the search's, 36800-815 | 83.3 | 91.7 | -8.3 | 1.48, 0.00, 0.24 |
| v2: no push onto a standing ambassador, 36800-831 | 86.5 | 89.6 | -3.1 | 0.78, 0.00, 0.08 |
| v3: spare ambassadors only, 36800-831 | 87.5 | 88.5 | -1.0 | 0.56, 0.00, 0.08 |
| **final (batches), the done-when's 16 seeds, 36800-815** | **87.5** | **85.4** | **+2.1** | 0.76, 0.00, 0.04 |
| **final, 32 untouched dev seeds, 36832-863** | **85.4** | **85.4** | **0.0** | 0.60, 0.00, 0.10 |

Each row is 48-96 games a side and the gaps are 1-8 games: noise-sized. The lost games of v1-v3 each had a push
that canceled an ambassador (an armor or a Liberate payment spent for one agent: `flips166.py`), which v2 removed.
After message 7528 nothing was tuned on 36800-815 again: the final rows are one build, read once.

## The full read

The standard read (the strategist, B+P, B+R, P+R, d0-d3, 48 seeds a cell: 576 games a side), r70 farmed with
`farm-seeds` (all shards on the main box, native), r69 (5b7899a's binary and configs, `out/r69`) run natively on the
same seeds, paired by `bench/ladder.py`. Bands don't judge this item (162's judge reads high).

| d | First read, freeze 1 (a7badc4), 40408-40455: r70 / r69 / paired | Confirm read, final build (c93bfd5), 40456-40503: r70 / r69 / paired |
|---|---|---|
| d0 | 100.0 / 100.0 / +0.0 ± 0.0 | 99.3 / 98.6 / +0.7 ± 1.2 |
| d1 | 95.8 / 93.8 / +2.1 ± 2.7 | 94.4 / 95.1 / -0.7 ± 2.7 |
| d2 | 84.7 / 81.9 / +2.8 ± 3.4 | 83.3 / 84.0 / -0.7 ± 3.6 |
| d3 | 79.2 / 75.7 / +3.5 ± 3.7 | 69.4 / 63.9 / +5.6 ± 4.4 |

The first read's seeds were spent by the card change that followed it (Honeypot Relay's draw 1, Rally the Allies'
gather), so the confirm read on the next free seeds is the one that judges the final build. The standing measures
move little (corruptions a game 10.5-12.0 against 10.8-11.8 at r69, agents placed within 2).

**Moves used** (confirm read; played in games where drafted; first read in brackets):

| Card | drafted | played when drafted | share | r69 share, same seeds |
|---|---|---|---|---|
| Kettle Operation | 412 | 154 | 37.4% (43.3%) | 26.7% |
| Lure Network | 401 | 165 | 41.1% (41.7%) | 49.1% |
| Honeypot Relay | 93 | 44 | 47.3% (12.9%, before its draw 1) | 54.1% |
| Exodus Corridors | 345 | 173 | 50.1% (46.9%) | 42.3% |
| Rally the Allies | 384 | 309 | 80.5% (-) | 79.0% |
| Checkpoint Network | 65 | 2 | **3.1%** (0.0%) | 1.3% |

The Protector's Patrol: 0.71 uses a game (about 1.1 a game with the Protector seated). A game pushes 1.27 pieces,
gathers 0.28, strikes 3.0 (r69: 1.6), and Liberate spends 2.16 standing ambassadors.

**Checkpoint Network** (message 7540): legal and kept (146 of 576 games), seldom played. The keep plan prices its
pushes at the table (`push` 2 a piece); the strategist's search plays a building for its eval delta, a building in
play worth `w.built` 4 + 0.5 x its cost. A push of distance 1 leaves an agent next to the site it left, so a Corrupt
line that gathers from next door still counts it, and the eval rightly sees little. The Patrol power (1 energy, no
slot) is used; the building is not. Screened: draw 1 on play at 9 credits, 2.6% -> 7.3% (dev 36864-895, not adopted);
as Matt's tier-1 example, an event at 2 credits and no insight ("push up to 3 agents or troops from a hex in reach"),
2.3% (1 of 44 drafted, dev 36896-927, not adopted). So it is not the price: a pure push does not beat what the eval
gives a card in hand (`w.hand` 3 + 0.4 x its cost) plus its credits. Left as frozen and reported unmet. The fix is
the agents' (an eval term for a push's later worth, or the keep plan's `push` row lowered so it isn't kept), not the
card's.

**Speed** (`bench/per_decision.py`, r69 against r70 on the same seeds, in turn, before the confirm read's load):
the strategist 1.18x a decision (5.1 against 4.4 ms; 8 games), greedy 0.77x (4 games). Under 1.3; no baseline moved.

## New-player summary

**Moving pieces, for new players.** Pieces on the board can be moved by cards. **Gather up to N** pulls pieces
into a hex in your reach, each from a hex next to it; **Push up to N** sends pieces out of a hex in your reach, each
to a hex next to it that you pick. The card says which pieces: ambassadors (anyone's), agents (the adversary's
pieces) or troops; botnets and corruption markers never move. You may stop early. A moved agent landing on an
ambassador removes both; a moved ambassador landing on agents removes one of them; nothing moves an agent onto a
firewall. Some cards (Kettle Operation, Lure Network, Honeypot Relay) gather agents and troops and remove them on
the spot. To **Liberate** a site, the ambassadors already standing on it pay first, whoever's they are; you pay the
rest from your supply. The Protector can **Patrol**: 1 energy to push up to 3 agents or troops.

**A rule that resisted a summary:** none of the player-facing rules did. One engine detail stays out of the
printed text: a gather or push is answered a batch at a time ("2 agents from X"), which the printed rule allows
but does not require.
