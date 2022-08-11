all: perfect
perfect: deps perfect.o
	$(CC) perfect.o -o perfect perfect6502/perfect6502.o perfect6502/netlist_sim.o

.PHONY: deps
deps:
	make -C perfect6502 perfect6502.o netlist_sim.o
