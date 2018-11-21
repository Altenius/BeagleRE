// Intermediate Language

pub struct InstructionId(usize);

pub struct InstructionTree {
	nodes: Vec<Instruction>,
	root: Option<InstructionId>,
}

impl InstructionTree {
	/// Creates an empty tree
	pub fn new() -> InstructionTree {
		InstructionTree {
			nodes: Vec::new(),
			root: None,
		}
	}

	/// Adds an instruction node and returns the ID
	pub fn add_node(&mut self, instruction: Instruction) -> InstructionId {
		self.nodes.push(instruction);
		InstructionId(self.nodes.len() - 1)
	}

	/// Sets the root node from an instruction ID. This MUST be a valid ID.
	pub fn set_root(&mut self, id: InstructionId) {
		self.root = Some(id);
	}

	/// Adds an instruction node and sets it as root
	pub fn add_root(&mut self, instruction: Instruction) {
		let id = self.add_node(instruction);
		self.set_root(id);
	}
}

pub struct Register;

pub enum Instruction {
	SetRegister(Register, InstructionId), // Register = Expression
	
	Load(InstructionId), // Load from memory
	Store(InstructionId, InstructionId), // Store into memory
	Push(InstructionId),
	Pop(InstructionId),

	ConstantInt32(u32),
}