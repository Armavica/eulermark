// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>

unsigned int fibonacci(unsigned int n) {
  unsigned int a = 1;
  unsigned int b = 1;
  unsigned int t;

  while (--n) {
    t = a + b;
    a = b;
    b = t;
  }

  return b;
}

int main() {
  int acc = 0;

  for (int i = 1; ; i++) {
    int fib = fibonacci(i);

    if (fib > 4000000) {
      break;
    } else if (fib % 2 == 0) {
      acc += fib;
    }
  }

  printf("%d\n", acc);
  return 0;
}
