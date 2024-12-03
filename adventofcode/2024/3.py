# ok, another parsing problem
# use regexes
import re, sys

instructions = sys.stdin.read()

########
regex = re.compile(r"mul\((\d{1,3}),(\d{1,3})\)")
pairs = re.findall(regex, instructions)
sum = 0
for (a, b) in pairs:
    sum += int(a)*int(b)
print(f"Part 1: {sum}")

########
regex = re.compile(r"mul\((\d{1,3}),(\d{1,3})\)|(do(?:n't)?\(\))")
triples = re.findall(regex, instructions)
enabled = True
sum = 0
for (a, b, c) in triples:
    if c == "do()":
        enabled = True
    elif c == "don't()":
        enabled = False
    elif enabled and a and b:
        sum += int(a)*int(b)
print(f"Part 2: {sum}")