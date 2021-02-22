use std::fs;
use byteorder::{ByteOrder, LittleEndian};
use std::io::Cursor;

pub struct Cpu{
    pc: u16,
    reg: [u16; 8],
    memory: [u16; 0x8000],
    stack: Vec<u16>
}

impl Cpu{
    pub fn load_program(&mut self){
        let program_file = fs::read("challenge.bin");

        let program_file = match program_file{
            Ok(file) => file,
            Err(error) => panic!("Problem opening program file: {:?}", error)
        };

        LittleEndian::read_u16_into(&program_file[..], &mut self.memory[..program_file.len()/2]);
        // println!("{:02X?}", self.memory);

    }

    pub fn new() -> Self{
        Cpu{stack: Vec::new(), 
            pc: 0, 
            reg: [0; 8], 
            memory: [0; 0x8000]}
    }
}

pub fn print_hello(){
    println!("Hello Worl");
}
