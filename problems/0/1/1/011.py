# Copyright (C) 2013 Jorge Aparicio

directions = [(1, 0), (0, 1), (1, 1), (1, -1)]
size = 20
stride = 4

with open('011.in') as f:
    arr = list(map(lambda line: list(map(int, line.split())), f.read().split('\n')))

ans = 0
for direction in directions:
    for i in range(size):
        if not 0 <= i + (stride - 1) * direction[0] < size:
            continue
        for j in range(size):
            if not 0 <= j + (stride - 1) * direction[1] < size:
                continue
            p = arr[i][j]
            for k in range(1, stride):
                p *= arr[i + k * direction[0]][j + k * direction[1]]

            if p > ans:
                ans = p

print(ans)
