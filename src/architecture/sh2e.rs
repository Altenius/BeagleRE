use smallvec::SmallVec;
use crate::memory::Layout;
use crate::architecture::{Architecture, Token, TokenBase, Instruction};
use crate::error::{Error, Result};

use std::fmt;



/// Renesas SH2E architecture
/// Uses the Super-H instruction set
pub struct SH2E {

}



enum SuperHNibble {
	InstructionCode(u8),
	RegSrc,
	RegDest,
	Imm,
	Disp,
}



enum SuperHFormat {
	Zero(u16),	// xxxx xxxx xxxx xxxx
	N(u8, u8),	// xxxx nnnn xxxx xxxx
	M(u8, u8), 	// xxxx mmmm xxxx xxxx
	NM(u8, u8), // xxxx nnnn mmmm xxxx
	MD(u8),		// xxxx xxxx mmmm dddd
	ND4(u8), 	// xxxx xxxx nnnn dddd
	NMD(u8),	// xxxx nnnn mmmm dddd
	D(u8),		// xxxx xxxx dddd dddd
	D12(u8),	// xxxx dddd dddd dddd
	ND8(u8),	// xxxx nnnn dddd dddd
	I(u8),		// xxxx xxxx iiii iiii
	NI(u8),		// xxxx nnnn iiii iiii
}



enum Register {
	R0,
	R1,
	R2,
	R3,
	R4,
	R5,
	R6,
	R7,
	R8,
	R9,
	R10,
	R11,
	R12,
	R13,
	R14,
	R15,
	GBR,
	PC,
}

impl Register {
	fn static_str(&self) -> &'static str {
		match *self {
			Register::R0 => "R0",
			Register::R1 => "R1",
			Register::R2 => "R2",
			Register::R3 => "R3",
			Register::R4 => "R4",
			Register::R5 => "R5",
			Register::R6 => "R6",
			Register::R7 => "R7",
			Register::R8 => "R8",
			Register::R9 => "R9",
			Register::R10 => "R10",
			Register::R11 => "R11",
			Register::R12 => "R12",
			Register::R13 => "R13",
			Register::R14 => "R14",
			Register::R15 => "R15",
			Register::GBR => "GBR",
			Register::PC => "PC",
		}
	}
}

impl fmt::Display for Register {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.static_str())
	}
}


impl From<u8> for Register {
	fn from(nibble: u8) -> Register {
		match nibble {
			0 => Register::R0,
			1 => Register::R1,
			2 => Register::R2,
			3 => Register::R3,
			4 => Register::R4,
			5 => Register::R5,
			6 => Register::R6,
			7 => Register::R7,
			8 => Register::R8,
			9 => Register::R9,
			10 => Register::R10,
			11 => Register::R11,
			12 => Register::R12,
			13 => Register::R13,
			14 => Register::R14,
			15 => Register::R15,
			_ => Register::R0,
		}
	}
}



enum ArgumentType {
	Immediate,					// #imm
	DirectDestReg,				// Rn
	DirectSrcReg,				// Rm
	IndirectDestReg,			// @Rn
	IndirectSrcReg,				// @Rm
	PostIncIndirectDestReg,		// @Rn+				Post-increment indirect register
	PreDecIndirectDestReg,		// @-Rn 			Pre-decrement indirect register
	PostIncIndirectSrcReg,		// @Rm+				Post-increment indirect register
	PreDecIndirectSrcReg,		// @-Rm 			Pre-decrement indirect register
	IndirectDestRegDisp,		// @(disp:4, Rn)
	IndirectSrcRegDisp,			// @(disp:4, Rm)
	IndirectIdxDestReg,			// @(R0, Rn)
	IndirectIdxSrcReg,			// @(R0, Rm)
	IndirectGbrDisp,			// @(disp:8, GBR)
	IndirectIdxGbr,				// @(R0, GBR)
	IndirectPcDisp,				// @(disp:8/12, PC)
}



struct SuperHInstruction {
	opcode: &'static str,
	format: SuperHFormat,
	arguments: &'static [ArgumentType],
}



// Instructions
const INSTRUCTIONS: &'static [SuperHInstruction] = &[
	SuperHInstruction {opcode: "nop", format: SuperHFormat::Zero(0b1001), arguments: &[]},

	SuperHInstruction {opcode: "mov", format: SuperHFormat::NI(0b1110), arguments: &[ArgumentType::Immediate, ArgumentType::DirectDestReg]},
	SuperHInstruction {opcode: "mov.w", format: SuperHFormat::NI(0b1001), arguments: &[ArgumentType::IndirectPcDisp, ArgumentType::DirectDestReg]},
	SuperHInstruction {opcode: "mov.l", format: SuperHFormat::NI(0b1101), arguments: &[ArgumentType::IndirectPcDisp, ArgumentType::DirectDestReg]},

	SuperHInstruction {opcode: "mov", format: SuperHFormat::NM(0b0110, 0b0011), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::DirectDestReg]},
	SuperHInstruction {opcode: "mov.b", format: SuperHFormat::NM(0b0010, 0b0000), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectDestReg]},
	SuperHInstruction {opcode: "mov.w", format: SuperHFormat::NM(0b0010, 0b0001), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectDestReg]},
	SuperHInstruction {opcode: "mov.l", format: SuperHFormat::NM(0b0010, 0b0010), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectDestReg]},

	SuperHInstruction {opcode: "mov.b", format: SuperHFormat::NM(0b0110, 0b0000), arguments: &[ArgumentType::IndirectSrcReg, ArgumentType::DirectDestReg]},
	SuperHInstruction {opcode: "mov.w", format: SuperHFormat::NM(0b0110, 0b0001), arguments: &[ArgumentType::IndirectSrcReg, ArgumentType::DirectDestReg]},
	SuperHInstruction {opcode: "mov.l", format: SuperHFormat::NM(0b0110, 0b0010), arguments: &[ArgumentType::IndirectSrcReg, ArgumentType::DirectDestReg]},

	SuperHInstruction {opcode: "mov.b", format: SuperHFormat::NM(0b0010, 0b0100), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::PreDecIndirectDestReg]},
	SuperHInstruction {opcode: "mov.w", format: SuperHFormat::NM(0b0010, 0b0101), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::PreDecIndirectDestReg]},
	SuperHInstruction {opcode: "mov.l", format: SuperHFormat::NM(0b0010, 0b0110), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::PreDecIndirectDestReg]},

	SuperHInstruction {opcode: "mov.b", format: SuperHFormat::NM(0b0110, 0b0100), arguments: &[ArgumentType::PostIncIndirectSrcReg, ArgumentType::DirectDestReg]},
	SuperHInstruction {opcode: "mov.w", format: SuperHFormat::NM(0b0110, 0b0101), arguments: &[ArgumentType::PostIncIndirectSrcReg, ArgumentType::DirectDestReg]},
	SuperHInstruction {opcode: "mov.l", format: SuperHFormat::NM(0b0110, 0b0110), arguments: &[ArgumentType::PostIncIndirectSrcReg, ArgumentType::DirectDestReg]},

	SuperHInstruction {opcode: "mov.b", format: SuperHFormat::ND4(0b10000000), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectDestRegDisp]},
	SuperHInstruction {opcode: "mov.w", format: SuperHFormat::ND4(0b10000001), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectDestRegDisp]},
	SuperHInstruction {opcode: "mov.l", format: SuperHFormat::NMD(0b0001), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectDestRegDisp]},
	
	SuperHInstruction {opcode: "mov.b", format: SuperHFormat::MD(0b10000100), arguments: &[ArgumentType::IndirectSrcRegDisp, ArgumentType::DirectDestReg]}, // Rn = R0
	SuperHInstruction {opcode: "mov.w", format: SuperHFormat::MD(0b10000101), arguments: &[ArgumentType::IndirectSrcRegDisp, ArgumentType::DirectDestReg]}, // Rn = R0
	SuperHInstruction {opcode: "mov.l", format: SuperHFormat::NMD(0b0101), arguments: &[ArgumentType::IndirectSrcRegDisp, ArgumentType::DirectDestReg]},

	SuperHInstruction {opcode: "mov.b", format: SuperHFormat::NM(0b0000, 0b0100), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectIdxDestReg]},
	SuperHInstruction {opcode: "mov.w", format: SuperHFormat::NM(0b0000, 0b0101), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectIdxDestReg]},
	SuperHInstruction {opcode: "mov.l", format: SuperHFormat::NM(0b0000, 0b0110), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectIdxDestReg]},
	
	SuperHInstruction {opcode: "mov.b", format: SuperHFormat::NM(0b0000, 0b1100), arguments: &[ArgumentType::IndirectIdxSrcReg, ArgumentType::DirectDestReg]},
	SuperHInstruction {opcode: "mov.w", format: SuperHFormat::NM(0b0000, 0b1101), arguments: &[ArgumentType::IndirectIdxSrcReg, ArgumentType::DirectDestReg]},
	SuperHInstruction {opcode: "mov.l", format: SuperHFormat::NM(0b0000, 0b1110), arguments: &[ArgumentType::IndirectIdxSrcReg, ArgumentType::DirectDestReg]},

	SuperHInstruction {opcode: "mov.b", format: SuperHFormat::D(0b11000000), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectGbrDisp]}, // Rm = R0
	SuperHInstruction {opcode: "mov.w", format: SuperHFormat::D(0b11000001), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectGbrDisp]}, // Rm = R0
	SuperHInstruction {opcode: "mov.l", format: SuperHFormat::D(0b11000010), arguments: &[ArgumentType::DirectSrcReg, ArgumentType::IndirectGbrDisp]}, // Rm = R0
];



struct Nibbles {
	nibbles: [u8; 4],
}



impl Nibbles {
	#[inline(always)]
	fn nibble(&self, offset: usize) -> u8 {
		self.nibbles[offset]
	}

	#[inline(always)]
	fn byte(&self, offset: usize) -> u8 {
		(self.nibbles[offset] << 4) | self.nibbles[offset + 1]
	}

	#[inline(always)]
	fn word(&self) -> u16 {
		((self.nibbles[0] as u16) << 12) | ((self.nibbles[1] as u16) << 8) | ((self.nibbles[2] as u16) << 4) | (self.nibbles[3] as u16)
	}

	#[inline(always)]
	fn three(&self, offset: usize) -> u16 {
		((self.nibbles[offset] as u16) << 8) | ((self.nibbles[offset + 1] as u16) << 4) | (self.nibbles[offset + 2] as u16)
	}

	/* Comparison checks */
	#[inline(always)]
	fn check_nibble(&self, offset: usize, nib: u8) -> bool {
		self.nibble(offset) == nib
	}

	#[inline(always)]
	fn check_byte(&self, offset: usize, byte: u8) -> bool {
		self.byte(offset) == byte
	}

	#[inline(always)]
	fn check_word(&self, word: u16) -> bool {
		self.word() == word
	}
}




impl SuperHInstruction {
	#[inline]
	fn disassemble(&self, nibbles: &Nibbles) -> Option<SmallVec<[Token; 6]>> {
		// TODO: optimizations
		let mut source_reg = Register::R0;
		let mut dest_reg = Register::R0;
		let mut immediate: usize = 0;
		let mut displacement: usize = 0;

		match self.format {
			SuperHFormat::Zero(x) if nibbles.check_word(x) => {
				// No arguments
			},
			SuperHFormat::N(x1, x2) if nibbles.check_nibble(0, x1) && (nibbles.check_byte(2, x2)) => {
				dest_reg = Register::from(nibbles.nibble(1));
			},
			SuperHFormat::M(x1, x2) if nibbles.check_nibble(0, x1) && (nibbles.check_byte(2, x2)) => {
				source_reg = Register::from(nibbles.nibble(1));
			},
			SuperHFormat::NM(x1, x2) if nibbles.check_nibble(0, x1) && nibbles.check_nibble(3, x2) => {
				dest_reg = Register::from(nibbles.nibble(1));
				source_reg = Register::from(nibbles.nibble(2));
			},
			SuperHFormat::MD(x1) if nibbles.check_byte(0, x1) => {
				source_reg = Register::from(nibbles.nibble(2));
				displacement = nibbles.nibble(3) as usize;
			},
			SuperHFormat::ND4(x1) if nibbles.check_byte(0, x1) => {
				source_reg = Register::R0;
				dest_reg = Register::from(nibbles.nibble(2));
				displacement = nibbles.nibble(3) as usize;
			},
			SuperHFormat::NMD(x1) if nibbles.check_nibble(0, x1) => {
				dest_reg = Register::from(nibbles.nibble(1));
				source_reg = Register::from(nibbles.nibble(2));
				displacement = nibbles.nibble(3) as usize;
			},
			SuperHFormat::D(x1) if nibbles.check_byte(0, x1) => {
				displacement = nibbles.byte(2) as usize;
			},
			SuperHFormat::D12(x1) if nibbles.check_nibble(0, x1) => {
				displacement = nibbles.three(1) as usize;
			},
			SuperHFormat::ND8(x1) if nibbles.check_nibble(0, x1) => {
				dest_reg = Register::from(nibbles.nibble(1));
				displacement = nibbles.byte(2) as usize;
			},
			SuperHFormat::I(x1) if nibbles.check_byte(0, x1) => {
				immediate = nibbles.byte(2) as usize;
			},
			SuperHFormat::NI(x1) if nibbles.check_nibble(0, x1) => {
				dest_reg = Register::from(nibbles.nibble(1));
				immediate = nibbles.byte(2) as usize;
			},
			_ => { return None; },// No match
		};

		let mut tokens = SmallVec::new();

		// Push the opcode
		tokens.push(Token::new(TokenBase::Opcode(self.opcode)));

		for arg in self.arguments {
			match arg {
				ArgumentType::Immediate => {
					tokens.push(Token::new(TokenBase::Immediate(immediate)).with_prefix("#"));
				}
				ArgumentType::DirectDestReg => {
					tokens.push(Token::new(TokenBase::Register(dest_reg.static_str())))
				}
				ArgumentType::DirectSrcReg => {
					tokens.push(Token::new(TokenBase::Register(source_reg.static_str())))
				}
				ArgumentType::IndirectDestReg => {
					tokens.push(Token::new(TokenBase::Register(dest_reg.static_str())).with_prefix("@"))
				}
				ArgumentType::IndirectSrcReg => {
					tokens.push(Token::new(TokenBase::Register(source_reg.static_str())).with_prefix("@"))
				}
				ArgumentType::PostIncIndirectDestReg => {
					tokens.push(Token::new(TokenBase::Register(dest_reg.static_str())).with_prefix("@").with_suffix("+"))
				}
				ArgumentType::PostIncIndirectSrcReg => {
					tokens.push(Token::new(TokenBase::Register(source_reg.static_str())).with_prefix("@").with_suffix("+"))
				}
				ArgumentType::PreDecIndirectDestReg => {
					tokens.push(Token::new(TokenBase::Register(dest_reg.static_str())).with_prefix("@-"))
				}
				ArgumentType::PreDecIndirectSrcReg => {
					tokens.push(Token::new(TokenBase::Register(source_reg.static_str())).with_prefix("@-"))
				}
				ArgumentType::IndirectDestRegDisp => {
					// Push displacement
					tokens.push(Token::new(TokenBase::Immediate(displacement)).with_prefix("@(").with_suffix(""));
					// Push destination register
					tokens.push(Token::new(TokenBase::Register(dest_reg.static_str())).with_suffix(")"));
				}
				ArgumentType::IndirectSrcRegDisp => {
					// Push displacement
					tokens.push(Token::new(TokenBase::Immediate(displacement)).with_prefix("@(").with_suffix(""));
					// Push source register
					tokens.push(Token::new(TokenBase::Register(source_reg.static_str())).with_suffix(")"));
				}
				ArgumentType::IndirectIdxDestReg => {
					// Push index register
					tokens.push(Token::new(TokenBase::Register(Register::R0.static_str())).with_prefix("@(").with_suffix(""));
					// Push destination register
					tokens.push(Token::new(TokenBase::Register(dest_reg.static_str())).with_suffix(")"));
				}
				ArgumentType::IndirectIdxSrcReg => {
					// Push index register
					tokens.push(Token::new(TokenBase::Register(Register::R0.static_str())).with_prefix("@(").with_suffix(""));
					// Push source register
					tokens.push(Token::new(TokenBase::Register(source_reg.static_str())).with_suffix(")"));
				}
				ArgumentType::IndirectGbrDisp => {
					// Push displacement
					tokens.push(Token::new(TokenBase::Immediate(displacement)).with_prefix("@(").with_suffix(""));
					// Push GBR
					tokens.push(Token::new(TokenBase::Register(Register::GBR.static_str())).with_suffix(")"));
				}
				ArgumentType::IndirectIdxGbr => {
					// Push R0
					tokens.push(Token::new(TokenBase::Register(Register::R0.static_str())).with_prefix("@(").with_suffix(""));
					// Push GBR
					tokens.push(Token::new(TokenBase::Register(Register::GBR.static_str())).with_suffix(")"));
				}
				ArgumentType::IndirectPcDisp => {
					// Push displacement
					tokens.push(Token::new(TokenBase::Immediate(displacement)).with_prefix("@(").with_suffix(""));
					// Push PC
					tokens.push(Token::new(TokenBase::Register(Register::PC.static_str())).with_suffix(")"));
				}
				_ => {},
			}
		}

		Some(tokens)
	}
}


impl SH2E {
	pub fn new() -> SH2E {
		SH2E {}
	}
}


impl Architecture for SH2E {
	fn disassemble_single(&self, layout: &Layout, address: usize) -> Result<(Instruction, usize)> {
		let mut instr = [0; 2];
		// Read two bytes for the instruction
		let amount_read = layout.read_memory(address, &mut instr);
		if amount_read < 2 {
			return Err(Error::InvalidMemory);
		}

		// Divide the instruction into nibbles
		let nibbles = Nibbles {nibbles: [instr[0] >> 4, instr[0] & 0x0F, instr[1] >> 4, instr[1] & 0x0F]};
		
		// Match the instruction
		for i in INSTRUCTIONS {
			if let Some(tokens) = i.disassemble(&nibbles) {
				return Ok((Instruction {tokens}, 2));
			}
		}

		Err(Error::InvalidInstruction)
	}
}