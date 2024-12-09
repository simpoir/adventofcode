#!/usr/bin/apl -s -L ./lib.apl --

sample ← """
3   4
4   3
2   5
1   3
3   9
3   3
"""
⍝ test result for sample
expected ← 11 31

⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ gen
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
parse ← { ⊃{'%d    %d' ⎕FIO.sscanf⍵ }¨⍵ }


⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ part1
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
∇r←part1 data
 a ← data[⍳≢data;1]
 b ← data[⍳≢data;2]
 a←a[⍋a]
 b←b[⍋b]
 r←+/|a-b
∇


⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ part2
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
∇r←part2 data
 a ← ∊data[⍳≢data;1]
 b ← ∊data[⍳≢data;2]
 r←+/{⍵×+/[1]⍵⍷b}¨a
∇

main
