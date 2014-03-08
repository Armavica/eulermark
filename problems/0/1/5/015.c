// Copright (C) 2013 Jorge Aparicio

#include <stdio.h>

#define SIZE 20

int main() {
  int i, j;
  long arr[SIZE + 1][SIZE + 1] = {0};

  for (i = 0; i <= SIZE; i++)
    for (j = 0; j <= SIZE; j++)
      if (i == 0 || j == 0)
        arr[i][j] = 1;
      else
        arr[i][j] = arr[i-1][j] + arr[i][j-1];

  printf("%ld\n", arr[SIZE][SIZE]);

  return 0;
}

