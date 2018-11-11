use bitflags::bitflags;

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
		for (i, entry) in self.sections.iter().enumerate() {
			if entry.0 > offset {
				self.sections.insert(i, (offset, section));
				return &mut self.sections.get_mut(i).unwrap().1;
			}
		}
		// All entries were less than offset or none exist; append to the end
		self.sections.push((offset, section));
		&mut self.sections.last_mut().unwrap().1
	}
}