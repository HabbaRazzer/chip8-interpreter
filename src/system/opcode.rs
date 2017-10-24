extern crate rand;

use system::{System, Word, Byte, RegisterIndex, Address};

const VALUE_MASK: Word = 0b0000_0000_1111_1111;
const REGISTER_MASK: Word = 0b0000_1111_0000_0000;
const ADDRESS_MASK: Word = 0b0000_1111_1111_1111;

const TYPE_MASK: Word = 0b0000_0000_0000_1111;
const LEFT_MASK: Word = 0b0000_1111_0000_0000;
const RIGHT_MASK: Word = 0b0000_0000_1111_0000;

#[allow(dead_code)]
enum OpCode {
	JumpAddress(Address),
	JumpAddressOffset(Address),
	SubJump(Address),
	SubReturn,
	SkipValue(RegisterIndex, Byte),
	SkipRegister(RegisterIndex, RegisterIndex),
	SkipNotValue(RegisterIndex, Byte),
	SkipNotRegister(RegisterIndex, RegisterIndex),
	SetDelayTimer(RegisterIndex),
	SetRegisterFromTimer(RegisterIndex),
	SetSoundTimer(RegisterIndex),
	SetValue(RegisterIndex, Byte),
	AddValue(RegisterIndex, Byte),
	SetRegister(RegisterIndex, RegisterIndex),
	OrRegister(RegisterIndex, RegisterIndex),
	AndRegister(RegisterIndex, RegisterIndex),
	XorRegister(RegisterIndex, RegisterIndex),
	AddRegister(RegisterIndex, RegisterIndex),
	SubRegisterRight(RegisterIndex, RegisterIndex),
	RShiftRegister(RegisterIndex, RegisterIndex),
	SubRegisterLeft(RegisterIndex, RegisterIndex),
	LShiftRegister(RegisterIndex, RegisterIndex),
	RandomValue(RegisterIndex, Byte),
	WaitKeyPress(RegisterIndex),
	SkipKeyPressed(RegisterIndex),
	SkipKeyNotPressed(RegisterIndex),
	Unknown
}

#[allow(dead_code)]
impl OpCode {
	fn execute(&self, system: &mut System) -> Result<(), &'static str> {
		match self {
			&OpCode::JumpAddress(address) => {
				system.pc = address;
			},

			&OpCode::JumpAddressOffset(address) => {
				system.pc = address + system.registers[0x0] as Word;
			},

			&OpCode::SubJump(address) => {
				system.stack[system.sp as usize] = system.pc;
				system.sp += 1;
				system.pc = address;
			},

			&OpCode::SubReturn => {
				system.sp -= 1;
				system.pc = system.stack[system.sp as usize];
			},

			&OpCode::SkipValue(register, value) => {
				if system.registers[register] == value {
					system.increment_pc();
				}
			},

			&OpCode::SkipRegister(left, right) => {
				if system.registers[left] == system.registers[right] {
					system.increment_pc();
				}
			},

			&OpCode::SkipNotValue(register, value) => {
				if system.registers[register] != value {
					system.increment_pc();
				}
			},

			&OpCode::SkipNotRegister(left, right) => {
				if system.registers[left] != system.registers[right] {
					system.increment_pc();
				}
			},

			&OpCode::SetDelayTimer(register) => {
				system.delay_timer = system.registers[register];
			},

			&OpCode::SetRegisterFromTimer(register) => {
				system.registers[register] = system.delay_timer;
			},

			&OpCode::SetSoundTimer(register) => {
				system.sound_timer = system.registers[register];
			},

			&OpCode::SetValue(register, value) => {
				system.registers[register] = value;
			},

			&OpCode::AddValue(register, value) => {
				system.registers[register]
					= system.registers[register].wrapping_add(value);
			},

			&OpCode::SetRegister(left, right) => {
				system.registers[left] = system.registers[right];
			},

			&OpCode::OrRegister(left, right) => {
				system.registers[left] |= system.registers[right];
			},

			&OpCode::AndRegister(left, right) => {
				system.registers[left] &= system.registers[right];
			},

			&OpCode::XorRegister(left, right) => {
				system.registers[left] ^= system.registers[right];
			},

			&OpCode::AddRegister(left, right) => {
				let (value, overflow) =
					system.registers[left].overflowing_add(system.registers[right]);
				system.registers[left] = value;
				if overflow {
					system.registers[0xF] = 0x1;
				} else {
					system.registers[0xF] = 0x0;
				}
			},

			&OpCode::SubRegisterRight(left, right) => {
				let (value, overflow) =
					system.registers[left].overflowing_sub(system.registers[right]);
				system.registers[left] = value;
				if overflow {
					system.registers[0xF] = 0x1;
				} else {
					system.registers[0xF] = 0x0;
				}
			},

			&OpCode::RShiftRegister(left, right) => {
				let least_bit = system.registers[right] & 0x1;
				system.registers[left] = system.registers[right] >> 1;
				system.registers[0xF] = least_bit;
			},

			&OpCode::SubRegisterLeft(left, right) => {
				let (value, overflow) =
					system.registers[right].overflowing_sub(system.registers[left]);
				system.registers[right] = value;
				if overflow {
					system.registers[0xF] = 0x1;
				} else {
					system.registers[0xF] = 0x0;
				}
			},

			&OpCode::LShiftRegister(left, right) => {
				let most_bit = system.registers[right] >> 0x7;
				system.registers[left] = system.registers[right] << 1;
				system.registers[0xF] = most_bit;
			},

			&OpCode::RandomValue(register, value) => {
				system.registers[register] = rand::random::<Byte>() & value;
			},

			&OpCode::WaitKeyPress(register) => {
				// TODO: This might not grab the most recently pushed key due to the way I check
				//   for each index here. It is possible that a key is pushed in an already checked
				//   index before a key is pushed in an index yet to be checked. In this way the
				//   key that was pressed second will be stored.
				loop {
					for (index, is_pressed) in system.keys.iter().enumerate() {
						if *is_pressed {
							system.registers[register] = index as u8;
							break;
						}
					}
				}
			},

			&OpCode::SkipKeyPressed(register) => {
				let index = system.registers[register] as usize;
				if system.keys[index] {
					system.increment_pc();
				}
			},

			&OpCode::SkipKeyNotPressed(register) => {
				let index = system.registers[register] as usize;
				if !system.keys[index] {
					system.increment_pc();
				}
			},

			&OpCode::Unknown => {
				return Err("Unrecognized Instruction!");
			}
		}

		Ok(())
	}
}

impl From<Word> for OpCode {
	fn from(word: Word) -> Self {
		let register = ((word & REGISTER_MASK) >> 8) as usize;
		let value = (word & VALUE_MASK) as Byte;

		match word {
			0x0000...0x1000 => {
				OpCode::SubReturn
			},

			0x1000...0x2000 => {
				OpCode::JumpAddress(word & ADDRESS_MASK)
			},

			0x2000...0x3000 => {
				OpCode::SubJump(word & ADDRESS_MASK)
			},

			0x3000...0x4000 => {
				OpCode::SkipValue(register, value)
			},

			0x4000...0x5000 => {
				OpCode::SkipNotValue(register, value)
			}

			0x5000...0x6000 => {
				OpCode::SkipRegister(
					((word & LEFT_MASK) >> 8) as usize,
					((word & RIGHT_MASK) >> 4) as usize)
			},

			0x6000...0x7000 => {
				OpCode::SetValue(register, value)
			},

			0x7000...0x8000 => {
				OpCode::AddValue(register, value)
			},

			0x8000...0x9000 => {
				let left = ((word & LEFT_MASK) >> 8) as usize;
                let right = ((word & RIGHT_MASK) >> 4) as usize;
				match word & TYPE_MASK {
					0x0 => {
						OpCode::SetRegister(left, right)
					},

					0x1 => {
						OpCode::OrRegister(left, right)
					},

					0x2 => {
						OpCode::AndRegister(left, right)
					},

					0x3 => {
						OpCode::XorRegister(left, right)
					},

					0x4 => {
						OpCode::AddRegister(left, right)
					},

					0x5 => {
						OpCode::SubRegisterRight(left, right)
					},

					0x6 => {
						OpCode::RShiftRegister(left, right)
					},

					0x7 => {
						OpCode::SubRegisterLeft(left, right)
					}

					0xE => {
						OpCode::LShiftRegister(left, right)
					},

					_ => {
						OpCode::Unknown
					}
				}
			},

			0x9000...0xA000 => {
				OpCode::SkipNotRegister(
					((word & LEFT_MASK) >> 8) as usize,
					((word & RIGHT_MASK) >> 4) as usize)
			},

			0xB000...0xC000 => {
				OpCode::JumpAddressOffset(word & ADDRESS_MASK)
			},

			0xC000...0xD000 => {
				OpCode::RandomValue(register, value)
			},

			0xE000...0xF000 => {
				match word & VALUE_MASK {
					0x9E => {
						OpCode::SkipKeyPressed(register)
					},

					0xA1 => {
						OpCode::SkipKeyNotPressed(register)
					},

					_ => {
						OpCode::Unknown
					}
				}
			},

			0xF000...0xFFFF => {
				match word & VALUE_MASK {
					0x15 => {
						OpCode::SetDelayTimer(register)
					},

					0x07 => {
						OpCode::SetRegisterFromTimer(register)
					},

					0x18 => {
						OpCode::SetSoundTimer(register)
					},

					0x0A => {
						OpCode::WaitKeyPress(register)
					},

					_ => {
						OpCode::Unknown
					}
				}
			},

			_ => {
				OpCode::Unknown
			}
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    /** Set some registers for the purposes of testing. */
    fn set_registers_for_test(system: &mut System) {
		OpCode::from(0x6064).execute(system).unwrap();
        OpCode::from(0x6127).execute(system).unwrap();
        OpCode::from(0x6212).execute(system).unwrap();
        OpCode::from(0x63AE).execute(system).unwrap();
        OpCode::from(0x64FF).execute(system).unwrap();
        OpCode::from(0x65B4).execute(system).unwrap();
        OpCode::from(0x6642).execute(system).unwrap();
        OpCode::from(0x6F25).execute(system).unwrap();
    }

    /** The opcode 0x6XNN should store the constant NN into register VX. */
    #[test]
    fn load_constant() {
        let mut system = System::new();

        OpCode::from(0x6015).execute(&mut system).unwrap();
        assert_eq!(0x15, system.registers[0x0]);

        OpCode::from(0x6120).execute(&mut system).unwrap();
        assert_eq!(0x20, system.registers[0x1]);

		OpCode::from(0x6225).execute(&mut system).unwrap();
        assert_eq!(0x25, system.registers[0x2]);

		OpCode::from(0x6330).execute(&mut system).unwrap();
        assert_eq!(0x30, system.registers[0x3]);

		OpCode::from(0x6435).execute(&mut system).unwrap();
        assert_eq!(0x35, system.registers[0x4]);

		OpCode::from(0x6540).execute(&mut system).unwrap();
        assert_eq!(0x40, system.registers[0x5]);

		OpCode::from(0x6645).execute(&mut system).unwrap();
        assert_eq!(0x45, system.registers[0x6]);

		OpCode::from(0x6750).execute(&mut system).unwrap();
        assert_eq!(0x50, system.registers[0x7]);

		OpCode::from(0x6855).execute(&mut system).unwrap();
        assert_eq!(0x55, system.registers[0x8]);

		OpCode::from(0x6960).execute(&mut system).unwrap();
        assert_eq!(0x60, system.registers[0x9]);

		OpCode::from(0x6A65).execute(&mut system).unwrap();
        assert_eq!(0x65, system.registers[0xA]);

		OpCode::from(0x6B70).execute(&mut system).unwrap();
        assert_eq!(0x70, system.registers[0xB]);

		OpCode::from(0x6C75).execute(&mut system).unwrap();
        assert_eq!(0x75, system.registers[0xC]);

		OpCode::from(0x6D80).execute(&mut system).unwrap();
        assert_eq!(0x80, system.registers[0xD]);

		OpCode::from(0x6E85).execute(&mut system).unwrap();
        assert_eq!(0x85, system.registers[0xE]);

		OpCode::from(0x6F90).execute(&mut system).unwrap();
        assert_eq!(0x90, system.registers[0xF]);
    }

    /** The opcode 0x7XNN should add the constant NN into register VX. */
    #[test]
    fn add_constant() {
        let mut system = System::new();

        OpCode::from(0x6015).execute(&mut system).unwrap();
        OpCode::from(0x7015).execute(&mut system).unwrap();
        assert_eq!(0x2A, system.registers[0x0]);

        OpCode::from(0x6A42).execute(&mut system).unwrap();
        OpCode::from(0x7A42).execute(&mut system).unwrap();
        assert_eq!(0x84, system.registers[0xA]);

        OpCode::from(0x6EFF).execute(&mut system).unwrap();
        OpCode::from(0x7E01).execute(&mut system).unwrap();
        // registers should overlow appropriately
        assert_eq!(0x00, system.registers[0xE]);
    }

    /** The opcode 0x8XY0 should copy the value from register VY into register VX. */
    #[test]
    fn copy_register() {
        let mut system = System::new();

        OpCode::from(0x6A42).execute(&mut system).unwrap();
        OpCode::from(0x8EA0).execute(&mut system).unwrap();
        assert_eq!(0x42, system.registers[0xA]);
        assert_eq!(0x42, system.registers[0xE]);

        OpCode::from(0x67DE).execute(&mut system).unwrap();
        OpCode::from(0x8F70).execute(&mut system).unwrap();
        assert_eq!(0xDE, system.registers[0x7]);
        assert_eq!(0xDE, system.registers[0xF]);
    }

    /** The opcode 0x8XY1 should set register VX to the value (VX OR VY). */
    #[test]
    fn oring_register() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        OpCode::from(0x8011).execute(&mut system).unwrap();
        assert_eq!(0x67, system.registers[0x0]);
        assert_eq!(0x27, system.registers[0x1]);

        OpCode::from(0x8231).execute(&mut system).unwrap();
        assert_eq!(0xBE, system.registers[0x2]);
        assert_eq!(0xAE, system.registers[0x3]);

        OpCode::from(0x8FE1).execute(&mut system).unwrap();
        assert_eq!(0x25, system.registers[0xF]);
        assert_eq!(0x00, system.registers[0xE]);
    }

    /** The opcode 0x8XY2 should set register VX to the value (VX AND VY). */
    #[test]
    fn anding_register() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        OpCode::from(0x8012).execute(&mut system).unwrap();
        assert_eq!(0x24, system.registers[0x0]);
        assert_eq!(0x27, system.registers[0x1]);

        OpCode::from(0x8232).execute(&mut system).unwrap();
        assert_eq!(0x02, system.registers[0x2]);
        assert_eq!(0xAE, system.registers[0x3]);

        OpCode::from(0x8FE2).execute(&mut system).unwrap();
        assert_eq!(0x00, system.registers[0xF]);
        assert_eq!(0x00, system.registers[0xE]);
    }

    /** The opcode 0x8XY3 should set register VX to the value (VX XOR VY). */
    #[test]
    fn xoring_register() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        OpCode::from(0x8013).execute(&mut system).unwrap();
        assert_eq!(0x43, system.registers[0x0]);
        assert_eq!(0x27, system.registers[0x1]);

        OpCode::from(0x8233).execute(&mut system).unwrap();
        assert_eq!(0xBC, system.registers[0x2]);
        assert_eq!(0xAE, system.registers[0x3]);

        OpCode::from(0x8FE3).execute(&mut system).unwrap();
        assert_eq!(0x25, system.registers[0xF]);
        assert_eq!(0x00, system.registers[0xE]);
    }

    /** The opcode 0x8XY4 should add register VY to register VX
      * If a carry occurs, set register VF to 01. */
    #[test]
    fn add_register_with_carry() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        OpCode::from(0x8014).execute(&mut system).unwrap();
        assert_eq!(0x8B, system.registers[0x0]);
        assert_eq!(0x00, system.registers[0xF]);

        OpCode::from(0x8234).execute(&mut system).unwrap();
        assert_eq!(0xC0, system.registers[0x2]);
        assert_eq!(0x00, system.registers[0xF]);

        OpCode::from(0x6502).execute(&mut system).unwrap();
        OpCode::from(0x8454).execute(&mut system).unwrap();
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

        OpCode::from(0x8125).execute(&mut system).unwrap();
        assert_eq!(0x15, system.registers[0x1]);
        assert_eq!(0x00, system.registers[0xF]);

        OpCode::from(0x8455).execute(&mut system).unwrap();
        assert_eq!(0x4B, system.registers[0x4]);
        assert_eq!(0x00, system.registers[0xF]);

        OpCode::from(0x6501).execute(&mut system).unwrap();
        OpCode::from(0x8B55).execute(&mut system).unwrap();
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

        OpCode::from(0x8016).execute(&mut system).unwrap();
        assert_eq!(0x13, system.registers[0x0]);
        assert_eq!(0x01, system.registers[0xF]);

        OpCode::from(0x8236).execute(&mut system).unwrap();
        assert_eq!(0x57, system.registers[0x2]);
        assert_eq!(0x00, system.registers[0xF]);

        OpCode::from(0x8446).execute(&mut system).unwrap();
        assert_eq!(0x7F, system.registers[0x4]);
        assert_eq!(0x01, system.registers[0xF]);
    }

    /** The opcode 0x8XY7 should subtract register VX from register VY
      *     If a borrow occurs, set register VF to 01. */
    #[test]
    fn sub_register_with_borrow_left_subtrahend() {
        let mut system = System::new();
        set_registers_for_test(&mut system);

        OpCode::from(0x8217).execute(&mut system).unwrap();
        assert_eq!(0x15, system.registers[0x1]);
        assert_eq!(0x00, system.registers[0xF]);

        OpCode::from(0x8547).execute(&mut system).unwrap();
        assert_eq!(0x4B, system.registers[0x4]);
        assert_eq!(0x00, system.registers[0xF]);

        OpCode::from(0x6501).execute(&mut system).unwrap();
        OpCode::from(0x85B7).execute(&mut system).unwrap();
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

        OpCode::from(0x801E).execute(&mut system).unwrap();
        assert_eq!(0x4E, system.registers[0x0]);
        assert_eq!(0x00, system.registers[0xF]);

        OpCode::from(0x823E).execute(&mut system).unwrap();
        assert_eq!(0x5C, system.registers[0x2]);
        assert_eq!(0x01, system.registers[0xF]);

        OpCode::from(0x844E).execute(&mut system).unwrap();
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

	/** The opcode 0x1NNN instructs the interpreter to jump to address NNN. */
	#[test]
	fn jump_address() {
		let mut system = System::new();

		OpCode::from(0x12AE).execute(&mut system).unwrap();
		assert_eq!(0x2AE, system.pc);
	}

	/** The opcode 0xBNNN instructs the interpreter to jump to address NNN with an offset
	  * 	specified in register V0. */
	#[test]
	fn jump_address_with_offset() {
		let mut system = System::new();
		OpCode::from(0x6064).execute(&mut system).unwrap();

		OpCode::from(0xB2AE).execute(&mut system).unwrap();
		assert_eq!(0x312, system.pc);
	}

	/** The opcode 0x2NNN should instruct the interpreter to start execution of instructions at
	  * 	address NNN. */
	#[test]
	fn subroutine_jump() {
		let mut system = System::new();

		OpCode::from(0x22AE).execute(&mut system).unwrap();
		assert_eq!(0x2AE, system.pc);
		assert_eq!(1, system.sp);
		assert_eq!(0x200, system.stack[(system.sp - 1) as usize]);
	}

	/** The opcode 0x00EE should instruct the interpreter to return from a subroutine. */
	#[test]
	fn subroutine_return() {
		let mut system = System::new();

		OpCode::from(0x22AE).execute(&mut system).unwrap();
		OpCode::from(0x00EE).execute(&mut system).unwrap();
		assert_eq!(0x200, system.pc);
		assert_eq!(0, system.sp);
	}

	/** The opcode 0x3XNN should instruct the interpreter to skip the next instruction if the
	  * 	value stored in register VX is NN. */
	#[test]
	fn skip_value() {
		let mut system = System::new();
		set_registers_for_test(&mut system);

		OpCode::from(0x3212).execute(&mut system).unwrap();
		// skip
		assert_eq!(0x202, system.pc);

		OpCode::from(0x3213).execute(&mut system).unwrap();
		// do NOT skip
		assert_eq!(0x202, system.pc);
	}

	/** The opcode 0x5XY0 should instruct the interpreter to skip the next instruction if the
	  * 	value stored in register VX is equal to the value stored in register VY. */
	#[test]
	fn skip_register() {
		let mut system = System::new();
		set_registers_for_test(&mut system);
		OpCode::from(0x6227).execute(&mut system).unwrap();

		OpCode::from(0x5120).execute(&mut system).unwrap();
		// skip
		assert_eq!(0x202, system.pc);

		OpCode::from(0x5140).execute(&mut system).unwrap();
		// do NOT skip
		assert_eq!(0x202, system.pc);
	}

	/** The opcode 0x4XNN should instruct the interpreter to skip the next instruction if the
	  * 	value stored in register VX is not equal to NN. */
	#[test]
	fn skip_not_value() {
		let mut system = System::new();
		set_registers_for_test(&mut system);

		OpCode::from(0x4112).execute(&mut system).unwrap();
		// skip
		assert_eq!(0x202, system.pc);

		OpCode::from(0x4212).execute(&mut system).unwrap();
		// do NOT skip
		assert_eq!(0x202, system.pc);
	}

	/** The opcode 0x5XY0 should instruct the interpreter to skip the next instruction if the
	  * 	value stored in register VX is not equal to the value stored in register VY. */
	#[test]
	fn skip_not_register() {
		let mut system = System::new();
		set_registers_for_test(&mut system);
		OpCode::from(0x6227).execute(&mut system).unwrap();

		OpCode::from(0x9140).execute(&mut system).unwrap();
		// skip
		assert_eq!(0x202, system.pc);

		OpCode::from(0x9120).execute(&mut system).unwrap();
		// do NOT skip
		assert_eq!(0x202, system.pc);
	}

	/** The opcode 0xFX15 should set the delay timer to the value stored in register VX. */
	#[test]
	fn set_delay_timer() {
		let mut system = System::new();
		OpCode::from(0x6227).execute(&mut system).unwrap();

		OpCode::from(0xF215).execute(&mut system).unwrap();
		assert_eq!(0x27, system.delay_timer);
	}

	/** The opcode 0xFX07 should store in register VX the current value of the delay timer. */
	#[test]
	fn set_register_from_timer() {
		let mut system = System::new();
		OpCode::from(0x6227).execute(&mut system).unwrap();
		OpCode::from(0xF215).execute(&mut system).unwrap();

		OpCode::from(0xFB07).execute(&mut system).unwrap();
		assert_eq!(0x27, system.registers[0xB]);
	}

	/** The opcode 0xFX18 should set the sound timer to the value stored in register VX. */
	#[test]
	fn set_sound_timer() {
		let mut system = System::new();
		OpCode::from(0x6227).execute(&mut system).unwrap();

		OpCode::from(0xF218).execute(&mut system).unwrap();
		assert_eq!(0x27, system.sound_timer);
	}
}
