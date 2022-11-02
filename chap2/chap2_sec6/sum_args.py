import sys

total = 0

for i, v in enumerate(sys.argv):
    if i == 0:
        continue
    try:
        total += float(v)
    except ValueError:
        print("{}は数値ではないため加算されません".format(v))
        pass
print(total)
