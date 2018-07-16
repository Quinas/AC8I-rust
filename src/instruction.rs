use chip::Chip8;

pub type Addr = u16;
pub type Reg = u8;

#[derive(Debug)]
pub enum OpCode {
	CLS,
	RET,
	SYS(Addr),
	JP(Addr),
	CALL(Addr),
	SE(Reg, u8),
	SNE(Reg, u8),
	SE2(Reg, Reg),
	LD(Reg, u8),
	ADD(Reg, u8),
	LD2(Reg, Reg),
	OR(Reg, Reg),
	AND(Reg, Reg),
	XOR(Reg, Reg),
	ADD2(Reg, Reg),
	SUB(Reg, Reg),
	SHR(Reg, Reg),
	SUBN(Reg, Reg),
	SHL(Reg, Reg),
	SNE2(Reg, Reg),
	LD3(Addr),
	JP2(Addr),
	RND(Reg, u8),
	DRW(Reg, Reg, u8),
	SKP(Reg),
	SKNP(Reg),
	LD4(Reg),
	LD5(Reg),
	LD6(Reg),
	LD7(Reg),
	ADD3(Reg),
	LD8(Reg),
	LD9(Reg),
	LD10(Reg),
	LD11(Reg),
}

fn shift_and_u16(code: u16, bits: u16) -> u16 {
	if bits == 0 {
		code
	} else {
		(code >> bits.trailing_zeros()) & (bits >> bits.trailing_zeros())
	}
}

fn shift_and_u8(code: u16, bits: u16) -> u8 {
	shift_and_u16(code, bits) as u8
}

pub fn fetch_opcode(chip: &Chip8) -> u16 {
	(chip.memory[chip.pc as usize] as u16) << 8 | (chip.memory[chip.pc as usize + 1] as u16)
}

pub fn decode_opcode(code: u16) -> Option<OpCode> {
	match code >> 12 {
		0 => match code {
			0x00E0 => Some(OpCode::CLS),
			0x00EE => Some(OpCode::RET),
			_ => Some(OpCode::SYS(code)),
		},
		1 => Some(OpCode::JP(shift_and_u16(code, 0x0FFF))),
		2 => Some(OpCode::CALL(shift_and_u16(code, 0x0FFF))),
		3 => Some(OpCode::SE(
			shift_and_u8(code, 0x0F00),
			shift_and_u8(code, 0x00FF),
		)),
		4 => Some(OpCode::SNE(
			shift_and_u8(code, 0x0F00),
			shift_and_u8(code, 0x00FF),
		)),
		5 => Some(OpCode::SE2(
			shift_and_u8(code, 0x0F00),
			shift_and_u8(code, 0x00F0),
		)),
		6 => Some(OpCode::LD(
			shift_and_u8(code, 0x0F00),
			shift_and_u8(code, 0x00FF),
		)),
		7 => Some(OpCode::ADD(
			shift_and_u8(code, 0x0F00),
			shift_and_u8(code, 0x00FF),
		)),
		8 => match code & 0x000F {
			0 => Some(OpCode::LD2(
				shift_and_u8(code, 0x0F00),
				shift_and_u8(code, 0x00F0),
			)),
			1 => Some(OpCode::OR(
				shift_and_u8(code, 0x0F00),
				shift_and_u8(code, 0x00F0),
			)),
			2 => Some(OpCode::AND(
				shift_and_u8(code, 0x0F00),
				shift_and_u8(code, 0x00F0),
			)),
			3 => Some(OpCode::XOR(
				shift_and_u8(code, 0x0F00),
				shift_and_u8(code, 0x00F0),
			)),
			4 => Some(OpCode::ADD2(
				shift_and_u8(code, 0x0F00),
				shift_and_u8(code, 0x00F0),
			)),
			5 => Some(OpCode::SUB(
				shift_and_u8(code, 0x0F00),
				shift_and_u8(code, 0x00F0),
			)),
			6 => Some(OpCode::SHR(
				shift_and_u8(code, 0x0F00),
				shift_and_u8(code, 0x00F0),
			)),
			7 => Some(OpCode::SUBN(
				shift_and_u8(code, 0x0F00),
				shift_and_u8(code, 0x00F0),
			)),
			0xE => Some(OpCode::SHL(
				shift_and_u8(code, 0x0F00),
				shift_and_u8(code, 0x00F0),
			)),
			_ => None,
		},
		9 => Some(OpCode::SNE2(
			shift_and_u8(code, 0x0F00),
			shift_and_u8(code, 0x00F0),
		)),
		0xA => Some(OpCode::LD3(shift_and_u16(code, 0x0FFF))),
		0xB => Some(OpCode::JP2(shift_and_u16(code, 0x0FFF))),
		0xC => Some(OpCode::RND(
			shift_and_u8(code, 0x0F00),
			shift_and_u8(code, 0x00FF),
		)),
		0xD => Some(OpCode::DRW(
			shift_and_u8(code, 0x0F00),
			shift_and_u8(code, 0x00F0),
			shift_and_u8(code, 0x000F),
		)),
		0xE => match code & 0x00FF {
			0x9E => Some(OpCode::SKP(shift_and_u8(code, 0x0F00))),
			0xA1 => Some(OpCode::SKNP(shift_and_u8(code, 0x0F00))),
			_ => None,
		},
		0xF => match code & 0x00FF {
			7 => Some(OpCode::LD4(shift_and_u8(code, 0x0F00))),
			0xA => Some(OpCode::LD5(shift_and_u8(code, 0x0F00))),
			0x15 => Some(OpCode::LD6(shift_and_u8(code, 0x0F00))),
			0x18 => Some(OpCode::LD7(shift_and_u8(code, 0x0F00))),
			0x1E => Some(OpCode::ADD3(shift_and_u8(code, 0x0F00))),
			0x29 => Some(OpCode::LD8(shift_and_u8(code, 0x0F00))),
			0x33 => Some(OpCode::LD9(shift_and_u8(code, 0x0F00))),
			0x55 => Some(OpCode::LD10(shift_and_u8(code, 0x0F00))),
			0x65 => Some(OpCode::LD11(shift_and_u8(code, 0x0F00))),
			_ => None,
		},
		_ => None,
	}
}
