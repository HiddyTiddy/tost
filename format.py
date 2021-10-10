text = input()

indent = 0
skip_sp = False
for i in text:
    if skip_sp :
        skip_sp = False
        continue
    print(i, end="")
    if i == "[":
        indent += 4
        print()
        print(" "*indent, end="")
    if i == "]":
        indent -= 4
    if i == ",":
        print()
        print(" "*indent, end="")
        skip_sp = True
        
print()
