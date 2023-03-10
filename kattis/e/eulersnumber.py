k=1
n=1
for i in range(int(input())):
    k*=i+1
    n+=1/k
print(n)