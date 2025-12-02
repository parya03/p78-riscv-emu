/**
 * Code to load ELF
**/

use std::fs;

pub unsafe fn load_elf(path: &String, mem: &mut [u64]) -> i32 {
    let elf_contents;
    match fs::read(path) {
        Ok(v) => elf_contents = v,
        Err(e) => {
            println!("Error reading ELF file");
            return -1;
        }
    }
    // dbg!(elf_header);
    
    let mut elf_header: [u8; 64] = [0; 64];
    elf_header.copy_from_slice(&elf_contents[0..64]);

    // Verify magic number
    // let magic_num: [u8; 4] = elf_header[0..4];
    if(elf_header[0..4] != [0x7F, 'E' as u8, 'L' as u8, 'F' as u8][..]) {
        println!("Wrong elf magic number");
        return -1;
    }
    
    let is_big_endianness = elf_header[5] - 1;
    let is_64_bit_addr = elf_header[4] - 1;
    println!("ELF is {}-bit format and {} endian", if is_64_bit_addr == 0 {"32"} else {"64"}, if is_big_endianness == 0 {"little"} else {"big"});
    
    // Check if ISA is correct
    if(elf_header[18] != 0xF3) {
        println!("This ELF is not for RISC-V!");
        return -1;
    }

    let mut entry_point: u64;
    match (is_big_endianness, is_64_bit_addr) {
        (0, 0) => entry_point = u32::from_le_bytes(match elf_header[24..28].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get entry point address!")
        }) as u64,
        (0, 1) => entry_point = u64::from_le_bytes(match elf_header[24..32].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get entry point address!")
        }) as u64,
        (1, 0) => entry_point = u32::from_be_bytes(match elf_header[24..28].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get entry point address!")
        }) as u64,
        (1, 1) => entry_point = u64::from_be_bytes(match elf_header[24..32].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get entry point address!")
        }) as u64,
        _ => panic!("Invalid elf header info"),
    }
    println!("ELF entry point: {:#016X}", entry_point);

    return 0;
}
