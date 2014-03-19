// Copyright (C) 2014 Jorge Aparicio

var ans = 0;
var curr = 1;
var next = 2;

while (curr < 4000000) {
  if (curr % 2 == 0) {
    ans += curr;
  }

  var tmp = next;
  next += curr;
  curr = tmp;
}

console.log(ans);

