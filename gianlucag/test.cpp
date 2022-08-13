
#include "mos6502.h"
#include "../solid65.h"

using namespace std;

uint8_t testcase[TCLEN];
uint8_t reads[8];

void log(char rw, uint16_t addr, uint8_t val, mos6502 * cpu) {
}

void write(uint16_t addr, uint8_t val) {
}

uint8_t read(uint16_t addr) {
	static size_t read_count = 0;
	uint8_t val = reads[read_count++];
	return val;
}

mos6502 cpu = mos6502(read, write);

int main(int argc, char *argv[]) {
	for(int i = 1; i < TCLEN; i++) {
		testcase[i] = strtoul(argv[i], NULL, 16);
	}
	cpu.SetResetP(testcase[B_P]);
	cpu.SetResetA(testcase[B_A]);
	cpu.SetResetX(testcase[B_X]);
	cpu.SetResetY(testcase[B_Y]);
	cpu.SetResetS(testcase[B_S]);

}
