use byteorder::{ByteOrder, LittleEndian};
use std::fs;
use std::fmt::Write;
use synacor_challenge::opcodes;

const REG_OFFSET: u16 = 32768;
const MAX_LITERAL: u16 = 32767;
const MAX_REG: u16 = 32775;

fn load_program()->Vec<u16>{
    let program_file = fs::read("challenge.bin").expect("Failed to open program file");
    let mut program_16 = vec![0; program_file.len()/2];

    LittleEndian::read_u16_into(&program_file[..], &mut program_16);

    return program_16.to_vec();
}

fn process_arg(arg: u16) -> String{
    if arg <= MAX_LITERAL{
        // literal value
        return format!(" #{}", arg).to_string();
    }else if arg <= MAX_REG{
        return format!(" R{}", arg - REG_OFFSET).to_string();
    }else{
        panic!("BAD INSTRUCTION ARG");
    }
}

fn main(){
    let program_bin = load_program();
    // println!("{:?}", program_bin);

    let mut program_asm = String::new();

    let mut i: usize = 0;

    while i < program_bin.len(){
        let op = program_bin[i];
        i+=1;

        if op > 21{ // if op doesn't represent an opcode, its a data literal
            assert!(op <= MAX_LITERAL);
            writeln!(&mut program_asm, "D{}", op).unwrap();
            continue;
        }

        let mut curr_instruction = String::from(opcodes::INSTR_NAMES[op as usize]);

        let curr_instr_len = opcodes::INSTR_SIZE[op as usize];

        for _ in 1..curr_instr_len{
            let arg = program_bin[i];
            let arg_string = process_arg(arg);
            curr_instruction.push_str(&arg_string);
            i+=1;
        }
        // println!("{}", curr_instruction);
        writeln!(&mut program_asm, "{}", curr_instruction).unwrap();
    }

    fs::write("disassembled.txt", program_asm).unwrap();
}
