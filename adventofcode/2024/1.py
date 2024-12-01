# this is mostly just an input parsing problem
a = []
b = []

for line in open(0):
    [i, j] = line.split("   ")
    a.append(int(i))
    b.append(int(j))
a.sort()
b.sort()

########
difference = 0
for i, j in zip(a, b):
    difference += abs(i - j)
print(f"Part 1: {difference}")

########
from collections import Counter
counts = Counter(b)
similarity = 0
for i in a:
    similarity += i * counts[i]
print(f"Part 2: {similarity}")