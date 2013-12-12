# Copyright (C) 2013 Jorge Aparicio


def fibonacci(n):
    if n not in fib:
        fib[n] = fib[n - 1] + fib[n - 2]
    return fib[n]


def even_fib_up_to(n):
    i = 1
    while True:
        f = fibonacci(i)
        if f > n:
            break
        if f % 2 == 0:
            yield f
        i += 1

fib = {1: 1, 2: 2}
print(sum(i for i in even_fib_up_to(4e6)))
