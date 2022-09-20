#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <dirent.h>
#include <unistd.h>
#include <errno.h>

#define RNG "/dev/random"

#include "solid65.h"
#include "opcodes.h"

int valid_opcode(int opcode) {
	opcode %= (sizeof(legals) / sizeof(legals[0]));
	return legals[opcode];
}

int main() {

	char testcase[TCLEN];

	// Open the random number generator
	FILE * rng = fopen(RNG, "r");
	if(!rng) {
		perror("couldn't open " RNG);
		exit(1);
	}

	// Read in the test case
	for(int i = 0; i < TCLEN; i++) {
		testcase[i] = fgetc(rng);
	}

	// Make sure that the opcode is a valid one
	testcase[PROG] = valid_opcode(fgetc(rng));

	// This program doesn't make any claims about what the result should be, so
	// clear out the result fields
	testcase[A_PCH] = 0;
	testcase[A_PCL] = 0;
	testcase[A_A] = 0;
	testcase[A_X] = 0;
	testcase[A_Y] = 0;
	testcase[A_P] = 0;
	testcase[A_S] = 0;

	// close the RNG
	fclose(rng);

	// Print out the generated test case
	for(int i = 0; i < TCLEN; i++) {
		printf("%02x ", (unsigned int) (testcase[i] & 0xff));
	}
	printf("\n");
}
