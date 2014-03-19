014 - Longest Collatz sequence
------------------------------

The following iterative sequence is defined for the set of positive integers:

`n` -> `n`/2 (`n` is even)

`n` -> 3`n` + 1 (`n` is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains
10 terms. Although it has not been proved yet (Collatz Problem), it is thought
that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

**NOTE:** Once the chain starts the terms are allowed to go above one million.

Language | LoC
--- | :---:
CPython | 6
Haskell | 17
Rust | 26
C | 30

Language | aTime | aTime
--- | :---: | :---:
C |   14 ms | 100%
Rust | 25.5 ms | 181%
Haskell |  361 ms | 2567%
CPython | 1.32 s | 9409%

Language | rTime | rTime
--- | :---: | :---:
C | 13.5 ms | 100%
Rust | 24.2 ms | 178%
Haskell |  360 ms | 2660%
CPython | 1.31 s | 9717%
