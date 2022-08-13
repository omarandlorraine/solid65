#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <dirent.h>
#include <unistd.h>
#include <errno.h>

#define RNG "/dev/random"

int main() {
	FILE * rng = fopen(RNG, "r");
	if(!rng) {
		perror("couldn't open " RNG);
		exit(1);
	}

	
}
