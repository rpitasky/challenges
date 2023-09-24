(k,a,b)=map(int,input().split())
for i in " "*k:
    p=int(input())
    print(["DA","NE"][a**2+b**2<p**2])