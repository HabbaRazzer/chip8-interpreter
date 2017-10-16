use system::{System, Word, Byte, RegisterAddress};

const VALUE_MASK: Word = 0b0000_0000_1111_1111;
const REGISTER_MASK: Word = 0b0000_1111_0000_0000;

const TYPE_MASK: Word = 0b0000_0000_0000_1111;
const LEFT_MASK: Word = 0b0000_1111_0000_0000;
const RIGHT_MASK: Word = 0b0000_0000_1111_0000;

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
}
