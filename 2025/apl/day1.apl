#!/usr/bin/apl -s -L ./lib.apl --

sample ← """
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"""
sample ← 'L50' 'R200' 'L2'
⍝ test result for sample
expected ← 3 6

⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ gen
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
parse ← { {⍎∊(('L'∊⍵)/'¯')(1↓⍵) }¨⍵ }

⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ part1
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
∇r←part1 data
 r←+/{0=⍵}¨100|+\∊50 data
∇


⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ part2
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
∇r←part2 data
 p2←+/2{(|⌈⍺-⌈⍵)⌊(|⌊⍺-⌊⍵)}/{⍵÷100}¨+\∊50 data
 r←p2 + part1 data
∇

main
