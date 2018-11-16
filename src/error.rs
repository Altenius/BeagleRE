use std::result;

#[derive(Debug)]
pub enum Error {
	InvalidInstruction,
	InvalidMemory, // Invalid address or there not enough memory for the operation
}

pub type Result<T> = result::Result<T, Error>;