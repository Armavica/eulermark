# Copyright (C) 2013 Jorge Aparicio

import numpy as np

dirs = [(1, 0), (0, 1), (1, 1), (1, -1)]
size = 20
stride = 4

with open('011.in') as f:
    arr = np.asarray(list(map(int, f.read().split())), dtype=np.int_)
    arr = arr.reshape(size, size)

ans = 0
for dir_ in dirs:
    for i in range(size):
        if not 0 <= i + (stride - 1) * dir_[0] < size:
            continue
        for j in range(size):
            if not 0 <= j + (stride - 1) * dir_[1] < size:
                continue
            p = arr[i, j]
            for k in range(1, stride):
                p *= arr[i + k * dir_[0], j + k * dir_[1]]

            if p > ans:
                ans = p

print(ans)
