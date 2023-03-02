(start, target) = input().split(" ")

if start == target:
    print(1)
    exit()

ddddd = set()
ddddd.add(start)
start = [int(k) for k in start]

for i in range(100):
    result = []
    count = 0

    counts = [0 for _ in range(10)]
    for c in start:
        counts[c] += 1

    for j in range(10):
        if counts[j] != 0:
            for c in str(counts[j]):
                result.append(int(c))

            result.append(j)

    pppp = "".join([str(w) for w in result])
    if pppp in ddddd:
        print("Does not appear")
        exit()

    ddddd.add(pppp)
    if pppp == target:
        print(i+2)

        exit()

    start = result

print("I'm bored")