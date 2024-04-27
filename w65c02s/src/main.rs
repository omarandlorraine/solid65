extern crate w65c02s;

use w65c02s::*;

struct Dummy;

// Implements System but doesn't log anything, so we can let the CPU reset before single-stepping
// the instruction.
impl System for Dummy {
    fn read(&mut self, _cpu: &mut W65C02S, _addr: u16) -> u8 {
        0
    }

    fn write(&mut self, _cpu: &mut W65C02S, _addr: u16, _value: u8) {
    }
}


struct Memory {
    vals: Vec<u8>,
    counter: usize,
}

impl System for Memory {
    fn read(&mut self, _cpu: &mut W65C02S, addr: u16) -> u8 {
        let a = addr & 0x07;
        let r = self.vals[a as usize];
        self.counter += 1;
        println!("r {} {:#06x} {:#04x} 0x00 0x00 0x00 0x00 0x00 0x0000", self.counter, addr, r);
        return r;
    }

    fn write(&mut self, _cpu: &mut W65C02S, addr: u16, value: u8) {
        self.counter += 1;
        println!("w {} {:#06x} {:#04x} 0x00 0x00 0x00 0x00 0x00 0x0000", self.counter, addr, value);
    }
}

impl Memory {
    fn new(vals: Vec<u8>) -> Self {
        Self {
            vals,
            counter: 0
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().into_iter().skip(1).map(|string| u8::from_str_radix(&string, 16).unwrap()).collect();
    let mut memory = Memory::new(args[6..].into());
    let mut cpu = W65C02S::new();
    cpu.set_a( args[1]);
    cpu.set_x( args[2]);
    cpu.set_y( args[3]);
    cpu.set_pc( Into::<u16>::into(args[0]));

    // TODO: Once this is confirmed working, we can remove the FIXME notice in the library's implementation
    // of the PHP opcode.
    cpu.step(&mut Dummy); // let the CPU reset; this does not log anything.
    cpu.set_p(args[4].into());
    cpu.set_s(args[5]);
    cpu.set_pc(Into::<u16>::into(args[0]));
    println!("b {} 0x0000 0x00 {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#06x}", memory.counter, cpu.get_a(), cpu.get_x(), cpu.get_y(), cpu.get_s(), cpu.get_p() & 0xcf, cpu.get_pc());
    cpu.step(&mut memory);

    println!("a {} 0x0000 0x00 {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#06x}", memory.counter + 1, cpu.get_a(), cpu.get_x(), cpu.get_y(), cpu.get_s(), cpu.get_p() & 0xcf, cpu.get_pc());
}
