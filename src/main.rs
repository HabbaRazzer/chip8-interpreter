struct System {
    memory: [u8; 4096],
    registers: [u8; 16],
    pc: u16,
    index: u16
}

impl System {
    fn new() -> Self {
        System {
            memory: [0; 4096],
            registers: [0; 16],
            pc: 0,
            index: 0
        }
    }
}

fn main() {
    let system = System::new();
    println!("{}, {}, {}, {}", system.memory[0], system.registers[0],
        system.pc, system.index);
}
