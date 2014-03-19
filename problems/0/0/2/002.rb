# Copyright (C) Jorge Aparicio

ans = 0
curr = 1
nxt = 2

while curr < 4000000 do
  if curr % 2 == 0 then
    ans += curr
  end

  tmp = nxt
  nxt += curr
  curr = tmp
end

puts ans

