for i in range(int(input())):
    (a,b)=map(int,input().split())
    print((pow(a,(b-1)//2,b)<2)*"yes"or"no")