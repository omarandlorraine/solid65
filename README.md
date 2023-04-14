# solid65
compare emulators against each other

This project connects several emulators of the 6502 CPU and tests them against
each other.

## How to use it

The basic idea here is to initialize the CPU to some random state, and then
execute a single, randomly generated, instruction. If we do this the same way on
several emulators, we can see if they agree or disagree. If they disagree, then
we maybe found a bug and a test case.

But first, of course, we need to clone the repository. This step downloads
solid65 itself, and (some of) the emulators it tests. To do this using git:

```
git clone --recursive https://github.com/omarandlorraine/solid65.git
```

You may of course get it from some other repository, but don't forget
`--recursive`.

Inside of the project directory, you can use `./run_test` to generate a random
test case and run it. The test case appears under the `tests/` directory, and
is a directory containing the results of the test. Each tested emulator has a
corresponding file under this directory, detailing the initial state, and each
memory access the emulator does during the course of the execution of the
random instruction.

To analyse the test results, you could make use of the `compare.py` script. Its
arguments are a path to the test, and the names of two emulators you want to
compare, and its output is a list of disagreements it found.

For example, run the following command line to see how the rubbermallet
emulator compares with mre's in their execution of the RTS instruction:

```
./compare.py tests/rts/ rubbermallet mre_mos6502
```


### gianluca/mos6502

[link to repository](https://github.com/gianlucag/mos6502)

Known issues with the tester:
- It doesn't check that the number of cycles matches the number of reads and writes

### fake6502

[link to sourcefile](http://rubbermallet.org/fake6502.c)

### mos6502

An emulator written in Rust.

Known issues with the tester:
- It does not log the internal state of the CPU with each bus cycle.

[link to repository](https://github.com/mre/mos6502)
