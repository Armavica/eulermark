// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>

static int P = 1000;

int main() {
  int a, b, c;

  for (c = P / 3 + 1; c < P / 2; c++) {
    for (b = (P - c) / 2 + 1; b < c; b++) {
      a = P - b - c;

      if (a*a + b*b == c*c) {
        printf("%d\n", a * b * c);

        return 0;
      }
    }
  }
}
