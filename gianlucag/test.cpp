
#include "mos6502/mos6502.h"
#include "../solid65.h"

using namespace std;

uint8_t testcase[TCLEN];

uint8_t read(uint16_t addr);
void write(uint16_t addr, uint8_t val);
mos6502 cpu = mos6502(read, write);

void log(char rw, uint16_t addr, uint8_t val) {
	static int cycles = 0;
	printf("%c %u ", rw, cycles);
	printf("0x%04x 0x%02x ", addr, val);
	printf("0x%02x ", cpu.GetA());
	printf("0x%02x ", cpu.GetX());
	printf("0x%02x ", cpu.GetY());
	printf("0x%02x ", cpu.GetS());
	printf("0x%02x ", cpu.GetP() & 0xcf);
	printf("0x%04x ", cpu.GetPC());
	printf("\n");
	cycles++;
}

void write(uint16_t addr, uint8_t val) {
	log('w', addr, val);
}

uint8_t read(uint16_t addr) {
	static int read_count = -2;
	uint8_t val;
	switch (read_count) {
		case -2:
			read_count++;
			val = testcase[B_PCH];
			return val;
		case -1:
			read_count++;
			val = testcase[B_PCL];
			return val;
		default:
			read_count++;
			val = testcase[PROG + read_count++];
			log('r', addr, val);
			return val;
	}
}

int main(int argc, char *argv[]) {
	uint64_t cycles;

	for(int i = 1; i < TCLEN; i++) {
		testcase[i] = strtoul(argv[i], NULL, 16);
	}
	log('b', 0, 0);

	cpu.SetResetP(testcase[B_P]);
	cpu.SetResetA(testcase[B_A]);
	cpu.SetResetX(testcase[B_X]);
	cpu.SetResetY(testcase[B_Y]);
	cpu.SetResetS(testcase[B_S]);
	cpu.Reset();
	cpu.Run(1, cycles);
	log('a', 0, 0);
}
