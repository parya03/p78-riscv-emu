use std::env;
use std::fs;

mod elfloader;
mod memory;
use memory::Memory;

const PC_REG: usize = 32; // Reg 32 is PC
// const PC: &i64 = &regs[PC_REG];

// const MEM_SIZE: usize = 8192; // In bytes

// static mut regs: [i64; 33] = [0; 33]; // Register file, 32 64-bit regs

// static mut MEMORY: [u64; (MEM_SIZE / 8)] = [0; (MEM_SIZE / 8)];

fn main() {
    // println!("Hello, world!");
    
    let args: Vec<String> = env::args().collect();
    // dbg!(args); 
    let elf_path = &args[1];
    println!("ELF Path: {elf_path}");
    
    // Allocate memory
    let mut memory = Memory::create();
    unsafe {
        elfloader::load_elf(elf_path, &mut memory);
    }
}
