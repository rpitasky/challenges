l=lambda:int(input())
k=l()
print(k+sum([k-l()for i in"a"*l()]))