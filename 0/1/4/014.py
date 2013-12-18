# Copyright (C) 2013 Jorge Aparicio


def collatz_length(n):
    if not n in collatz:
        collatz[n] = collatz_length(3*n + 1 if n % 2 else n / 2) + 1

    return collatz[n]

collatz = {1: 1}

print(max(range(1, 1000001), key=collatz_length))
