from collections import Counter
from timeit import timeit

list_1 = []
list_2 = []

with open("../data/day_1", "r") as f:
    for line in f.readlines():
        split = line.split("   ")
        list_1.append(int(split[0].strip()))
        list_2.append(int(split[1].strip()))

list_1.sort()
list_2.sort()


def get_total_distance() -> int:
    distance = []
    for i in range(0, len(list_1)):
        distance.append(abs(list_1[i] - list_2[i]))

    return sum(distance)


def get_similarity() -> int:
    similarity = 0
    occurances = Counter(list_2)
    for value in list_1:
        similarity += value * occurances[value]

    return similarity


def benchmark():
    get_total_distance()
    get_similarity()

    assert get_total_distance() == 1530215
    assert get_similarity() == 26800609


result = timeit("benchmark()", setup="from __main__ import benchmark", number=10000)

avg = result / 10000

print(f"Time: {avg:.10f}")
