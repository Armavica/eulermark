// Copyright (C) 2013 Jorge Aparicio

#include <stdio.h>

#define PERIMETER 1000
#define HALF_PERIMETER PERIMETER / 2

int main() {
  int a, b, c;

  for (c = HALF_PERIMETER; c > 0; c--) {
    for (b = c - 1; b > 0; b--) {
      a = PERIMETER - b - c;

      if (a*a + b*b == c*c) {
        printf("%d\n", a * b * c);

        return 0;
      }
    }
  }
}
