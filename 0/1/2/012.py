from collections import defaultdict
from itertools import count


def triangle_number(n):
    return n * (n + 1) // 2


def next_prime(n, hint=2):
    for factor in range(hint, n + 1):
        if n % factor == 0:
            return factor
        elif factor * factor >= n:
            return n


def factorize(n):
    factors = defaultdict(lambda: 0)

    factor = 2

    while (n != 1):
        factor = next_prime(n, factor)

        while (n % factor == 0):
            factors[factor] += 1
            n //= factor

    return factors


def product(xs):
    prod = 1
    for x in xs:
        prod *= x
    return prod


def number_of_divisors(n):
    return product(map(lambda x: x + 1, factorize(n).values()))

print(next(filter(lambda x: number_of_divisors(x) > 500,
                  map(triangle_number, count(1)))))
