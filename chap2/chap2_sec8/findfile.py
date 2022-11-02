import sys
import os


def main():
    if len(sys.argv) < 3:
        print("findfile.py (path) (keyword)")
        quit()
    target_dir = sys.argv[1]
    keyword = sys.argv[2]

    for dirname, _, files in os.walk(target_dir):
        for file in files:
            if keyword in file:
                fullpath = os.path.join(dirname, file)
                print(fullpath)


if __name__ == "__main__":
    main()
