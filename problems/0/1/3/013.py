# Copyright (C) 2013 Jorge Aparicio

with open('013.in') as f:
    print(str(sum(map(int, f.readlines())))[0:10])
