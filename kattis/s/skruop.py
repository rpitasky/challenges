v=7
for i in range(int(input())):
    v=max(0,min(10,v-1+2*(input()[-2]=="p")))
print(v)