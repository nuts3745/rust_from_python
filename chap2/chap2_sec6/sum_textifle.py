import sys


def main():
    total = 0
    for i, v in enumerate(sys.argv):
        if i == 0:
            continue
        with open(v, "rt") as f:
            text = f.read()
            for line in text.split("\n"):
                try:
                    total += float(line)
                except ValueError:
                    pass
    print(total)


if __name__ == "__main__":
    main()
