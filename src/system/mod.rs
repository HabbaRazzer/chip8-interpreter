mod macros;

const NUM_BYTES: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 48;

const VALUE_MASK: u16 = 0b0000_0000_1111_1111;
const REGISTER_ADDRESS_MASK: u16 = 0b0000_1111_0000_0000;

#[allow(dead_code)]
pub struct System {
    memory: [u8; NUM_BYTES],
    registers: [u8; NUM_REGISTERS],
    pc: u16,
    index: u16,
    stack: [u8; STACK_SIZE],
    sp: u8
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
            sp: 0
        }
    }

    /** Index into system memory at address and return word located there. */
    pub fn read_word(&self, address: usize) -> u16 {
        let left = self.memory[address] as u16;
        let right = self.memory[address + 1] as u16;
        (left << 8) | right
    }

    /** Execute the instruction. */
    pub fn execute(&mut self, opcode: u16) {
        match opcode {
            0x6000...0x7000 => {
                self.registers[((opcode & REGISTER_ADDRESS_MASK) >> 8) as usize] =
                    (opcode & VALUE_MASK) as u8;
            },
            _ => {
                eprintln!("Unrecognized instruction!");
            }
        }
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

    /** The opcode 0x6XNN should store the constant NN into register VX. */
    #[test]
    fn load_constant() {
        let mut system = System::new();
        system.execute(0x6015);
        assert_eq!(0x15, system.registers[0]);
        system.execute(0x6120);
        assert_eq!(0x20, system.registers[1]);
        system.execute(0x6225);
        assert_eq!(0x25, system.registers[2]);
        system.execute(0x6330);
        assert_eq!(0x30, system.registers[3]);
        system.execute(0x6435);
        assert_eq!(0x35, system.registers[4]);
        system.execute(0x6540);
        assert_eq!(0x40, system.registers[5]);
        system.execute(0x6645);
        assert_eq!(0x45, system.registers[6]);
        system.execute(0x6750);
        assert_eq!(0x50, system.registers[7]);
        system.execute(0x6855);
        assert_eq!(0x55, system.registers[8]);
        system.execute(0x6960);
        assert_eq!(0x60, system.registers[9]);
        system.execute(0x6A65);
        assert_eq!(0x65, system.registers[0xA]);
        system.execute(0x6B70);
        assert_eq!(0x70, system.registers[0xB]);
        system.execute(0x6C75);
        assert_eq!(0x75, system.registers[0xC]);
        system.execute(0x6D80);
        assert_eq!(0x80, system.registers[0xD]);
        system.execute(0x6E85);
        assert_eq!(0x85, system.registers[0xE]);
        system.execute(0x6F90);
        assert_eq!(0x90, system.registers[0xF]);
    }
}
