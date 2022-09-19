#include <stdio.h>
#include <stdint.h>
#include "../solid65.h"

uint8_t testcase[TCLEN];

//6502 CPU registers
extern uint16_t pc;
extern uint8_t sp;
extern uint8_t a;
extern uint8_t x;
extern uint8_t y;
extern uint8_t status;

void log(char rw, uint16_t addr, uint8_t val) {
    static int cycles = 0;
    printf("%c %u ", rw, cycles);
    printf("%04x %02x ", addr, val);
    printf("%02x ", a);
    printf("%02x ", x);
    printf("%02x ", y);
    printf("%02x ", sp);
    printf("%02x ", status & 0xdf);
    printf("%04x ", pc);
    printf("\n");
    cycles++;
}

void write6502(uint16_t addr, uint8_t val) {
    log('w', addr, val);
}

uint8_t read6502(uint16_t addr) {
    static int read_count = -2;
	uint8_t val;
    switch (read_count) {
        case -2:
            read_count++;
            val = testcase[B_PCL];
            log('r', addr, val);
            return val;
        case -1:
            read_count++;
            val = testcase[B_PCH];
            log('r', addr, val);
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

    reset6502();
    status = (testcase[B_P]);
    a = (testcase[B_A]);
    x = (testcase[B_X]);
    y = (testcase[B_Y]);
    sp = (testcase[B_S]);
	step6502();
    log('a', 0, 0);
}
