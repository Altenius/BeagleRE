// Intermediate Language

pub struct InstructionId(usize);

pub struct InstructionTree {
	nodes: Vec<Instruction>,
}

pub struct Register;

pub enum Instruction {
	SetRegister(Register, InstructionId), // Register = Expression
	Load(InstructionId), // Load from memory
	Store(InstructionId, InstructionId), // Store into memory
	Push(InstructionId),
	Pop(InstructionId),
}