-- Copyright (C) 2014 Jorge Aparicio

ans = 0
curr = 1
next = 2

while curr < 4000000 do
  if curr % 2 == 0 then
    ans = ans + curr
  end

  tmp = next
  next = next + curr
  curr = tmp
end

print(ans)

