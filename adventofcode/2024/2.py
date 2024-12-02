# Another input-parsing problem :)
numbers = []
for line in open(0):
    numbers.append(list(map(int, line.split())))

########
def safe(line):
    increasing = line[0] < line[1]

    diffs = [line[n] - line[n-1] for n in range(1, len(line))]
    return max(map(abs, diffs)) <= 3 and all(map(lambda x: increasing and x > 0 or not increasing and x < 0, diffs))

safe_count = 0
for line in numbers:
    if safe(line):
        safe_count += 1
print(f"Part 1: {safe_count}")

########
# data's small, we can brute force
# use safe_count from part 1
for line in numbers:
    if safe(line):
        continue # already counted

    for index in range(len(line)):
        if safe(line[:index] + line[1 + index:]):
            safe_count += 1
            break
print(f"Part 2: {safe_count}")
