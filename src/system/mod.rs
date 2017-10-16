mod opcode;

pub type Word = u16;
pub type Byte = u8;
pub type RegisterAddress = usize;

const NUM_BYTES: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 48;

#[allow(dead_code)]
pub struct System {
    memory: [Byte; NUM_BYTES],
    registers: [Byte; NUM_REGISTERS],
    pc: Word,
    index: Word,
    stack: [Byte; STACK_SIZE],
    sp: Byte,
}

#[allow(dead_code)]
impl System{
    pub fn new() -> Self {
        System {
            memory: [0; NUM_BYTES],
            registers: [0; NUM_REGISTERS],
            pc: 0,
            index: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    /** Index into system memory at address and return word located there. */
    pub fn read_word(&self, address: usize) -> Word {
        let left = self.memory[address] as Word;
        let right = self.memory[address + 1] as Word;
        (left << 8) | right
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /** Load some test data into the system memory for the purposes of testing. */
    fn load_test_data(system: &mut System) {
        system.memory[0] = 0xAF;
        system.memory[1] = 0x7F;
        system.memory[NUM_BYTES / 2] = 0x77;
        system.memory[NUM_BYTES / 2 + 1] = 0x88;
        system.memory[NUM_BYTES - 2] = 0x3A;
        system.memory[NUM_BYTES - 1] = 0x01;
    }

    /** Read a word (two bytes) from the system memory. */
    #[test]
    fn read_word() {
        let mut system = System::new();
        load_test_data(&mut system);
        assert_eq!(0xAF7F, system.read_word(0));
        assert_eq!(0x7788, system.read_word(NUM_BYTES / 2));
        assert_eq!(0x3A01, system.read_word(NUM_BYTES - 2));
    }
}
