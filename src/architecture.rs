use crate::error::Result;
use crate::memory::Layout;
use smallvec::SmallVec;

pub mod sh2e;

pub struct Disassembly {

}

pub trait Architecture {
	/// Disassembles a single instruction or returns an error.
	/// Returns the amount of bytes used.
	fn disassemble_single(&self, layout: &Layout, address: usize) -> Result<(SmallVec<[Token; 6]>, usize)>;

	// Analyzer
}

pub enum Token {
	Opcode,
	Intermediate,
}

pub trait Instruction {
	/// Returns tokens used for displaying the instruction
	fn tokens(&self) -> &[Token];
}