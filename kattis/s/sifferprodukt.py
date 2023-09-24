s=input()
while len(s)>1:
    p=1
    for i in map(int,s):
        if i!=0:
            p*=i
    s=str(p)
print(s)