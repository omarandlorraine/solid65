
// Set the CPU's internal state to these values before starting the test
#define B_PCH 0
#define B_PCL 1
#define B_A   2
#define B_X   3
#define B_Y   4
#define B_P   5
#define B_S   6
// Compare the CPU against these values after the test has run
#define A_PCH 7
#define A_PCL 8
#define A_A   9
#define A_X   10
#define A_Y   11
#define A_P   12
#define A_S   13

// Any time your emulated CPU reads a value, supply the values from this locus on.
#define PROG  14

// The highest number of reads a single instruction can do is 8.
#define TCLEN (PROG+8)
