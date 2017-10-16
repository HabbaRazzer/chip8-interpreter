use system::{Word, Byte};

const VALUE_MASK: Word = 0b0000_0000_1111_1111;
const REGISTER_ADDRESS_MASK: Word = 0b0000_1111_0000_0000;

const TYPE_MASK: Word = 0b0000_0000_0000_1111;
const LEFT_MASK: Word = 0b0000_1111_0000_0000;
const RIGHT_MASK: Word = 0b0000_0000_1111_0000;

impl From<Word> for OpCode {
	fn from(word: Word) -> Self {
		let target = (word & REGISTER_ADDRESS_MASK) as Byte;
		let source = ((word & SOURCE_MASK) >> 4) as Byte;
		let value = (word & VALUE_MASK) as Byte;

		match word {
			0x6000...0x7000 => { // 0x6XNN : VX = NN
				OpCode::Set(target, SetKind::Value(value))
			},

			0x7000...0x8000 => { // 0x7XNN : VX = VX + NN
				OpCode::Add(target, AddKind::Value(value))
			},

			0x8000...0x9000 => {
				let least = word & 0b0000_0000_0000_1111;
				match least {
					0x0 => {       // 0x8XY0 : VX = VY
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::Value))
					},

					0x1 => {       // 0x8XY1 : VX = VX | VY
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::Or))
					},

					0x2 => {       // 0x8XY2 : VX = VX & VY
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::And))
					},

					0x3 => {       // 0x8XY3 : VX = VX ^ VY
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::Xor))
					},

					0x4 => {       // 0x8XY4 : VX = VX + VY
						OpCode::Add(target,
							AddKind::Register(source, RegisterOperation::Value))
					},

					0x5 => {       // 0x8XY5 : VX = VX - VY
						OpCode::Sub(target,
							source as SourceRegisterAddress, SubKind::SourceSubtrahend)
					},

					0x6 => {       // 0x8XY6 : VX = VY >> 1
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::RightShift))
					},

					0x7 => {       // 0x8XY7 : VY = VY - VX
						OpCode::Sub( target,
							source as SourceRegisterAddress, SubKind::TargetSubtrahend)
					}

					0xE => {       // 0x8XYE : VX = VY << 1
						OpCode::Set(target,
							SetKind::Register(source, RegisterOperation::LeftShift))
					},

					_ => {
						OpCode::UnKnown
					}

				}
			},

			0xC000...0xD000 => { // 0xCXNN : VX = (random) & NN
				OpCode::Random(target, (word & VALUE_MASK) as Mask)
			}

			_ => {
				OpCode::Unknown
			}
		}
	}
}

pub enum OpCode {
	Unknown,
	Set(RegisterAddress, SetKind),
	Add(RegisterAddress, AddKind),
	Sub(RegisterAddress, SourceRegisterAddress, SubKind),
	Random(RegisterAddress, Mask),
}

pub enum SetKind {
	Value(Byte),
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
	Value(Byte),
	Register(RegisterAddress, RegisterOperation)
}

pub enum SubKind {
	TargetSubtrahend,
	SourceSubtrahend,
}
