// Copyright (C) 2014 Jorge Aparicio

#include <stdio.h>

static int stride = 5;

int product(int* factors) {
  int out = 1;

  for (int i = 0; i < stride; i++) {
    out *= factors[i];
  }

  return out;
}

int main() {
  int chr, p, pos = 0, max = 0;
  int factors[5] = {0};

  FILE *file = fopen("008.in", "r");

  chr = fgetc(file);
  while (chr != EOF) {
    if (chr != '\n') {
      factors[pos] = chr - 48;
      p = product(factors);

      if (p > max) {
        max = p;
      }

      pos = (pos + 1) % stride;
    }

    chr = fgetc(file);
  }

  printf("%d\n", max);

  return 0;
}

