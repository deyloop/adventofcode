with open("./input", "r") as input:
    lines = input.readlines()

lines = [l.strip() for l in lines]

elves = []
cur_elve = 0
for l in lines:
    if not l == "":
        cur_elve += int(l)
    else:
        elves.append(cur_elve)
        cur_elve = 0

print(max(elves))

