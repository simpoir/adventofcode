#!/usr/bin/apl

⍝ Suppress gnu-apl DUMPED message
⍞←(⎕UCS 27),'[A',(⎕UCS 27),'[K'

testmode←(⊂'-t')∊⎕arg

⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
⍝ Main
⍝⍝⍝⍝⍝⍝⍝⍝⍝⍝
∇r←load ;base;fname
 r←sample
 →(testmode)⍴0
 base ← ↑{(~⍵∊'/')⊂⍵} ↑{(~⍵∊'.')⊂⍵} ↑⌽⎕ARG
 fname ← ∊'../data/' base '.txt'
 r←⎕FIO[49] fname
∇

∇main
 data ← parse load

 ⎕←'Part 1:',res1←part1 data
 ⍞←(testmode^~res1=expected[1]) ⍴⊂∊'expected' expected[1] 'got' res1

 ⎕←'Part 2:',res2←part2 data
 ⍞←(testmode^~res2=expected[2]) ⍴⊂∊'expected' expected[2] 'got' res2

 ⍎')OFF'
∇
