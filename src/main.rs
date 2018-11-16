pub mod architecture;
pub mod error;
pub mod memory;
pub mod workspace;
pub mod il;

fn main() {
    let mut ws = workspace::Workspace::new(Box::new(architecture::sh2e::SH2E::new()));

    ws.memory.add_section(memory::Section::from_raw(0, include_bytes!("60E0FB00.bin").to_vec()));

    let (instruction, size) = ws.arch.disassemble_single(&ws.memory, 0x836).unwrap();

    println!("{}", instruction);
}
