# 118 bug hunt: fixed (r46 → r47)

The findings the 118 hunt fixed, numbered on from the 099 hunt's F131. The open and won't-fix ones are in
[docs/qa-findings.md](../../../docs/qa-findings.md); the changes are in git. Seeds: 34272 on (fresh at r46).

| # | Fix |
|---|---|
| F132 | `play_read.py` and `use_read.py` list as "never kept" only the cards the runs dealt from (results.json's card rows): all 18 they listed at r46 were cards cut from the deck but still in content. |
| F133 | What a teammate trigger removes (Joint Task Force) fires its owner's fight triggers (`Cx::fought`), not those of the player whose card set it off; with no fight trigger on the player, nothing fired at all (r47). |
| F134 | At N pays whenever its tokens reach N: a feeder's tokens (Recruit Operatives, Deep State Ledger onto Intrusion Lab) waited for the card's own next trigger (r47). |
| F135 | "Move up to N" (Summit Caucus's consolidate, Liaison Corps's move) no longer vetoes a card that also gains ambassadors; Summit Caucus with nothing to bring asks for no piece (r47). |
| F136 | "Delay N threats" (Predictive Modeling, Early Warning Drill) offers only threats the seat may counter, the house term "a threat"; with none, it no longer vetoes the card's draw or shields (their plays fell to a third before that) (r47). |
| F137 | The megaproject converter texts: Horizon's, Cintamani's and Biomia's said "once a round" (the engine has no limit) and Cintamani's 1 insight a cube (it pays 2); the seat converter hover said "once a round" for every converter (Onyx's twice). |
| F138 | Synthesis's power text read ": discard a card from your hand…" (a power with no price). |
| F139 | Viewer: at 4 countries mid-game at 1366×768 the strips left the board 191 px (e2e's `4 countries` check failed by state); the board keeps 40% of the window and the country strip scrolls. |
| F140 | Viewer: an event waiting on its own choice showed nowhere (out of the hand, not yet discarded); it shows as "playing now". |
| F141 | `docs/cards.md`'s cost column left out research cubes (Forensic Audit: 27 cr; 4 rc). |
| F142 | Stale rules prose against content and code: card-grammar.md's Joint Task Force, Citation Engine, Lights-Out Assembly and Telemetry Archive; the empty-deck draw (a draw never reshuffles); player-cards.md's feeders (Vendor Financing feeds nothing; Deep State Ledger does), Intrusion Lab's first threshold, the 0/1/2-insight counts, the Science niche and typed-counter counts, Popular Uprising's `once` (gone since 106) and its liberation rate under Reuse; the rules README's Embassy Compound (cut); ai-powers.md's tier-2 keep under Synthesis and 5 insight a round (6 from round 1); economy.md's Parts Exchange (1 chip), energy-cost cards, Orbital Embassy, Positron Storage Ring; board.md's carded cubes (Moonshot Grant). |
| F143 | Agents: the keep plan counted a delay job for every face-up threat, though a delay now reaches only those the seat may counter (F136's agent side). |
| F144 | Fabrication Megaplex's income box printed "7 [credits] [chips]" (a 1 dropped in a two-resource box): read as 7 of each. |
| F145 | Viewer: the Synthesis button and log read "power: Synthesis (): …" (F138's web twin). |
| F146 | A gain-per-tag card short of tags reads "too few tags", not "no target" (Capacitor Discharge). |
| F147 | Viewer: draft and keep tiles show the credit cost and what the card needs (a new player kept four cards blind to "needs 2 cyber"). |
| F148 | A threat's prompts name the threat ("Commandeer: seat 0: discard which card (1 of 2)?"), not only the seat. |
| F149 | "place nodes (…) on which hex?" places one node: singular. The loss chip "node gone 2" says "a seat's last node gone 2 fewest held". |
| F150 | Viewer: a new choice scrolls the deciding seat's buttons into view (they sat below another seat's panel). |
| F151 | Measures gone quiet: `economy_read.py` divided by rounds that never reached upkeep (income 14% low); `energy_read.py`'s Popular Uprising row read a dead counter and its gift-Action row matched no card; `board_read.py --cards` listed five cards not in the deck (four cut) and missed Forensic Audit and Moonshot Grant; `corp_read.py`'s Cintamani column read a counter no content writes. |
| F152 | `docs/ops.md` counted no op in an Action's OR branch or a track step (gain_per_tokens read 0); op.rs's Threshold, DelayThreat and ClearCity docs were stale. |
| F153 | Stale docs, second pass: principles.md and card-pricing.md's cost range and medians (Moonshot Grant 46), ai-boards.md's free placements and median, economy.md's chips (Procure 2, Parts Exchange 1, Fabrication Megaplex) and insight rows, player-cards.md's self-filling engines (Agency Directorate), the README's and card-grammar's 2-hex move and card-set counts, Foundation Model Campus's 3 a tag. |
| F154 | Checker: `aic-sim docs --check` refuses a rules file quoting a card's words ("…" (Card Name)) its face no longer prints. |
| F155 | A Cordon (Hardpoint Lockdown) lasts the next round as printed: cleared after the build's arrivals, it let that build's own compute (Rapid Buildout, Total Offensive) land arrivals in the cordoned country (r47). |
| F156 | A threat a standing counter (Autonomous Defense Grid) pays in full is countered as it is dealt, or when the counter enters play: it sat face up at 0, payable by nobody, holding its botnets and corrupted stations against removal all round (r47). |
| F157 | Siege resolves in its printed order, nodes destroyed before the ambassador is lost: lost first, an ambassador's shield was stripped before the destroys (r47). |
| F158 | Field Grant's face says the megaproject must be incomplete, as the engine requires. |
| F159 | The card prompt "build joint_task_force where?" names the card. |
| F160 | `aic-sim replay` no longer tells a hand-written record (revision 0, "not recorded") that it came from rules revision 0. |
| F161 | A node power with every node token placed reads "no node left", not "unaffordable"; the keep-upgrade track step's hover gives the price, not a config key; the compute hover no longer names a hack that cuts a botnet (no content prints one). |
| F162 | Viewer, sessions and records: a record's warnings and older revision survive a reload (the first save stamped it r47); a session that no longer loads is kept (`aic.keptSession()`), not overwritten; the Record dialog says when its copy is re-stamped; the hover follows autoplay and the keyboard; undo at an earlier step asks first; New game, Random seed and file drop wait for the engine to load. |
| F163 | The card viewer's AI-board export prints a track step's effects (the Researcher's free station) and research cubes in words, and Synthesis as free. |
| F164 | Checker: `wasm-parity.mjs` adds engine-agent games on the baseline and at 3 countries, which play the r41+ engines (triggers, At N, reuse) the random and greedy games barely reached. |
| F165 | Stale docs, third pass: Open Archive's cubes (board.md), peek and spread_shield and the botnet hack named as printed (no content prints them), a player's station as a cyber target usually holds (threats.md), Deepfake Wave's and Think Tank's insight, Total War's bases, Embargo under shared_row, AI boards as a Unity source (gone since 050), token cards' growth and pay, the liberate engines, the Protector's troop, a standing ambassador on bases and factories, Harden's second price, a board's `cards` and `start` (gone since 091). |
| F166 | Agents: F149's prompt rename ("place a node") turned off the Protector's wall turtle (`policy::wall_hex` matched "place nodes"), unseen in batch 2's reads; one const (`board::NODE_PROMPT`) now writes and reads it. |
| F167 | An OR Action's prompt named its card by id ("synthetic_data_forge, which half of the Action?"). |
| F168 | "too few tags" (F146) only on a card that does nothing else; beside another op (Antitrust Action, Prefab Plant Kit) that op vetoed it, and the status reads "no target". The viewer colors it as a blocker. |
| F169 | A Cordon may be renewed for the next round on a country cordoned only this round (F155 left the picker skipping any flagged country); its op doc and card-grammar.md say it lasts the round. |
| F170 | Viewer, F139 and F150 reviewed: the country strip's floor (3em, from 7.5em) no longer overflows a real 1366×768 window; the deciding seat scrolls into view only at the live end, not on stepping back; the Record dialog's copy note keeps "the copy holds only N actions" for a game that stopped replaying. |
| F171 | Checker: the rules-quote check reads a card name wrapped across a line. |
| F172 | `play_read.py` and `use_read.py` (F132) take the deck from each run's config too, so a deck card no game drew still reads "never kept". |
| F173 | At `board.corp_energy` 0 a neutral city's energy was still sold: 5 credits for 0 energy. Not offered now; the prompt names the energy ("(1 energy)"). |
| F174 | At `board.chips_credits` 0 and `board.sell_credits` 0, selling a chip or a card was offered for nothing; not offered, as `cube_insight` 0 already did. |
| F175 | `alignment.unity_cards` above the printed Unity cards is refused (it dealt the whole deck, unsaid, while the agents and docs read the config's count); `unity_ease`'s documented minimum is 1, as the check refuses 0. |
| F176 | The prompts of what resolves name it: a threat's, a Discord card's, an adversary card's twist and a project's node destruction, troop landing and station flip ("Agitators: seat 0: destroy which of your nodes?"); F148 had named only the threat ops. |
| F177 | Viewer: "click the highlighted hex" for one hex (not "one of the 1 highlighted hexes"); a card's status no longer wraps mid-phrase ("too few tags"); the seed box shows a loaded record's seed. |
| F178 | Checker: the sweep's overlap check clips each button to its scroll pane, as `hidden-target` does: buttons scrolled out of the seat pane read as 925 overlaps with the toolbar once a choice scrolled the pane (F150). |
| F179 | Open Science (the Researcher's capstone) gave nothing in a 1-player game; alone, "other players" means you (ruling 7, r44) (r47). |
| F180 | The Cordon hover says "this round or next" at level 2 and "this round" at 1 (F155's "until the next round ends" read wrong at both); card-grammar.md says it holds from its play. |
| F181 | Viewer: the give button read "give N insight" with N the receiver's gain (`give_insight`); the giver pays 1. The Record dialog's "Copy up to the shown step" on a game that stopped replaying gave the whole game's count. |
| F182 | threats.md states F156's rule (a standing counter's full pay counters at once) and that it counters nothing under `shared_row` (D54); the T2-keep hover says "before Science discounts"; `runaway_read.py` says its insight is the seats' sum. |
| F183 | A program counts its own tags toward its gain per tag when judged playable, as it does when it resolves: Consular Pouch (1 ambassador a Diplomacy tag) was refused with no Diplomacy in play though it brings one (r47). |
| F184 | With `alignment.variant=off` a Sway does nothing: a card that only sways (Planetary Referendum) or sways beside a feeder (Manifesto Leak, 8 credits) and Biomia's converter are no longer offered for nothing. |
| F185 | F174's review: a Sell at `sell_credits` 0 is refused in `act`, not only hidden from the legal list. |
| F186 | Summit Caucus offers no destination that holds every ambassador of yours in reach (the pick moved nothing), and isn't offered for its move when none could move (F135's test was too loose). |
| F187 | F184's review: with alignment off, a Sway vetoes only a card that does nothing else (Planetary Referendum, Manifesto Leak with no token card), not the seven that sway beside a real effect (Peace Conference, Back-Channel Talks, Nuclear Option…): it is a "you may" op. |
| F188 | F183's agent side: the agents' play plan judges a program with its own tags counted, as the engine does (one helper, `cards::own_counted`). |
| F189 | ai-boards.md: alone, Open Science's insight is the Researcher's own and Patience keeps it (F179). |
| F190 | Card viewer: an AI board's face scrolls (the Researcher's capstones and the Protector's last line were cut after F163 lengthened its track); a space's income reads "+1 credit" (not "1 credits" beside its energy price); "AI board" no longer prints twice. |
| F191 | F190's review: the card viewer's detail line drops an empty tier ("AI board ·  · id"); a space with two incomes reads "+1 energy, +2 credits". |
| F192 | Experiment report: a spec with `records` 0 says records are off (it read "None matched"); the strongest agent's "Decisions" and "Help and counters" findings say they average its cells; the fresh-seed row prints its selection seeds inclusive, as the header does. |
| F193 | Six cut cards' scenes (Signal Hijack, Adaptive Malware, Planetary Referendum, Liquidity Event, Bounty Board, Peacekeeper Rotation) carry `was`, not `card`, so the scene pool shows them free (tools/card-art/README.md). |
| F194 | tools/card-art/README.md's counts (149 deck cards, 167 in content, 19 PDF pages) and card-guidelines' square-tile legend (research cubes). |
| F195 | `cards.variant=off` deals no player cards at all, as the rules say: the decks were full, so the board's draw powers (Procure, Debrief, Fieldwork) drew and the seats played what they drew (3.75 plays a game). The fresh-seed row's range reads "No selection seeds" at 0 (F192's review). |
| F196 | Content::check refuses a capstone off a track's last space (shown as uncovered, counted, never read) and an "add N here" of a kind the card doesn't hold (it counted toward the card's own token and fired no trigger). |
| F197 | D8 (the 099 hunt's, pick (a)): "move up to N" (Liaison Corps, Transmission Corridor) offers no piece that only its one ambassador already holds when the supply is empty (the pick moved nothing) (r47). |
