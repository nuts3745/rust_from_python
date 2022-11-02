AFILE = "./fizzbuzz_python.txt"
BFILE = "./fizzbuzz_rust.txt"


def main():
    with open(AFILE, "r") as afile:
        astr = afile.read()
    with open(BFILE, "r") as bfile:
        bstr = bfile.read()
    astr = astr.strip()
    bstr = bstr.strip()

    if astr == bstr:
        print("OK")
    else:
        print("NG")


if __name__ == "__main__":
    main()
