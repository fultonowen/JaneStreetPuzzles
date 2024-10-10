# Knight Moves 6

## Quick Overview

Used backtracking to constrain A+B+C values less than 50. Any paths where current
score was more than 2024 were pruned, along with paths longer than the accumulated minimum jump path.

## a1 to f6 Paths

### A: 3 B: 2 C: 1
Shortest Cell Path: 30,19,6,14,18,7,15,28,32,21,34,26,22,9,5  
Formatted path: a1,b3,a5,c4,a3,b5,d4,e2,c1,d3,e1,c2,e3,d5,f6  
(((3 + 3 + 3) * 2 * 3 * 2 + 2 + 2) * 3 * 2 + 2) * 3 * 1 + 1 + 1 = 2024

### A: 3 B: 1 C: 2
Shortest Cell Path: 30,26,34,23,27,31,20,28,32,21,25,12,8,0,13,9,5  
Formatted path: a1,c2,e1,f3,d2,b1,c3,e2,c1,d3,b2,a4,c5,a6,b4,d5,f6

### A: 2 B: 3 C: 1
Shortest Cell Path: 30,26,34,21,8,16,20,24,32,19,27,31,18,14,22,9,5  
Formatted path: a1,c2,e1,d3,c5,e4,c3,a2,c1,b3,d2,b1,a3,c4,e3,d5,f6

### A: 1 B: 3 C: 2
Shortest Path: 30,26,34,21,29,33,25,12,20,7,3,14,27,16,5  
Formatted path: a1,c2,e1,d3,f2,d1,b2,a4,c3,b5,d6,c4,d2,e4,f6  

## a6 to f1

### A: 3 B: 2 C: 1
Shortest Cell Path: 0,8,12,25,21,13,2,10,14,1,9,20,16,27,35  
Formatted path: a6,c5,a4,b2,d3,b4,c6,e5,c4,b6,d5,c3,e4,d2,f1  
Quick Answer Check:  
(((3 * 2 * 3 + 3) * 2 * 3 * 2 * 1 * 2) + 2) * 1 * 2 * 1 * 2  * 1 = 2024

### A: 3 B: 1 C: 2
Shortest Cell Path: 0,8,16,20,31,27,19,15,28,17,9,13,21,34,26,22,35  
Formatted path: a6,c5,e4,c3,b1,d2,b3,d4,e2,f4,d5,b4,d3,e1,c2,e3,f1  

### A: 2 B: 3 C: 1
Shortest Cell Path: 0,8,21,25,14,18,26,13,2,6,19,15,11,22,35  
Formatted path: a6,c5,d3,b2,c4,a3,c2,b4,c6,a5,b3,d4,f5,e3,f1

### A: 1 B: 3 C: 2
Shortest Path: 0,8,16,20,12,1,9,17,4,15,19,27,14,22,35  
Formatted path: a6,c5,e4,c3,a4,b6,d5,f4,e6,d4,b3,d2,c4,e3,f1  

## Submitted answer:
3,2,1,a1,b3,a5,c4,a3,b5,d4,e2,c1,d3,e1,c2,e3,d5,f6,a6,c5,a4,b2,d3,b4,c6,e5,c4,b6,d5,c3,e4,d2,f1