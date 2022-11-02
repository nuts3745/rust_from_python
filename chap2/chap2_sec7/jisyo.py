import sys

DICFILE = "ejdict-hand-utf8.txt"


def main():
    if len(sys.argv) < 2:
        print("[USAGE] jisyo.py WORD")
        quit()
    word = sys.argv[1]

    with open(DICFILE, "rt", encoding="utf-8") as fp:
        while True:
            line = fp.readline()
            if not line:
                break
            if word in line:
                print(line.strip())


if __name__ == "__main__":
    main()
