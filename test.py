#!/usr/bin/python
import sys
import random
import subprocess


def random_byte() -> int:
    return random.randint(0, 255)


def random_opcode() -> int:
    return random.choice([ 0x01, 0x05, 0x06, 0x08, 0x09, 0x0a, 0x0d, 0x0e, 0x10, 0x11, 0x15, 0x16, 0x18, 0x19, 0x1d, 0x1e, 0x20, 0x21, 0x24, 0x25, 0x26, 0x28, 0x29, 0x2a, 0x2c, 0x2d, 0x2e, 0x30, 0x31, 0x35, 0x36, 0x38, 0x39, 0x3d, 0x3e, 0x40, 0x41, 0x45, 0x46, 0x48, 0x49, 0x4a, 0x4c, 0x4d, 0x4e, 0x50, 0x51, 0x55, 0x56, 0x58, 0x59, 0x5d, 0x5e, 0x60, 0x61, 0x65, 0x66, 0x68, 0x69, 0x6a, 0x6c, 0x6d, 0x6e, 0x70, 0x71, 0x75, 0x76, 0x78, 0x79, 0x7d, 0x7e, 0x81, 0x84, 0x85, 0x86, 0x88, 0x8a, 0x8c, 0x8d, 0x8e, 0x90, 0x91, 0x94, 0x95, 0x96, 0x98, 0x99, 0x9a, 0x9d, 0xa0, 0xa1, 0xa2, 0xa4, 0xa5, 0xa6, 0xa8, 0xa9, 0xaa, 0xac, 0xad, 0xae, 0xb0, 0xb1, 0xb4, 0xb5, 0xb6, 0xb8, 0xb9, 0xba, 0xbc, 0xbd, 0xbe, 0xc0, 0xc1, 0xc4, 0xc5, 0xc6, 0xc8, 0xc9, 0xca, 0xcc, 0xcd, 0xce, 0xd0, 0xd1, 0xd5, 0xd6, 0xd8, 0xd9, 0xdd, 0xde, 0xe0, 0xe1, 0xe4, 0xe5, 0xe6, 0xe8, 0xe9, 0xea, 0xec, 0xed, 0xee, 0xf0, 0xf1, 0xf5, 0xf6, 0xf8, 0xf9, 0xfd, 0xfe ])


def random_testcase():
    test_case = [random_byte() for _ in range(0, 15)]

    # examine which byte the program counter is pointing to and make sure it's a valid opcode.
    pc = test_case[0]
    pc &= 0x7
    test_case[pc + 6] = random_opcode()

    return test_case


class Cycle:
    def __init__(self, line):
        fields = line.split()
        try:
            (self.type, cycle_number, self.addr_bus, self.data_bus, self.a, self.x, self.y, self.sp, self.status, self.pc) = fields
        except ValueError:
            print("Couldn't unpack " + str(fields))

        self.cycle_number = int(cycle_number)

    def internal_state(self):
        return (self.a, self.x, self.y, self.sp, self.status, self.pc)

		
class Cycles:
    def __init__(self, filename, file):
        self.filename = filename
        self.cycles = []
        for line in file:
            self.cycles.append(Cycle(line))


    def cycle_types(self):
        return "".join([c.type for c in self.cycles])

    def addresses(self):
        return [c.addr_bus for c in self.cycles]

    def before(self):
        return self.cycles[0]

    def after(self):
        return self.cycles[-1]


class Emulator:
    def __init__(self, name):
        self.name = name

class CargoEmulator(Emulator):
    """an emulator that's built an run using Cargo"""
    def __init__(self, name):
        super().__init__(name)
        self.cycle_accurate = False

    def init(self):
        subprocess.run(["cargo", "update"], cwd=self.name)
        subprocess.run(["cargo", "build"], cwd=self.name)

    def test(self, test_case):
        cycles = []
        cmd = ["cargo", "run", "-q", "--"] + [f'{c:02x}' for c in test_case]
        proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, cwd=f"./{self.name}/")
        while True:
            line = proc.stdout.readline()
            if not line:
                break
            cycles += [Cycle(line.decode())]

        return cycles
        

class MakeEmulator(Emulator):
    """an emulator that's built with a Makefile"""
    def __init__(self, name):
        super().__init__(name)
        self.cycle_accurate = True

    def init(self):
        subprocess.run("make", shell=True, cwd=self.name)

    def test(self, test_case):
        cycles = []
        cmd = ["./test"] + [hex(c)[2:] for c in test_case]
        process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=self.name)
        for line in process.stdout:
            cycles += [Cycle(line.decode())]

        return cycles

mre_mos6502 = CargoEmulator("mre_mos6502")
w65c816 = CargoEmulator("w65c816")
rubbermallet = MakeEmulator("rubbermallet")
gianlucag = MakeEmulator("gianlucag")

all_emulators = [mre_mos6502, w65c816, rubbermallet, gianlucag]
all_emulators = [mre_mos6502, rubbermallet, gianlucag]
		
def test(a, b):
    a_name = a[0].name
    b_name = b[0].name

    failed = False

    if not b[1]:
        print(f"The {b[0].name} emulator didn't output anything here")
        failed = True

    if a[1][0].internal_state() != b[1][0].internal_state():
        print(f"The initial states differ between {a_name} and {b_name}; not bothering to test anything else on {b_name}")
        print(str(a[1][0].internal_state()))
        print(str(b[1][0].internal_state()))
        failed = True

    for reg in zip(("a", "x", "y", "stack pointer", "flags", "pc"),  a[1][-1].internal_state(), b[1][-1].internal_state()):
        if reg[1] != reg[2]:
            print(f"The instruction results in a different {reg[0]} on {a_name} and {b_name}; this is a bug")
            print(reg)
            failed = True

    for (count, cycle) in enumerate(b[1], start=0):
        if cycle.cycle_number != count:
            print(f"{b_name} does not have monotonically increasing cycle counts, (see column 2).")
            failed = True

    if b[0].cycle_accurate:
        types = zip([cycle.type for cycle in a[1]], [cycle.type for cycle in b[1]])
        for (count, typ) in enumerate(types):
            if typ[0] != typ[1]:
                print("the cycle types don't match (read/write/before/after, see column 1): %s" % str(typ))
                failed = True

        addresses = zip([cycle.addr_bus for cycle in a[1]], [cycle.addr_bus for cycle in b[1]])
        for (count, addrs) in enumerate(addresses):
            if addrs[0] != addrs[1]:
                print(f"in cycle {cycle.cycle_number}, {a_name} accesses {addrs[0]} but {b_name} accesses {addrs[1]}")
                failed = True

    return failed

for emu in all_emulators:
    emu.init()

def run_test(test_case, emus=all_emulators):
    print(" ".join([f'{c:02x}' for c in test_case]))
    results = list([(emu, emu.test(test_case)) for emu in all_emulators])
    
    for b in results:
        if test(results[0], b):
            return True
    return False

while True:
    tc = random_testcase()
    if run_test(tc):
        print(f"A new regression test: {tc}")
        sys.exit(1)

for regression_test in [[93, 105, 158, 107, 178, 221, 173, 186, 203, 41, 228, 237, 9, 72, 194]]:
    if run_test(regression_test):
        print(f"A regression test failed!")
        sys.exit(1)
