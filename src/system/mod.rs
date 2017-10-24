mod opcode;
use std::fs::File;
use std::io::prelude::*;

pub type Word = u16;
pub type Byte = u8;
pub type Address = u16;
pub type RegisterIndex = usize;

const NUM_BYTES: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 48;
const NUM_KEYS: usize = 16;

#[allow(dead_code)]
#[derive(Clone)]
pub struct System {
    memory: Vec<Byte>,
    registers: [Byte; NUM_REGISTERS],
    pc: Word,
    index: Word,
    stack: Vec<Word>,
    sp: Byte,
    delay_timer: Byte,
    sound_timer: Byte,
    keys: Vec<bool>,
}

#[allow(dead_code)]
impl System{
    /// Creates a new System.
    pub fn new() -> Self {
        System {
            memory: vec![0; NUM_BYTES],
            registers: [0; NUM_REGISTERS],
            pc: 0x200,
            index: 0,
            stack: vec![0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: vec![false; NUM_KEYS],
        }
    }

    /// Creates a new System, given a path to a Chip8 rom to be loaded.
    pub fn from_rom(path: &str) -> Self {
        let mut system = System {
            memory: vec![0; NUM_BYTES],
            registers: [0; NUM_REGISTERS],
            pc: 0x200,
            index: 0,
            stack: vec![0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: vec![false; NUM_KEYS],
        };

        let mut handle = File::open(path).expect("File not found!");
        let mut buffer: Vec<Byte> = Vec::new();
        handle.read_to_end(&mut buffer).unwrap();
        for i in 0..buffer.len() {
            system.memory[0x200 + i] = buffer[i];
        };

        system
    }

    /// Index into system memory at address and return word located there.
    pub fn read_word(&self, address: usize) -> Word {
        let left = self.memory[address] as Word;
        let right = self.memory[address + 1] as Word;
        (left << 8) | right
    }

    /// Increment the program counter for this system.
    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }

    /// Set key at specified index.
    pub fn set_key(&mut self, index: usize) {
        self.keys[index] = true;
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
