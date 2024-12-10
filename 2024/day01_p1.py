l1 = []
l2 = []

with open("day01_p1_input", "r") as f:
    for line in f:
        line = line.strip()
        numbs = line.split(" ")
        a,b = numbs[0], numbs[3]
        l1.append(int(a))
        l2.append(int(b))

l1.sort()
l2.sort()

print("part 1 answer:")
print(sum(map(lambda x: abs(x[1] - x[0]), zip(l1,l2))))

s = 0
for i in l1:
    s += i * sum([1 for x in l2 if x == i])

print("part 2 answer:")
print(s)

