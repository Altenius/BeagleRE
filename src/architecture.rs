use crate::error::Result;
use crate::memory::Layout;
use smallvec::SmallVec;

use std::fmt;


pub mod sh2e;



pub struct Disassembly {

}


#[derive(Debug)]
pub struct Instruction {
	tokens: SmallVec<[Token; 6]>,
}


impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for (i, token) in self.tokens.iter().enumerate() {
			if i > 1 {
				write!(f, ", ")?;
			} else if i != 0 {
				write!(f, " ")?;
			}
			match token.base {
				TokenBase::Opcode(op) => {
					write!(f, "{}{}{}", token.prefix, op, token.suffix)?;
				}
				TokenBase::Immediate(num) => {
					write!(f, "{}{}{}", token.prefix, num, token.suffix)?;
				}
				TokenBase::Register(reg) => {
					write!(f, "{}{}{}", token.prefix, reg, token.suffix)?;
				}
			};
		}
		Ok(())
	}
}



pub trait Architecture {
	/// Disassembles a single instruction or returns an error.
	/// Returns the amount of bytes used.
	fn disassemble_single(&self, layout: &Layout, address: usize) -> Result<(Instruction, usize)>;

	// Analyzer
}



#[derive(Debug)]
pub enum TokenBase {
	Opcode(&'static str),
	Immediate(usize),
	Register(&'static str),
}



#[derive(Debug)]
pub struct Token {
	prefix: &'static str,
	suffix: &'static str,
	base: TokenBase,
}

impl Token {
	pub fn new(base: TokenBase) -> Token {
		Token {
			prefix: "",
			suffix: "",
			base,
		}
	}

	pub fn with_prefix(mut self, prefix: &'static str) -> Token {
		self.prefix = prefix;
		self
	}

	pub fn with_suffix(mut self, suffix: &'static str) -> Token {
		self.suffix = suffix;
		self
	}
}