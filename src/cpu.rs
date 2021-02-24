use std::fs;
use byteorder::{ByteOrder, LittleEndian};
use std::io::Cursor;

const NUM_REGS: usize = 8;
const MEM_LOCATIONS: usize = 0x8000;

pub struct Cpu{
    pc: u16,
    reg: [u16; NUM_REGS],
    memory: [u16; MEM_LOCATIONS],
    stack: Vec<u16>
}

impl Cpu{
    pub fn load_program(&mut self){
        let program_file = fs::read("challenge.bin").expect("Failed to open program file");

        LittleEndian::read_u16_into(&program_file[..], &mut self.memory[..program_file.len()/2]);
        // println!("{:02X?}", self.memory);
    }

    pub fn new() -> Self{
        Cpu{stack: Vec::new(), 
            pc: 0, 
            reg: [0; NUM_REGS], 
            memory: [0; MEM_LOCATIONS]}
    }
}
