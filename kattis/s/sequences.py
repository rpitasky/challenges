o=0
q=1
i=0
k=10**9+7
for b in input():
 if b=='1':
  o=(o+q)%k
 elif b=='0':
  i=(i+o)%k
 else:
  i=(2*i+o)%k
  o=(2*o+q)%k
  q=2*q%k
print(i)