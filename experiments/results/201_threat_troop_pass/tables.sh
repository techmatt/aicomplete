#!/bin/sh
# 201: writes the item's tables from the reads in out/ (rerun from the repo root).
R=experiments/results/201_threat_troop_pass
P="python $R/paired.py"
V="python $R/vibe_read.py"
{
echo "# 201: the fresh ladder (strategist), paired against r88"
echo
echo "## First read, seeds 45055-45198 (144 a pair, 432 games a level), r89 at 200's dials (compute 1,4,9,28,30)"
echo
$P r88=out/f-r88-s r89=out/f-s
echo
echo "## Dial screens on those spent seeds (d0 compute 0; d2 agents 1 or compute 17; d3 compute 22 or 18)"
echo
$P r89=out/f-s a2=out/d1:a2 c2=out/d1:c2 c0=out/d1:c0 c22=out/d2:c22 c18=out/d2:c18
echo
echo "## Confirm, seeds 45199-45342, r89 at its final dials"
echo
$P r88=out/c-r88-s r89=out/c-s
} > $R/ladder-strategist.md
{
echo "# 201: the fresh ladder (greedy), paired against r88"
echo
echo "## First read, seeds 45055-45102 (48 a pair), r89 at 200's dials"
echo
$P r88=out/f-r88-g r89=out/f-g
echo
echo "## Confirm, seeds 45199-45246, r89 at its final dials"
echo
$P r88=out/c-r88-g r89=out/c-g
} > $R/ladder-greedy.md
{
echo "# 201: screens (dev seeds 36800-36831, 32 a pair, d0-d3, strategist)"
echo
echo "Levers: p insight counters 3/4 (p5: 3/5); tn troops strike nodes (troop_node 0); ts troops not used up;"
echo "st1 troop_steps 1; m011 / m001 Musters by stage I/II/III; nw no Wall Muster; tt threat troops -1; sp Botnets"
echo "spread on four events; bc3 takedown 3 credits; ai agent income on six stage-II/III events; a0 / a1 agent income"
echo "0 / 1 at d1-d3; c1 compute 1 everywhere. Screens 1-2 on r88 (Annihilate on); 3-4 with Annihilate and the"
echo "antimatter action off (message 9119); 5 on the r89 build at 200's dials."
echo
echo "## Screen 1"
echo
$P base=out/s1 p=out/s1:p ptn=out/s1:p+tn ptnts=out/s1:p+tn+ts ptnm=out/s1:p+tn+m011 ptntsm=out/s1:p+tn+ts+m011 all=out/s1:p+tn+ts+m011+st1
echo
echo "## Screen 2 (A = p+tn+m011+nw+st1; Aoff: the eval's troop_exposure 0)"
echo
$P base=out/s1 A=out/s2:p+tn+m011+nw+st1 Aoff=out/s2x:p+tn+m011+nw+st1 m001=out/s2:p+tn+m001+nw+st1 wall=out/s2:p+tn+m011+st1 ts=out/s2:p+tn+m011+nw+st1+ts st2=out/s2:p+tn+m011+nw sp=out/s2:p+tn+m011+nw+st1+sp+bc3 ai=out/s2:p+tn+m011+nw+st1+ai
echo
echo "## Screen 3 (no Annihilate; F = p+tn+m011+st1+tt+sp+bc3+ai)"
echo
$P r88=out/s1 base=out/s3 p=out/s3:p T=out/s3:p+tn+m011+st1 Ttt=out/s3:p+tn+m011+st1+tt Tsp=out/s3:p+tn+m011+st1+tt+sp+bc3 F=out/s3:p+tn+m011+st1+tt+sp+bc3+ai Foff=out/s3x:p+tn+m011+st1+tt+sp+bc3+ai F001=out/s3:p+tn+m001+st1+tt+sp+bc3+ai Fp5=out/s3:p5+tn+m011+st1+tt+sp+bc3+ai
echo
echo "## Screen 4 (G = p+tn+m001+st1+tt+nw+sp+bc3+ai, the shipped rules; the agent dial's reach)"
echo
G=p+tn+m001+st1+tt+nw+sp+bc3+ai
$P r88=out/s1 G=out/s4:$G Ga1=out/s4:$G+a1 Ga0=out/s4:$G+a0 Ga0c1=out/s4:$G+a0+c1
echo
echo "## Screen 5 (message 9148, the r89 build at 200's dials): the Wall Muster back (w1); extra troops early (e1: Muster 1 on every stage-I card) or late (l2: stage-III Musters 2)"
echo
$P base=out/s5 w1=out/s5:w1 e1=out/s5:e1 l2=out/s5:l2
} > $R/screens.md
{
echo "# 201: the vibe measures (strategist, d0-d3)"
echo
echo "Snapshots are taken as production starts each round (after the action phase). Sources: factory (an adversary"
echo "factory's own troop), muster (an event's Muster), wall (the board's Muster where the Protector stands), threat,"
echo "project (projects, Discord cards and twists)."
echo
echo "## r88, fresh seeds 45055-45198"
echo
$V out/f-r88-s --cells base
echo "## r89 (first read, 200's dials), same seeds"
echo
$V out/f-s --cells base
echo "## r89 at its final dials, confirm seeds 45199-45342"
echo
$V out/c-s --cells base
echo "## r88, confirm seeds 45199-45342"
echo
$V out/c-r88-s --cells base
echo "## Screen 5 (dev seeds): the Wall Muster back, extra troops early or late"
echo
$V out/s5 --cells base,w1,e1,l2
} > $R/vibe.md
echo TABLES_DONE
