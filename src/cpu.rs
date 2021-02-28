use std::fs;
use std::io;
use byteorder::{ByteOrder, LittleEndian};
// use std::io::Cursor;
use crate::opcodes;

const NUM_REGS: usize = 8;
const MEM_LOCATIONS: usize = 0x8000;

const REG_OFFSET: u16 = 32768;
const MAX_LITERAL: u16 = 32767;
const MAX_REG: u16 = 32775;
const MODULO_C: u16 = 32768;

const debug: bool = false;

pub struct Cpu{
    pc: u16,
    reg: [u16; NUM_REGS],
    mem: [u16; MEM_LOCATIONS],
    stack: Vec<u16>,
    stdin_buf: String,
    stdin_empty: bool,
    stdin_index: usize
}

impl Cpu{
    pub fn load_program(&mut self){
        let program_file = fs::read("challenge.bin").expect("Failed to open program file");

        LittleEndian::read_u16_into(&program_file[..], &mut self.mem[..program_file.len()/2]);
        // println!("{:02X?}", self.memory);
    }

    fn get_operand(&mut self, arg: u16)->u16{
        if arg <= MAX_LITERAL{
            return arg;
        }else if arg <= MAX_REG{
            return self.reg[(arg - REG_OFFSET) as usize];
        }else{
            panic!("BAD ARG");
        }
    }

    fn get_args(&mut self, op: u16) ->(u16, u16, u16){
        match op{
            opcodes::HALT | opcodes::NOOP | opcodes::RET  => return (0, 0, 0), 
            
            opcodes::JT | opcodes:: JF => {
                // returning imm/regval , imm/regval
                let a = self.get_operand(self.mem[(self.pc + 1) as usize]);
                let b = self.get_operand(self.mem[(self.pc + 2) as usize]);
                return (a, b, 0);
            },
            opcodes::SET | opcodes::NOT | opcodes::RMEM  =>{
                // returning reg # , imm/regval
                let a = self.mem[(self.pc + 1) as usize] - REG_OFFSET;
                let b = self.get_operand(self.mem[(self.pc + 2) as usize]);
                return (a, b, 0);
            },
            opcodes::WMEM =>{
                // a is either an imm address or regval
                // tuple value 3 will indicate which
                let b = self.get_operand(self.mem[(self.pc + 2) as usize]);

                let a = self.mem[(self.pc + 1) as usize];
                if a <= MAX_LITERAL{
                    return (a, b, 0);
                }else if a <= MAX_REG{
                    return ((a - REG_OFFSET), b, 1);
                }else{
                    panic!("BAD WMEM INSTRUCTION");
                }
            },
            opcodes::PUSH  =>{
                // returning regval/imm
                let a = self.get_operand(self.mem[(self.pc + 1) as usize]);
                return (a, 0, 0);
            },
            opcodes::IN | opcodes::POP =>{
                // returning reg #
                let a = self.mem[(self.pc+1) as usize] - REG_OFFSET;
                return (a, 0, 0);
            },
            opcodes::JMP | opcodes::CALL | opcodes::OUT =>{
                // returning imm/regval
                let a = self.get_operand(self.mem[(self.pc+1) as usize]);
                return (a, 0, 0);
            },
            opcodes::EQ | opcodes::GT | opcodes::ADD | opcodes::MULT | 
            opcodes::MOD | opcodes::AND | opcodes::OR =>{
                // returning reg # imm/regval imm/regval 
                let a = self.mem[(self.pc+1) as usize] - REG_OFFSET;
                let b = self.get_operand(self.mem[(self.pc+2) as usize]);
                let c = self.get_operand(self.mem[(self.pc+3) as usize]);
                return (a, b, c);
            },
            _ => return (0, 0, 0)
        }
    }

    pub fn step(&mut self)->bool{
        let op = self.mem[self.pc as usize];
        let pc_old = self.pc;

        assert!(op < 22);

        let (a, b, c) = self.get_args(op); 
        self.pc += opcodes::INSTR_SIZE[op as usize];

        if debug{
            println!("--------------------------------------------------------------");
            println!("PC: {} | OP: {}", pc_old, opcodes::INSTR_NAMES[op as usize]);
            println!("Args: {} {} {}", a, b, c);
        }

        match op{
            opcodes::HALT  => return false, 
            opcodes::SET   => self.reg[a as usize] = b,
            opcodes::PUSH  => self.stack.push(a),
            opcodes::POP   => self.reg[a as usize] = self.stack.pop().unwrap(),
            opcodes::EQ    => self.reg[a as usize] = if b == c {1} else {0},
            opcodes::GT    => self.reg[a as usize] = if b > c {1} else {0},
            opcodes::JMP   => self.pc = a,
            opcodes::JT    => self.pc = if a != 0 {b} else {self.pc},
            opcodes::JF    => self.pc = if a == 0 {b} else {self.pc},
            opcodes::ADD   => self.reg[a as usize] = (b % MODULO_C + c % MODULO_C) % MODULO_C,
            opcodes::MULT  => {
                let result: u32 = (b as u32) * (c as u32)  % 32768;
                self.reg[a as usize] = result as u16;
            },
            opcodes::MOD   => self.reg[a as usize] = b % c,
            opcodes::AND   => self.reg[a as usize] = b & c,
            opcodes::OR    => self.reg[a as usize] = b | c,
            opcodes::NOT   => self.reg[a as usize] = !b & 0x7FFF, 
            opcodes::RMEM  => self.reg[a as usize] = self.mem[b as usize],
            opcodes::WMEM  => {
                if c == 0{
                    self.mem[a as usize] = b;
                }else{
                    self.mem[self.reg[a as usize] as usize] = b;
                }
            },
            opcodes::CALL  => {
                self.stack.push(pc_old + opcodes::INSTR_SIZE[op as usize]);
                self.pc = a;
            },
            opcodes::RET => self.pc = self.stack.pop().unwrap(),
            opcodes::OUT => {
                if !debug{
                    print!("{}", (a as u8) as char );
                }
            },
            opcodes::IN => {

                if self.stdin_empty{
                    io::stdin()
                        .read_line(&mut self.stdin_buf)
                        .expect("Failed to read line");
                    
                    self.stdin_buf = self.stdin_buf.replace('\r', "");
                    self.stdin_empty = false;
                    // self.stdin_index = 0;
                }

                let curr_char = self.stdin_buf.chars().nth(self.stdin_index).unwrap();
                self.stdin_index+=1;
                if curr_char == '\n'{
                    self.stdin_empty = true;
                }

                self.reg[a as usize] = curr_char as u16;
            },

            opcodes::NOOP => (),
            _ => return false
        }

        if debug{
            println!("Reg: {:?}, Stack: {:?}", self.reg, self.stack);
        }

        return true;
    }

    pub fn run(&mut self){
        println!("Starting execution!");
        while self.step() {
        }
        println!("Execution End");
    }

    pub fn new() -> Self{
        Cpu{stack: Vec::new(), 
            pc: 0, 
            reg: [0; NUM_REGS], 
            mem: [0; MEM_LOCATIONS],
            stdin_buf: String::new(),
            stdin_empty: true,
            stdin_index: 0}
    }
}
