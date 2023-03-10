for a in"a"*int(input()):
 i=input().lower()
 n=''.join(filter(lambda q:q not in i,map(chr,range(97,123))))
 print("missing "+n if n else"pangram")