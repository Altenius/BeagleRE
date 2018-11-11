use std::result;

#[derive(Debug)]
pub enum Error {
	InvalidInstruction,
}

pub type Result<T> = result::Result<T, Error>;