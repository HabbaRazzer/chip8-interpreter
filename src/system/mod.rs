mod macros;
mod opcode;
extern crate rand;

pub type Word = u16;
pub type Byte = u8;
pub type RegisterAddress = u8;

const NUM_BYTES: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 48;

const VALUE_MASK: Word = 0b0000_0000_1111_1111;
const REGISTER_ADDRESS_MASK: Word = 0b0000_1111_0000_0000;

const TYPE_MASK: Word = 0b0000_0000_0000_1111;
const LEFT_MASK: Word = 0b0000_1111_0000_0000;
const RIGHT_MASK: Word = 0b0000_0000_1111_0000;

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

    /** Execute the instruction. */
    pub fn execute(&mut self, opcode: Word) {
        match opcode {

            0x6000...0x7000 => {    // 0x6XNN : VX = NN
                let register = ((opcode & REGISTER_ADDRESS_MASK) >> 8) as usize;
                let value = (opcode & VALUE_MASK) as Byte;
                self.registers[register] = value;
            },

            0x7000...0x8000 => {    // 0x7XNN : VX = VX + NN
                let register = ((opcode & REGISTER_ADDRESS_MASK) >> 8) as usize;
                let value = (opcode & VALUE_MASK) as Byte;
                self.registers[register] = self.registers[register].wrapping_add(value);
            },

            0x8000...0x9000 => {
                let left = ((opcode & LEFT_MASK) >> 8) as usize;
                let right = ((opcode & RIGHT_MASK) >> 4) as usize;
                match opcode & TYPE_MASK {

                    0x0 => {    // 0x8XY0 : VX = VY
                        self.registers[left] = self.registers[right];
                    },

                    0x1 => {    // 0x8XY1 : VX = VX | VY
                        self.registers[left] |= self.registers[right];
                    },

                    0x2 => {    // 0x8XY2 : VX = VX & VY
                        self.registers[left] &= self.registers[right];
                    },

                    0x3 => {    // 0x8XY3 : VX = VX ^ VY
                        self.registers[left] ^= self.registers[right];
                    },

                    0x4 => {    // 0x8XY4 : VX = VX + VY
                        let (value, overflow) =
                            self.registers[left].overflowing_add(self.registers[right]);
                        self.registers[left] = value;
                        if overflow {
                            self.registers[0xF] = 0x1;
                        } else {
                            self.registers[0xF] = 0x0;
                        }
                    },

                    0x5 => {    // 0x8XY5 : VX = VX - VY
                        let (value, overflow) =
                            self.registers[left].overflowing_sub(self.registers[right]);
                        self.registers[left] = value;
                        if overflow {
                            self.registers[0xF] = 0x1;
                        } else {
                            self.registers[0xF] = 0x0;
                        }
                    },

                    0x6 => {    // 0x8XY6 : VX = VY >> 1
                        let least_bit = self.registers[right] & 0x1;
                        self.registers[left] = self.registers[right] >> 1;
                        self.registers[0xF] = least_bit;
                    },

                    0x7 => {    // 0x8XY7 : VY = VY - VX
                        let (value, overflow) =
                            self.registers[right].overflowing_sub(self.registers[left]);
                        self.registers[right] = value;
                        if overflow {
                            self.registers[0xF] = 0x1;
                        } else {
                            self.registers[0xF] = 0x0;
                        }
                    },

                    0xE => {    // 0x8XYE : VX = VY << 1
                        let most_bit = self.registers[right] >> 7;
                        self.registers[left] = self.registers[right] << 1;
                        self.registers[0xF] = most_bit;
                    },

                    _ => {
                        eprintln!("Unrecognized instruction!");
                    }
                }
            },

            0xC000...0xD000 => {    // 0xCXNN : VX = (random) & NN
                let register = ((opcode & REGISTER_ADDRESS_MASK) >> 8) as usize;
                let value = (opcode & VALUE_MASK) as Byte;
                self.registers[register] = rand::random::<Byte>() & value;
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

    /** Set some registers for the purposes of testing. */
    fn set_registers_for_test(system: &mut System) {
        system.execute(0x6064);
        system.execute(0x6127);
        system.execute(0x6212);
        system.execute(0x63AE);
        system.execute(0x64FF);
        system.execute(0x65B4);
        system.execute(0x6642);
        system.execute(0x6F25);
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
        assert_eq!(0x15, system.registers[0x0]);

        system.execute(0x6120);
        assert_eq!(0x20, system.registers[0x1]);

        system.execute(0x6225);
        assert_eq!(0x25, system.registers[0x2]);

        system.execute(0x6330);
        assert_eq!(0x30, system.registers[0x3]);

        system.execute(0x6435);
        assert_eq!(0x35, system.registers[0x4]);

        system.execute(0x6540);
        assert_eq!(0x40, system.registers[0x5]);

        system.execute(0x6645);
        assert_eq!(0x45, system.registers[0x6]);

        system.execute(0x6750);
        assert_eq!(0x50, system.registers[0x7]);

        system.execute(0x6855);
        assert_eq!(0x55, system.registers[0x8]);

        system.execute(0x6960);
        assert_eq!(0x60, system.registers[0x9]);

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

    /** The opcode 0x7XNN should add the constant NN into register VX. */
    #[test]
    fn add_constant() {
        let mut system = System::new();

        system.execute(0x6015);
        system.execute(0x7015);
        assert_eq!(0x2A, system.registers[0x0]);

        system.execute(0x6A42);
        system.execute(0x7A42);
        assert_eq!(0x84, system.registers[0xA]);

        system.execute(0x6EFF);
        system.execute(0x7E01);
        // registers should overlow appropriately
        assert_eq!(0x00, system.registers[0xE]);
    }

    /** The opcode 0x8XY0 should copy the value from register VY into register VX. */
    #[test]
    fn copy_register() {
        let mut system = System::new();

        system.execute(0x6A42);
        system.execute(0x8EA0);
        assert_eq!(0x42, system.registers[0xA]);
        assert_eq!(0x42, system.registers[0xE]);

        system.execute(0x67DE);
        system.execute(0x8F70);
        assert_eq!(0xDE, system.registers[0x7]);
        assert_eq!(0xDE, system.registers[0xF]);
    }

    /** The opcode 0x8XY1 should set register VX to the value (VX OR VY). */
    #[test]
    fn oring_register() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        system.execute(0x8011);
        assert_eq!(0x67, system.registers[0x0]);
        assert_eq!(0x27, system.registers[0x1]);

        system.execute(0x8231);
        assert_eq!(0xBE, system.registers[0x2]);
        assert_eq!(0xAE, system.registers[0x3]);

        system.execute(0x8FE1);
        assert_eq!(0x25, system.registers[0xF]);
        assert_eq!(0x00, system.registers[0xE]);
    }

    /** The opcode 0x8XY2 should set register VX to the value (VX AND VY). */
    #[test]
    fn anding_register() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        system.execute(0x8012);
        assert_eq!(0x24, system.registers[0x0]);
        assert_eq!(0x27, system.registers[0x1]);

        system.execute(0x8232);
        assert_eq!(0x02, system.registers[0x2]);
        assert_eq!(0xAE, system.registers[0x3]);

        system.execute(0x8FE2);
        assert_eq!(0x00, system.registers[0xF]);
        assert_eq!(0x00, system.registers[0xE]);
    }

    /** The opcode 0x8XY3 should set register VX to the value (VX XOR VY). */
    #[test]
    fn xoring_register() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        system.execute(0x8013);
        assert_eq!(0x43, system.registers[0x0]);
        assert_eq!(0x27, system.registers[0x1]);

        system.execute(0x8233);
        assert_eq!(0xBC, system.registers[0x2]);
        assert_eq!(0xAE, system.registers[0x3]);

        system.execute(0x8FE3);
        assert_eq!(0x25, system.registers[0xF]);
        assert_eq!(0x00, system.registers[0xE]);
    }

    /** The opcode 0x8XY4 should add register VY to register VX
      * If a carry occurs, set register VF to 01. */
    #[test]
    fn add_register_with_carry() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        system.execute(0x8014);
        assert_eq!(0x8B, system.registers[0x0]);
        assert_eq!(0x00, system.registers[0xF]);

        system.execute(0x8234);
        assert_eq!(0xC0, system.registers[0x2]);
        assert_eq!(0x00, system.registers[0xF]);

        system.execute(0x6502);
        system.execute(0x8454);
        assert_eq!(0x01, system.registers[0x4]);
        // overflow has occured - register VF should be set to 0x01
        assert_eq!(0x01, system.registers[0xF]);
    }

    /** The opcode 0x8XY5 should subtract register VY from register VX
      * If a borrow occurs, set register VF to 01. */
    #[test]
    fn sub_register_with_borrow_right_subtrahend() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        system.execute(0x8125);
        assert_eq!(0x15, system.registers[0x1]);
        assert_eq!(0x00, system.registers[0xF]);

        system.execute(0x8455);
        assert_eq!(0x4B, system.registers[0x4]);
        assert_eq!(0x00, system.registers[0xF]);

        system.execute(0x6501);
        system.execute(0x8B55);
        assert_eq!(0xFF, system.registers[0xB]);
        // note - a borrow occurs here because subtrahend > minuend,
            // therefore register VF should be set to 0x01
        assert_eq!(0x01, system.registers[0xF]);
    }

    /** The opcode 0x8XY6 should store the value stored in register VY right shifted by 1 bit
    *     in register VX. Register VF should be set to the least significant bit. */
    #[test]
    fn rshift_register() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        system.execute(0x8016);
        assert_eq!(0x13, system.registers[0x0]);
        assert_eq!(0x01, system.registers[0xF]);

        system.execute(0x8236);
        assert_eq!(0x57, system.registers[0x2]);
        assert_eq!(0x00, system.registers[0xF]);

        system.execute(0x8446);
        assert_eq!(0x7F, system.registers[0x4]);
        assert_eq!(0x01, system.registers[0xF]);
    }

    /** The opcode 0x8XY7 should subtract register VX from register VY
      *     If a borrow occurs, set register VF to 01. */
    #[test]
    fn sub_register_with_borrow_left_subtrahend() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        system.execute(0x8217);
        assert_eq!(0x15, system.registers[0x1]);
        assert_eq!(0x00, system.registers[0xF]);

        system.execute(0x8547);
        assert_eq!(0x4B, system.registers[0x4]);
        assert_eq!(0x00, system.registers[0xF]);

        system.execute(0x6501);
        system.execute(0x85B7);
        assert_eq!(0xFF, system.registers[0xB]);
        // note - a borrow occurs here because subtrahend > minuend,
            // therefore register VF should be set to 0x01
        assert_eq!(0x01, system.registers[0xF]);
    }

    /** The opcode 0x8XYE should store the value stored in register VY left shifted by 1 bit
      *     in register VX. Register VF should be set to the most significant bit. */
    #[test]
    fn lshift_register() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        system.execute(0x801E);
        assert_eq!(0x4E, system.registers[0x0]);
        assert_eq!(0x00, system.registers[0xF]);

        system.execute(0x823E);
        assert_eq!(0x5C, system.registers[0x2]);
        assert_eq!(0x01, system.registers[0xF]);

        system.execute(0x844E);
        assert_eq!(0xFE, system.registers[0x4]);
        assert_eq!(0x01, system.registers[0xF]);
    }

    /** The opcode 0xCXNN should generate a random number, mask it with NN and store it in
      *     register VX. */
    #[test]
    #[ignore]
    fn random_register() {
        // TODO
    }
}
