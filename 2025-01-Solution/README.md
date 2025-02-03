Generate all possible combinations 9 digits given 0 through 9

Constrain board to reduce run time:
- Given the board, generate the first row based on starting board possibilities
- Begin solving board trying every number
- If GCD at end of row(s) filled up is not greater than the current max, stop backtracking

### Max GCD: 12345679
   
### Max Solution:
|   |   |   |   |   |   |   |   |   |
|---|---|---|---|---|---|---|---|---|
| 3 | 9 | 5 | 0 | 6 | 1 | 7 | 2 | 8 |
| 0 | 6 | 1 | 7 | 2 | 8 | 3 | 9 | 5 |
| 7 | 2 | 8 | 3 | 9 | 5 | 0 | 6 | 1 |
| 9 | 5 | 0 | 6 | 1 | 7 | 2 | 8 | 3 |
| 2 | 8 | 3 | 9 | 5 | 0 | 6 | 1 | 7 |
| 6 | 1 | 7 | 2 | 8 | 3 | 9 | 5 | 0 |
| 8 | 3 | 9 | 5 | 0 | 6 | 1 | 7 | 2 |
| 5 | 0 | 6 | 1 | 7 | 2 | 8 | 3 | 9 |
| 1 | 7 | 2 | 8 | 3 | 9 | 5 | 0 | 6 |

### Puzzle Solution: 283950617