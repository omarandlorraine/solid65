
// Set the CPU's internal state to these values before starting the test
#define B_PCH 0
#define B_PCL 1
#define B_A   2
#define B_X   3
#define B_Y   4
#define B_P   5
#define B_S   6

// Any time your emulated CPU reads a value, supply the values from this locus on.
#define PROG  7

// The highest number of reads a single instruction can do is 8.
#define TCLEN (PROG+8)
