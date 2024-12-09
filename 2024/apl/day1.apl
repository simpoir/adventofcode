#!/usr/bin/apl -s --
testmode←(⊂'-t')∊⎕arg
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
parse ← { '%d    %d' ⎕FIO.sscanf ⍵ }


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
data ← ⊃parse¨load

⎕←'Part 1:',res1←part1 data
⍞←(testmode^~res1=expected[1]) ⍴⊂∊'expected' expected[1] 'got' res1

⎕←'Part 2:',res2←part2 data
⍞←(testmode^~res2=expected[2]) ⍴⊂∊'expected' expected[2] 'got' res2
