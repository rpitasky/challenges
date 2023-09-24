d=[0]*31
d[0]=1
d[2]=3
while 1:
	n=int(input())
	n+1or exit()
	if not(n%2or d[n]):
		for i in range(4,n+1):
			d[i]=4*d[i-2]-d[i-4]
	print(d[n])