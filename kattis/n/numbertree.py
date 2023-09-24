h=input().split()+[""]
k=1
[k:=2*k+(c=='R')for c in h[1]]
print((1<<int(h[0])+1)-k)