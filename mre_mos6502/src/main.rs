extern crate mos6502;

use mos6502::cpu;
use mos6502::memory::Bus;
use mos6502::registers::StackPointer;
use mos6502::registers::Status;


struct Memory {
    vals: Vec<u8>,
    counter: usize,
}

impl Memory {
    fn new(vals: Vec<u8>) -> Self {
        Self {
            vals,
            counter: 0
        }
    }
}

impl Bus for Memory {
    fn get_byte(&mut self, addr: u16) -> u8 {
        let r = self.vals[self.counter];
        println!("r {} {:#06x} {:#04x} 0x00 0x00 0x00 0x00 0x00 0x0000", (self.counter / 2) + 1, addr, r);
        self.counter += 2;
        return r;
    }

    fn set_byte(&mut self, addr: u16, r: u8) {
        println!("w {} {:#06x} {:#04x} 0x00 0x00 0x00 0x00 0x00 0x0000", (self.counter / 2) + 1, addr, r);
        self.counter += 2;
    }
}

fn main() {
    let args: Vec<_> = std::env::args().into_iter().skip(1).map(|string| u8::from_str_radix(&string, 16).unwrap()).collect();
    let memory = Memory::new(args[7..].into());
    let mut mos6502 = cpu::CPU::new(memory);
    mos6502.registers.accumulator = args[1] as i8;
    mos6502.registers.index_x = args[2];
    mos6502.registers.index_y = args[3];
    mos6502.registers.stack_pointer = StackPointer(args[5]);
    mos6502.registers.program_counter = Into::<u16>::into(args[0]);

    // TODO: Once this is confirmed working, we can remove the FIXME notice in the library's implementation
    // of the PHP opcode.
    mos6502.registers.status = Status::from_bits_truncate(args[4].into());
    println!("b {} 0x0000 0x00 {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#06x}", mos6502.memory.counter, mos6502.registers.accumulator, mos6502.registers.index_x, mos6502.registers.index_y, mos6502.registers.stack_pointer.0, mos6502.registers.status.bits() & 0xcf, mos6502.registers.program_counter);
    mos6502.single_step();

    println!("a {} 0x0000 0x00 {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#06x}", mos6502.memory.counter, mos6502.registers.accumulator, mos6502.registers.index_x, mos6502.registers.index_y, mos6502.registers.stack_pointer.0, mos6502.registers.status.bits() & 0xcf, mos6502.registers.program_counter)
    
}
