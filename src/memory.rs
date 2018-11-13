use bitflags::bitflags;
use std::cmp;

/// Group of `[Section]`s
pub struct Layout {
	sections: Vec<(usize, Section)>,
}

bitflags! {
	pub struct SectionFlags : u32 {
		/* Code section flags */
		const Read = 0b1;
		const Write = 0b10;
		const Execute = 0b100;

		/* Types */
		const Data = 0b1000;
		const Code = 0b10000;

		const ReadWrite = Self::Read.bits | Self::Write.bits;
	}
}

/// Contiguous section of memory
pub struct Section {
	memory: Vec<u8>,
}

impl Section {
	pub fn len(&self) -> usize {
		self.memory.len()
	}

	pub fn from_raw(raw: Vec<u8>) -> Section {
		Section {
			memory: raw,
		}
	}
}

impl Layout {
	pub fn add_section(&mut self, offset: usize, section: Section) -> &mut Section {
		// Insert the section in order
		for (i, entry) in self.sections.iter().enumerate() {
			if entry.0 > offset {
				self.sections.insert(i, (offset, section));
				return &mut self.sections.get_mut(i).unwrap().1;
			}
		}
		// All entries were less than offset or the array is empty; append to the end
		self.sections.push((offset, section));
		&mut self.sections.last_mut().unwrap().1
	}

	/// Reads memory at the address into the buffer. Returns the amount written to the buffer
	pub fn read_memory(&self, start_address: usize, start_buffer: &mut [u8]) -> usize {
		// Find the section containing the address
		let start_section = self.sections.iter().enumerate().find(|(_, (sec_address, _section))| *sec_address <= start_address);
		if start_section.is_none() {
			return 0;
		}
		let (mut section_id, mut section) = start_section.unwrap();

		let mut buffer = start_buffer;
		let mut address = start_address;

		while !buffer.is_empty() {
			// Find the offset within the section
			let offset = address - section.0;
			let to_read = cmp::min(section.1.memory.len() - offset, buffer.len());
			buffer[..to_read].clone_from_slice(&section.1.memory[offset..to_read]);

			
		}

		0
	}
}