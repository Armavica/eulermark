# Copyright (C) 2013 Jorge Aparicio

ans = 0
current = 1
next = 2
while current < 4000000
  if current % 2 == 0
    ans += current

  tmp = next
  next += current
  current = tmp

console.log ans
