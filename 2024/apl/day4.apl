#!/usr/bin/apl -s -L ./lib.apl --
testmode←(⊂'-t')∊⎕arg
sample ← """
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"""
⍝ test result for sample
expected ← 18 9

⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ gen
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
parse ← {⊃⍵}


⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ part1
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
∇r←part1 D;X
 X←'XMAS'
 x←⌽X
 vX←4 1⍴X
 vx←⊖vX
 ⍝ Quad search
 r←+/∊(X⍷D)+x⍷D
 r←r++/∊vX⍷D
 r←r++/∊vx⍷D
 ⍝ diag search
 r←r++/∊{d←4 4↑⍵↓D⋄+/∊(1 1⍉d)(1 1⍉⌽d)∘.≡x X}¨¯1+⍳¯3+⍴D
∇


⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ part2
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
∇r←part2 D
 r←+/∊{d←3 3↑⍵↓D⋄(2 2⌷d='A')∧((⊂1 3 9 7)⌷∊d)∨.⍷'SSMMSSM'}¨¯1+⍳¯2+⍴D
∇

main
