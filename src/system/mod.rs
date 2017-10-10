const NUM_BYTES: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 48;

pub struct System {
    memory: [u8; NUM_BYTES],
    registers: [u8; NUM_REGISTERS],
    pc: u16,
    index: u16,
    stack: [u8; STACK_SIZE],
    sp: u8
}

impl System {
    pub fn new() -> Self {
        System {
            memory: [0; NUM_BYTES],
            registers: [0; NUM_REGISTERS],
            pc: 0,
            index: 0,
            stack: [0; STACK_SIZE],
            sp: 0
        }
    }
}
