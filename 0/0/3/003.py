# Copyright (C) 2013 Jorge Aparicio


def factors(n):
    f = 2

    while n != 1:
        if f * f > n:
            yield n
            n = 1
        elif n % f:
            f += 2 if f % 2 else 1
        else:
            yield f
            n //= f


print(list(factors(600851475143)).pop())
