from math import*
while(n:=int(input())):
 print(e**(lgamma(n+1)-log(tau*n)/2-n*log(n)+n))