// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>

int main() {
  int ans = 0, curr = 1, next = 2, tmp;

  while (curr < 4000000) {
    if (curr % 2 == 0)
      ans += curr;

    tmp = next;
    next += curr;
    curr = tmp;
  }

  printf("%d\n", ans);

  return 0;
}
