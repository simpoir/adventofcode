#!/usr/bin/apl -s -L ./lib.apl --
testmode←(⊂'-t')∊⎕arg
sample ← """
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"""
expected ← 161 48

parse←{∊⍵}

⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ part1
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
∇r←part1 data
 r←+/×/⍎¨⊃1↓¨'mul\((\d+),(\d+)\)' ⎕RE['g'] data
∇


⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ part2
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
∇r←part2 data;f
 f←1
 MATCHES←'do\(\)|don''t\(\)|mul\((\d+),(\d+)\)' ⎕RE['g'] data
 f←{f←('don''t()'≢⍵)∧(d←'do()'≡⍵)∨f⋄f∧~d}¨MATCHES
 r←+/{×/⍎¨1↓⍵}¨↑,/f⊂MATCHES
∇

main
