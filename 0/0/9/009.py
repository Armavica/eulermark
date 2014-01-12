# Copyright (C) 2013 Jorge Aparicio

P = 1000

print(next((P - b - c) * b * c
           for c in range(P // 3 + 1, P // 2)
           for b in range((P - c) // 2 + 1, c)
           if (P - b - c) * (P - b - c) + b * b == c * c))
