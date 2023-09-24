input()
k=list(filter(lambda a:a>=0,map(int,input().split())))
print(sum(k)/len(k))