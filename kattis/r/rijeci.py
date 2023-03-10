a=1
b=0
for i in range(int(input())):
    n=a
    a=b
    b=b+n
print(a,b)