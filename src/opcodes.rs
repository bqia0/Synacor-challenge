pub const HALT :u16 = 0;
pub const SET  :u16 = 1;
pub const PUSH :u16 = 2;
pub const POP  :u16 = 3;
pub const EQ   :u16 = 4;
pub const GT   :u16 = 5;
pub const JMP  :u16 = 6;
pub const JT   :u16 = 7;
pub const JF   :u16 = 8;
pub const ADD  :u16 = 9;
pub const MULT :u16 = 10;
pub const MOD  :u16 = 11;
pub const AND  :u16 = 12;
pub const OR   :u16 = 13;
pub const NOT  :u16 = 14;
pub const RMEM :u16 = 15;
pub const WMEM :u16 = 16;
pub const CALL :u16 = 17;
pub const RET  :u16 = 18;
pub const OUT  :u16 = 19;
pub const IN   :u16 = 20;
pub const NOOP :u16 = 21;

pub const INSTR_SIZE: [u16; 22] =[1, 3, 2, 2, 4, 4, 2, 3, 3, 4, 4, 4, 4, 4, 3, 3, 3, 2, 1, 2, 2, 1];

pub const INSTR_NAMES: [&str; 22] = [
    "HALT",
    "SET",
    "PUSH",
    "POP",
    "EQ",
    "GT",
    "JMP",
    "JT",
    "JF",
    "ADD",
    "MULT",
    "MOD",
    "AND",
    "OR",
    "NOT",
    "RMEM",
    "WMEM",
    "CALL",
    "RET",
    "OUT",
    "IN",
    "NOOP"
];

