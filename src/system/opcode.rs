use system::Word;

const VALUE_MASK: Word = 0b0000_0000_1111_1111;
const REGISTER_ADDRESS_MASK: Word = 0b0000_1111_0000_0000;

const TYPE_MASK: Word = 0b0000_0000_0000_1111;
const LEFT_MASK: Word = 0b0000_1111_0000_0000;
const RIGHT_MASK: Word = 0b0000_0000_1111_0000;

impl From<Word> for OpCode {
	fn from(word: Word) -> Self {
		let target = (word & REGISTER_ADDRESS_MASK) as u8;
		let source = ((word & SOURCE_MASK) >> 4) as u8;
		let value = (word & VALUE_MASK) as u8;

		match word {
			0x6000...0x7000 => {
				OpCode::Set(target, SetKind::Value(value))
			},

			0x7000...0x8000 => {
				OpCode::Add(target, AddKind::Value(value))
			},

			0x8000...0x9000 => {
				let least = word & 0b0000_0000_0000_1111;
				match least {
					0x0 => {
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::Value))
					},

					0x1 => {
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::Or))
					},

					0x2 => {
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::And))
					},

					0x3 => {
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::Xor))
					},

					0x4 => {
						OpCode::Add(target,
							AddKind::Register(source, RegisterOperation::Value))
					},

					0x5 => {
						OpCode::Sub(target,
							source as SourceRegisterAddress, SubKind::SourceSubtrahend)
					},

					0x6 => {
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::RightShift))
					},

					0x7 => {
						OpCode::Sub( target,
							source as SourceRegisterAddress, SubKind::TargetSubtrahend)
					}

					0xE => {
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::LeftShift))
					},

					_ => {
						OpCode::UnKnown
					}

				}
			},

			0xC000...0xD000 => {
				OpCode::Random(target, (word & VALUE_MASK) as Mask)
			}

			_ => {
				OpCode::UnKnown
			}
		}
	}
}

pub enum OpCode {
	UnKnown,
	Clear,
	Return,
	Jump(MemoryAddress),
	Call(MemoryAddress),
	SkipEq(RegisterAddress, SkipKind),
	SkipNe(RegisterAddress, SkipKind),
	Set(RegisterAddress, SetKind),
	Add(RegisterAddress, AddKind),
	Sub(RegisterAddress, SourceRegisterAddress, SubKind),
	Random(RegisterAddress, Mask),
}

pub enum SkipKind {
	Value(Bit),
	Register(RegisterAddress)
}

pub enum SetKind {
	Value(Bit),
	Register(RegisterAddress, RegisterOperation)
}

pub enum RegisterOperation {
	Value,
	Or,
	And,
	Xor,
	RightShift,
	LeftShift,
}

pub enum AddKind {
	Value(Bit),
	Register(RegisterAddress, RegisterOperation)
}

pub enum SubKind {
	TargetSubtrahend,
	SourceSubtrahend,
}
