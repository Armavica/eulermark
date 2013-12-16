# Copyright (C) 2013 Jorge Aparicio

s = (i for i in [1..999] when i % 3 == 0 or i % 5 == 0).reduce (p, c) -> p + c
console.log s
