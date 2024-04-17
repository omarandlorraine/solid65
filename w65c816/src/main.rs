extern crate w65c816;

struct Memory {
    vals: Vec<u8>,
    counter: u32,
    opcodes: u32,
    reset: bool,
}

impl Memory {
    fn new(vals: Vec<u8>) -> Self {
        Self {
            vals,
            counter: 0,
                opcodes: 0,
                reset: true,
        }
    }
}

impl w65c816::System for Memory {
    fn read(&mut self, addr: u32, addr_type: w65c816::AddressType, _signals: &w65c816::Signals) -> u8 {
        let a = addr & 0x07;
        let r = self.vals[a as usize];
        self.counter += 1;
        if addr_type == w65c816::AddressType::Opcode {
            self.opcodes += 1;
        }

        if self.opcodes < 2 {
            println!("r {} {:#06x} {:#04x} 0x00 0x00 0x00 0x00 0x00 0x0000", self.counter, addr & 0xffff, r);
        }
        return r;
    }

    fn write(&mut self, addr: u32, r: u8, _addr_type: w65c816::AddressType, _signals: &w65c816::Signals) {
        self.counter += 1;
        println!("w {} {:#06x} {:#04x} 0x00 0x00 0x00 0x00 0x00 0x0000", self.counter, addr, r);
    }

    fn res(&mut self) -> bool {
        let r = self.reset;
        self.reset = false;
        r
    }
}

fn main() {
    let args: Vec<_> = std::env::args().into_iter().skip(1).map(|string| u8::from_str_radix(&string, 16).unwrap()).collect();
    let mut memory = Memory::new(args[6..].into());
    let mut mos6502 : w65c816::CPU = Default::default();
    mos6502.set_a(args[1]);
    mos6502.set_x(args[2].into());
    mos6502.set_y(args[3].into());
    mos6502.set_s(args[5].into());
    mos6502.set_pc(args[0].into());

    // TODO: Once this is confirmed working, we can remove the FIXME notice in the library's implementation
    // of the PHP opcode.
    mos6502.set_p(args[4].into());
    println!("b 0 0x0000 0x00 {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#06x}", mos6502.a(), mos6502.x(), mos6502.y(), mos6502.s() & 0xff, mos6502.p() & 0xcf, mos6502.pc());
    while memory.opcodes < 2 {
        mos6502.cycle(&mut memory);
    }

    println!("a {} 0x0000 0x00 {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#06x}", mos6502.tcu(),  mos6502.a(), mos6502.x(), mos6502.y(), mos6502.s() & 0xff, mos6502.p() & 0xcf, mos6502.pc());
    
}
