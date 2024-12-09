#!/usr/bin/apl -s --
testmode←(⊂'-t')∊⎕arg
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
parse ← {⍎⍵}


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

⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ Main
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
∇r←load
 r←sample
 →(testmode)⍴0
 base ← ↑{(~⍵∊'/')⊂⍵} ↑{(~⍵∊'.')⊂⍵} ↑⌽⎕ARG
 fname ← ∊'../data/' base '.txt'
 r←⎕FIO[49] fname
∇
data ← parse¨load

⎕←'Part 1:',res1←part1 data
⍞←(testmode^~res1=expected[1]) ⍴⊂∊'expected' expected[1] 'got' res1

⎕←'Part 2:',res2←part2 data
⍞←(testmode^~res2=expected[2]) ⍴⊂∊'expected' expected[2] 'got' res2
