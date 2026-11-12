#!/bin/sh
# 200: writes the item's tables from the reads in out/ (rerun from the repo root).
R=experiments/results/200_engine_overhaul
P="python $R/paired.py"
{
echo "# 200: the fresh ladder (strategist), paired against r87"
echo
echo "Seeds 44767-44910 (144 a pair, 432 games a level); r88 as frozen with d2 compute 17 here. d0, d1, d3 and d4 are the final config's"
echo "(their dials never moved after this read); d2's final dial (compute 9) is confirmed on 44911-45054 below."
echo
$P r87=out/f-r87-s r88=out/f-s
echo
echo "## d1-d2 confirm, seeds 44911-45054 (d1 here at agents 1, compute 8: not kept; d2 at compute 9: kept)"
echo
$P r87=out/c-r87-s r88=out/c-s
} > $R/ladder-strategist.md
{
echo "# 200: the fresh ladder (greedy), paired against r87"
echo
echo "Seeds 44767-44814 (48 a pair, 144 games a level). The bands are the strategist's."
echo
$P r87=out/f-r87-g r88=out/f-g
echo
echo "## d1-d2 confirm, seeds 44911-44958 (d1 at agents 1, compute 8: not kept; d2 compute 9)"
echo
$P r87=out/c-r87-g r88=out/c-g
} > $R/ladder-greedy.md
{
echo "# 200: ablations (the sim players never spend the resource; sources stay)"
echo
echo "Agent \`strategist:no_spend=N\` (1 chips, 2 research cubes, 3 antimatter), d3-d4, 3 pairs."
echo
echo "## Final build (r88 frozen), dev seeds 36832-36895, 192 games a level"
echo
$P full=out/fA0 no_chips=out/fA1 no_cubes=out/fA2 no_antimatter=out/fA3
echo
echo "## First read (before the cube engines, Annihilate Sway 2), seeds 36800-36863"
echo
$P full=out/sA0 no_chips=out/sA1 no_cubes=out/sA2 no_antimatter=out/sA3
echo
echo "## With the cube cards (n3), seeds 36800-36863"
echo
$P full=out/s2A0 no_cubes=out/s2A2
} > $R/ablations.md
{
echo "# 200: screens"
echo
echo "## Screen 1: plants at needs 1 (n1) x eval held antimatter 5 (am5), dev seeds 36800-36831, against r87"
echo
$P r87=out/b200-r87 base=out/s1 n1=out/s1:n1 am5=out/s1am5 n1am5=out/s1am5:n1
echo
echo "## Screen 2-3: the cube cards added (n3), and the eval's held antimatter at 2 / 3.5 / 5 on it, same seeds"
echo
$P r87=out/b200-r87 n2=out/s1am5:n1 w2=out/s3w2 w35=out/s3w35 w5=out/s3w5
echo
echo "## Screen 4: on dev seeds 36832-36927 (1440 games a side): n2 (no cube cards), n4 (with), r87"
echo
$P r87=out/s4r87 n2=out/s4n2 n4=out/s4n4
echo
echo "## Screen 5: Annihilate Sway 3 (ann3), plants at 0 play insight (p0), same seeds"
echo
$P r87=out/s4r87 base=out/s5 ann3=out/s5:ann3 p0=out/s5:p0 ann3p0=out/s5:ann3p0
echo
echo "## Screens 6-7 (calibration, d2-d3, dev seeds 36832-36927, with ann3)"
echo
$P base=out/s6 c19=out/s6:c19 c21=out/s6:c21 c26=out/s6:c26 c30=out/s6:c30 a3=out/s6:a3
echo
echo "x1: compute 1,4,21,26,26; x2: 1,4,21,30,30; x3: 1,4,17,30,30; x4: 1,4,24,30,30; all with agents 0,2,3,3,3"
echo
$P base=out/s7 x1=out/s7:x1 x2=out/s7:x2 x3=out/s7:x3 x4=out/s7:x4
echo
echo "## Screens 8-9 (calibration, d1-d2, on the spent fresh seeds 44767-44910)"
echo
$P base=out/f-s d1c2=out/s8:d1c2 d1a1=out/s8:d1a1 d2c13=out/s8:d2c13 d2c9=out/s8:d2c9 a1c8=out/s9:a1c8 a1c12=out/s9:a1c12
echo
echo "## Screens 11-12 (d1 between steps, spent fresh seeds 44767-44910; none adopted)"
echo
echo "cb2: counter_bonus_at 2; cc2: corrupt_cut_at 2; cc2cm1: with counter_more_at 1; a1cm1: agents 1 at d1 with counter_more_at 1"
echo
$P base=out/f-s cb2=out/s11:cb2 cc2=out/s11:cc2 cc2cm1=out/s12:cc2cm1 a1cm1=out/s12:a1cm1
echo
echo "## Screen 10 (information, not adopted): the antimatter action at 2 Discord, spent fresh seeds 44767-44910"
echo
$P base=out/f-s amd2=out/s10:amd2
} > $R/screens.md
python $R/resource_use.py out/f-s out/f-r87-s --top 10 > $R/resource-use.md
python $R/resource_use.py out/s10 --top 6 > $R/resource-use-amd2.md
echo TABLES_DONE
