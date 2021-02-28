//use cpu::{Cpu};
use synacor_challenge::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.load_program();
    cpu.run();
}
