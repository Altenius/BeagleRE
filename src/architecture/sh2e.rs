use smallvec::SmallVec;
use crate::memory::Layout;
use crate::architecture::{Architecture, Instruction, Token};
use crate::error::{Error, Result};

/// Renesas SH2E architecture
/// Uses the Super-H instruction set
pub struct SH2E {

}

pub struct SuperHInstruction {

}

impl Instruction for SuperHInstruction {
	fn tokens(&self) -> &[Token] {
		&[]
	}
}

impl Architecture for SH2E {
	fn disassemble_single(&self, layout: &Layout, address: usize) -> Result<(SmallVec<[Token; 6]>, usize)> {
		let mut instr = &[0; 2];

		// Read two bytes for the instruction

		Err(Error::InvalidInstruction)
	}
}