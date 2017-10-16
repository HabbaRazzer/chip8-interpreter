extern crate rand;

use system::{System, Word, Byte, RegisterAddress};

const VALUE_MASK: Word = 0b0000_0000_1111_1111;
const REGISTER_MASK: Word = 0b0000_1111_0000_0000;

const TYPE_MASK: Word = 0b0000_0000_0000_1111;
const LEFT_MASK: Word = 0b0000_1111_0000_0000;
const RIGHT_MASK: Word = 0b0000_0000_1111_0000;

#[allow(dead_code)]
enum OpCode {
	SetValue(RegisterAddress, Byte),
	AddValue(RegisterAddress, Byte),
	SetRegister(RegisterAddress, RegisterAddress),
	OrRegister(RegisterAddress, RegisterAddress),
	AndRegister(RegisterAddress, RegisterAddress),
	XorRegister(RegisterAddress, RegisterAddress),
	AddRegister(RegisterAddress, RegisterAddress),
	SubRegisterRight(RegisterAddress, RegisterAddress),
	RShiftRegister(RegisterAddress, RegisterAddress),
	SubRegisterLeft(RegisterAddress, RegisterAddress),
	LShiftRegister(RegisterAddress, RegisterAddress),
	RandomValue(RegisterAddress, Byte),
	Unknown
}

#[allow(dead_code)]
impl OpCode {
	fn execute(&self, system: &mut System) {
		match self {
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
			}

			_ => {
				eprintln!("Unrecognized Instruction!");
			}
		}
	}
}

impl From<Word> for OpCode {
	fn from(word: Word) -> Self {
		let register = ((word & REGISTER_MASK) >> 8) as usize;
		let value = (word & VALUE_MASK) as Byte;

		match word {
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

			0xC000...0xD000 => {
				OpCode::RandomValue(register, value)
			}

			_ => {
				OpCode::Unknown
			}

		}
	}
}
