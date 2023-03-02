dna = input()
m = int(input())
s = []
e = []
for i in range(m):
    l = input().split(' ')
    s.append(int(l[0]) - 1)
    e.append(int(l[1]) - 1)
a = [0]
c = [0]
g = [0]
t = [0]
at = 0
ct = 0
gt = 0
tt = 0
for ch in dna:
    if ch == 'A':
        at += 1
    if ch == 'C':
        ct += 1
    if ch == 'G':
        gt += 1
    if ch == 'T':
        tt += 1
    a.append(at)
    c.append(ct)
    g.append(gt)
    t.append(tt)
for i in range(m):
    ac = a[min(e[i]+1, len(dna))] - a[s[i]]
    cc = c[min(e[i]+1, len(dna))] - c[s[i]]
    gc = g[min(e[i]+1, len(dna))] - g[s[i]]
    tc = t[min(e[i]+1, len(dna))] - t[s[i]]
    ans = ''
    # Need to print a permutation of ACGT, in the order from most to least
    # of ac, cc, gc, tc, and tiebreakers are in the order of A T G C
    k = list(((ac, 3), (tc, 2), (gc, 1), (cc, 0)))
    k.sort()
    k.reverse()
    ans = ''
    for (e1, p) in k:
        ans += "CGTA"[p]
    print(ans)

