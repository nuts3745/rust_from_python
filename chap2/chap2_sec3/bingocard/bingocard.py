import random

BINGO_NUMBER = 75
nums = list(range(1, BINGO_NUMBER + 1))
random.shuffle(nums)

for y in range(0, 5):
    for x in range(0, 5):
        if y * 5 + x == 12:
            print("  *,", end="")
        else:
            print("{:>3},".format(nums[y * 5 + x]), end="")
    print("")
