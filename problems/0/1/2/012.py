from functools import reduce

primes = [2]


def factorize(n):
    factors = []

    for prime in primes:
        if n == 1:
            break

        if prime * prime > n:
            factors.append((n, 1))
            break

        i = 0
        while n % prime == 0:
            i += 1
            n /= prime

        if i != 0:
            factors.append((prime, i))

    if n != 1 and (len(primes) == 0 or n > primes[-1]):
        primes.append(n)

    return factors


def number_of_divisors(factors):
    return reduce(lambda acc, f: acc * (f[1] + 1), factors, 1)


def triangle(left_factors, right_factors):
    (m, n) = (len(left_factors), len(right_factors))
    (i, j) = (0, 0)

    out = []

    while i != m or j != n:
        if i == m:
            out.append(right_factors[j])
            j += 1
        elif j == n:
            out.append(left_factors[i])
            i += 1
        else:
            ((a, x), (b, y)) = (left_factors[i], right_factors[j])

            if a == 2:
                x -= 1
            elif b == 2:
                y -= 1

            if a > b:
                out.append((b, y))
                j += 1
            elif b > a:
                out.append((a, x))
                i += 1
            else:
                if x + y > 0:
                    out.append((a, x + y))

                i += 1
                j += 1

    return out


nod = 0
n = 2
nex = [(2, 1)]
while nod < 500:
    n += 1
    (cur, nex) = (nex, factorize(n))

    nod = number_of_divisors(triangle(cur, nex))

print(int(n * (n - 1) / 2))
