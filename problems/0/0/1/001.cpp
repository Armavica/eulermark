// Copyright (C) 2013 Jorge Aparicio

#include <iostream>

using namespace std;

main() {
  int acc = 0;

  for (int i = 1; i < 1000; i++) {
    if (i % 3 == 0 || i % 5 == 0) {
      acc += i;
    }
  }

  cout << acc << endl;

  return 0;
}
