mod opcode;
use std::fs::File;
use std::io::prelude::*;

pub type Word = u16;
pub type Byte = u8;
pub type Address = u16;
pub type RegisterIndex = usize;
pub type Key = u32;

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
    last_key_pressed: (Byte, bool),
    stopped: bool
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
            last_key_pressed: (0xFF, false),
            stopped: false
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
            last_key_pressed: (0xFF, false),
            stopped: false
        };

        let mut handle = File::open(path).expect("File not found!");
        let mut buffer: Vec<Byte> = Vec::new();
        handle.read_to_end(&mut buffer).unwrap();
        for i in 0..buffer.len() {
            system.memory[0x200 + i] = buffer[i];
        };

        system
    }

    /// Halt execution of the system.
    pub fn stop(&mut self) {
        self.stopped = true;
    }

    /// Start execution of the system.
    pub fn start(&mut self) {
        self.stopped = false;
    }

    /// Handles key input.
    pub fn handle_input(&mut self, key: Key) {
        let value = match key {
            49  => {    // 1 -> 1
                self.set_key(0x1);
                0x1
            },
            50  => {    // 2 -> 2
                self.set_key(0x2);
                0x2
            },
            51  => {    // 3 -> 3
                self.set_key(0x3);
                0x3
            },
            52  => {    // 4 -> C
                self.set_key(0xC);
                0xC
            },
            113 => {    // Q -> 4
                self.set_key(0x4);
                0x4
            },
            119 => {    // W -> 5
                self.set_key(0x5);
                0x5
            },
            101 => {    // E -> 6
                self.set_key(0x6);
                0x6
            },
            114 => {    // R -> D
                self.set_key(0xD);
                0xD
            },
            97  => {    // A -> 7
                self.set_key(0x7);
                0x7
            },
            115 => {    // S -> 8
                self.set_key(0x8);
                0x8
            },
            100 => {    // D -> 9
                self.set_key(0x9);
                0x9
            },
            102 => {    // F -> E
                self.set_key(0xE);
                0xE
            },
            122 => {    // Z -> A
                self.set_key(0xA);
                0xA
            },
            120 => {    // X -> 0
                self.set_key(0x0);
                0x0
            },
            99  => {    // C -> B
                self.set_key(0xB);
                0xB
            },
            118 => {    // V -> F
                self.set_key(0xF);
                0xF
            },
            _ => 0xFF
        };

        if self.stopped {
            self.last_key_pressed = (value, true);
        } else {
            self.last_key_pressed = (value, false);
        }
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
