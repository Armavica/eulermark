# Copyright (C) 2013 Jorge Aparicio

print(next(a * b * c
           for c in range(500, 0, -1)
           for b in range(c - 1, 0, -1)
           for a in range(1000 - b - c, 1001 - b - c)
           if a * a + b * b == c * c))
