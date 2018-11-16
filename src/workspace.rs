use crate::memory::Layout;
use crate::architecture::Architecture;

pub struct Workspace {
	pub memory: Layout,
	pub arch: Box<Architecture>,
}

impl Workspace {
	pub fn new(arch: Box<Architecture>) -> Workspace {
		Workspace {
			memory: Layout::new(),
			arch,
		}
	}
}