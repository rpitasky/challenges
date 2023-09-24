import re
a=re.sub
print(len(a(" {4}|\w","",a("[A-Z]","\ta",input()).expandtabs(4))))