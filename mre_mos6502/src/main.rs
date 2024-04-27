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
        let a = addr & 0x07;
        let r = self.vals[a as usize];
        self.counter += 1;
        println!("r {} {:#06x} {:#04x} 0x00 0x00 0x00 0x00 0x00 0x0000", self.counter, addr, r);
        return r;
    }

    fn set_byte(&mut self, addr: u16, r: u8) {
        self.counter += 1;
        println!("w {} {:#06x} {:#04x} 0x00 0x00 0x00 0x00 0x00 0x0000", self.counter, addr, r);
    }
}

fn main() {
    let args: Vec<_> = std::env::args().into_iter().skip(1).map(|string| u8::from_str_radix(&string, 16).unwrap()).collect();
    let memory = Memory::new(args[6..].into());
    #[cfg(feature = "cmos")]
    let mut mos6502 = cpu::CPU::new(memory, mos6502::instruction::Cmos6502);
    #[cfg(not(feature = "cmos"))]
    let mut mos6502 = cpu::CPU::new(memory, mos6502::instruction::Nmos6502);
    mos6502.registers.accumulator = args[1];
    mos6502.registers.index_x = args[2];
    mos6502.registers.index_y = args[3];
    mos6502.registers.stack_pointer = StackPointer(args[5]);
    mos6502.registers.program_counter = Into::<u16>::into(args[0]);

    // TODO: Once this is confirmed working, we can remove the FIXME notice in the library's implementation
    // of the PHP opcode.
    mos6502.registers.status = Status::from_bits_truncate(args[4].into());
    println!("b {} 0x0000 0x00 {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#06x}", mos6502.memory.counter, mos6502.registers.accumulator, mos6502.registers.index_x, mos6502.registers.index_y, mos6502.registers.stack_pointer.0, mos6502.registers.status.bits() & 0xcf, mos6502.registers.program_counter);
    mos6502.single_step();

    println!("a {} 0x0000 0x00 {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#06x}", mos6502.memory.counter + 1, mos6502.registers.accumulator, mos6502.registers.index_x, mos6502.registers.index_y, mos6502.registers.stack_pointer.0, mos6502.registers.status.bits() & 0xcf, mos6502.registers.program_counter)
    
}
