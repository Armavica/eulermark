# Copyright (C) 2013 Jorge Aparicio


def is_palindrome(n):
    s = str(n)
    return s == s[::-1]

print(max([a * b
      for a in range(100, 1000)
      for b in range(a, 1000)
      if is_palindrome(a * b)]))
