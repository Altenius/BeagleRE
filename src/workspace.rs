use crate::memory::Layout;
use crate::architecture::Architecture;

pub struct Workspace {
	memory: Layout,
	arch: Box<Architecture>,
}