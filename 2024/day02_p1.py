from itertools import pairwise
safe = 0
with open("day02_p1_input", "r") as file:
    for report in file:
        report = report.strip().split(" ")
        report = [int(x) for x in report]

        pairs = list(pairwise(report))
        diffs = list(map(lambda x: x[0] - x[1], pairs))
        directions = list(map(lambda x: 1 if x >= 0 else -1, diffs))
        dirpairs = list(pairwise(directions))

        if (
            all([abs(x) >= 1 and abs(x) <= 3 for x in diffs])
            and
            all([x[0] == x[1] for x in dirpairs])
        ):
            safe += 1

print(safe)


