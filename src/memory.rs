use bitflags::bitflags;
use std::cmp;

/// Group of `[Section]`s
pub struct Layout {
	sections: Vec<Section>,
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
	address: usize,
	memory: Vec<u8>,
}

impl Section {
	pub fn len(&self) -> usize {
		self.memory.len()
	}

	pub fn from_raw(address: usize, raw: Vec<u8>) -> Section {
		Section {
			address,
			memory: raw,
		}
	}
}

impl Layout {
	pub fn new() -> Layout {
		Layout {
			sections: Vec::new(),
		}
	}

	pub fn add_section(&mut self, section: Section) -> &mut Section {
		let offset = section.address;
		// Insert the section in order
		for (i, entry) in self.sections.iter().enumerate() {
			if entry.address > offset {
				self.sections.insert(i, section);
				return self.sections.get_mut(i).unwrap();
			}
		}
		// All entries were less than offset or the array is empty; append to the end
		self.sections.push(section);
		self.sections.last_mut().unwrap()
	}

	/// Reads memory at the address into the buffer. Returns the amount written to the buffer
	pub fn read_memory(&self, start_address: usize, start_buffer: &mut [u8]) -> usize {
		// Find the section containing the address
		let start_section = self.sections.iter().enumerate().find(|(_, section)| section.address <= start_address);
		if start_section.is_none() {
			return 0;
		}
		let (mut section_id, mut section) = start_section.unwrap();

		let mut buffer = start_buffer;
		let mut address = start_address;

		while !buffer.is_empty() {
			// Find the offset within the section
			let offset = address - section.address;
			let to_read = cmp::min(section.memory.len() - offset, buffer.len());
			buffer[..to_read].clone_from_slice(&section.memory[offset..offset + to_read]);

			// Increment pointer
			buffer = &mut buffer[to_read..];
			address += to_read;

			// Get new section
			let section_opt = &self.sections[section_id..].iter().enumerate().find(|(_, section)| section.address <= address);
			if section_opt.is_none() {
				break;
			}

			let (section_id_l, section_l) = section_opt.unwrap();
			section_id = section_id_l;
			section = section_l;
		}

		address - start_address
	}

	/// Finds the section containing the address. If found, returns the section.
	/// If not found, returns None.
	pub fn get_section_at(&self, address: usize) -> Option<&Section> {
		let section = self.sections.iter().find(|section| section.address <= address)?;

		Some(section)
	}
}