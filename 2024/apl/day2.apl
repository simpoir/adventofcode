#!/usr/bin/apl -s -L ./lib.apl --
sample ← """
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"""
expected ← 2 4

⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ gen
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
parse ← {⍎¨⍵}


⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ part1
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
RANGE←{^/(|⍵)∊1 2 3}
DIR←{a←0<¨⍵ ⋄ (∧/a)∨∧/~a}
ISVALID ← {{(DIR⍵)∧RANGE⍵}(2-/⍵)}
∇r←part1 data
 r ← +/ISVALID¨data
∇


⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ part2
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
SPLICE←{∊((⍺-1)↑⍵)(⍺↓⍵)}
∇r←part2 data
 r ← +/{L←⍵ ⋄ ∨/{ISVALID (⍵ SPLICE L)}¨⍳≢⍵}¨data
∇
main
