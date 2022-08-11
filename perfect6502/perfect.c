#include "../solid65.h"
#include <stdint.h>
#include <stdlib.h>
#include <stddef.h>
#include "perfect6502/perfect6502.h"

uint8_t input[TCLEN];

int main(int argc, char * argv[]) {
	for(int i = 0; i < TCLEN; i++) {
		input[i] = strtoul(argv[i], NULL, 16);
	}
}
