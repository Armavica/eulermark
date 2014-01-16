// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>
#include <stdbool.h>

bool isPalindrome(int n) {
  int reversed = 0, tmp = n;

  while (tmp) {
    reversed = 10*reversed + (tmp%10);
    tmp /= 10;
  }

  return reversed == n;
}

int main() {
  int i, j, p, max = 0;

  for (i = 100; i < 1000; i++)
    for (j = i; j < 1000; j++) {
      p = i * j;

      if (p > max && isPalindrome(p))
        max = p;
    }

  printf("%u\n", max);

  return 0;
}

