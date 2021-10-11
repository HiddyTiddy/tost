"""
formatter yeah ill riir
"""

text = input()

INDENT = 0
SKIP = False
for i in text:
    if SKIP :
        SKIP = False
        continue
    print(i, end="")
    if i == "[":
        INDENT += 4
        print()
        print(" "*INDENT, end="")
    if i == "]":
        INDENT -= 4
    if i == ",":
        print()
        print(" "*INDENT, end="")
        SKIP = True

print()
