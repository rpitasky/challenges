cards = []
for i in range(4):
    cards.extend(input().split(' '))

hand = []

while len(cards) != 0:
    hand.append(cards[0])
    cards.remove(cards[0])
    while True:
        removals = []
        found = False
        for i in range(len(hand) - 3):
            # print('---')
            # print(i)
            # print(len(hand))
            card1 = hand[i]
            card2 = hand[i + 3]
            if card1[1] == card2[1]:
                removals.append([i, i + 3])
            if card1[0] == card2[0]:
                removals.append([i, i + 1, i + 2, i + 3])
        r2 = None
        r4 = None
        for r in removals:
            if len(r) == 2:
                if r2 is None or r[0] > r2[0]:
                    r2 = r
            if len(r) == 4:
                if r4 is None or r[0] > r4[0]:
                    r4 = r
        r = None
        if r2 is not None: r = r2
        if r4 is not None: r = r4
        if r is not None:
            found = True
            cs = [hand[j] for j in r]
            for j in cs:
                hand.remove(j)
             #break
        if not found:
            break

print(len(hand), ' '.join(hand))
