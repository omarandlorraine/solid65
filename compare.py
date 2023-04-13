#!/usr/bin/python
import sys

class Cycle:
    def __init__(self, line):
        fields = line.split()
        try:
            (self.type, cycle_number, self.addr_bus, self.data_bus, self.a, self.x, self.y, self.sp, self.status, self.pc) = fields
        except ValueError:
            print("Couldn't unpack " + str(fields))

        self.cycle_number = int(cycle_number)
		
class Cycles:
    def __init__(self, filename, file):
        self.filename = filename
        self.cycles = []
        for line in file:
            self.cycles.append(Cycle(line))

    def check_monotonic_cycle_count_increment(self):
        for (count, cycle) in enumerate(self.cycles, start=0):
            if cycle.cycle_number != count:
                return True
        return False

    def cycle_types(self):
        return "".join([c.type for c in self.cycles])

    def addresses(self):
        return [c.addr_bus for c in self.cycles]

file_a = sys.argv[1] + sys.argv[2] + ".results"
file_b = sys.argv[1] + sys.argv[3] + ".results"

def test(a, b):
    for cyc in [a, b]:
        if cyc.check_monotonic_cycle_count_increment():
            print("%s does not have monotonically increasing cycle counts, (see column 2)." % cyc.filename)

    cycle_types = a.cycle_types(), b.cycle_types()
    if cycle_types[0] != cycle_types[1]:
        print("the cycle types don't match (read/write/before/after, see column 1): %s" % str(cycle_types))

    addresses = zip(a.addresses(), b.addresses())
    for (count, addrs) in enumerate(addresses):
        if addrs[0] != addrs[1]:
            print("in cycle %d, %s accesses %s but %s accesses %s" % (count, a.filename, addrs[0], b.filename, addrs[1]))


with open(file_a) as a:
    cycles_a = Cycles(sys.argv[2], a)
    with open(file_b) as b:
        cycles_b = Cycles(sys.argv[3], b)
        test(cycles_a, cycles_b)
		
