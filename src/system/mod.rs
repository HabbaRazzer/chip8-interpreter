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

impl System{
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

    pub fn read_word(&self, address: usize) -> &[u8] {
        &self.memory[address..address + 2]
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
        assert_eq!([0xAF, 0x7F], system.read_word(0));
        assert_eq!([0x77, 0x88], system.read_word(NUM_BYTES / 2));
        assert_eq!([0x3A, 0x01], system.read_word(NUM_BYTES - 2));
    }
}
