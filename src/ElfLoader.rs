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
    
    let is_big_endianness: bool = elf_header[5] == 2;
    let is_64_bit_addr: bool = elf_header[4] == 2;
    println!("ELF is {}-bit format and {} endian", if is_64_bit_addr == false {"32"} else {"64"}, if is_big_endianness == false {"little"} else {"big"});
    
    // Check if ISA is correct
    if(elf_header[18] != 0xF3) {
        println!("This ELF is not for RISC-V!");
        return -1;
    }

    let mut entry_point: u64;
    match (is_big_endianness, is_64_bit_addr) {
        (false, false) => entry_point = u32::from_le_bytes(match elf_header[24..28].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get entry point address!")
        }) as u64,
        (false, true) => entry_point = u64::from_le_bytes(match elf_header[24..32].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get entry point address!")
        }) as u64,
        (true, false) => entry_point = u32::from_be_bytes(match elf_header[24..28].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get entry point address!")
        }) as u64,
        (true, true) => entry_point = u64::from_be_bytes(match elf_header[24..32].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get entry point address!")
        }) as u64,
        _ => panic!("Invalid elf header info"),
    }
    println!("ELF entry point: {:#016X}", entry_point);

    // ELF fields to be decoded
    // In 32-bit mode, we automatically cast to 64-bit
    let e_phoff: u64;
    let e_shoff: u64;
    let e_flags: u32;
    let e_phentsize: u16;
    let e_phnum: u16;
    let e_shentsize: u16;
    let e_shnum: u16;
    let e_shstrndx: u16;

    if(is_64_bit_addr) {
        e_phoff = u64::from_be_bytes(match elf_header[0x20..0x28].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_phoff address!")
        }) as u64;

        e_shoff = u64::from_be_bytes(match elf_header[0x28..0x30].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_shoff address!")
        }) as u64;

        e_flags = u32::from_be_bytes(match elf_header[0x30..0x34].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_flags address!")
        }) as u32;

        e_phentsize = u16::from_be_bytes(match elf_header[0x36..0x38].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_phentsize address!")
        }) as u16;

        e_phnum = u16::from_be_bytes(match elf_header[0x38..0x3A].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_phnum address!")
        }) as u16;

        e_shentsize = u16::from_be_bytes(match elf_header[0x3A..0x3C].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_shentsize address!")
        }) as u16;

        e_shnum = u16::from_be_bytes(match elf_header[0x3C..0x3E].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_shnum address!")
        }) as u16;

        e_shstrndx = u16::from_be_bytes(match elf_header[0x3E..0x40].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_shstrndx address!")
        }) as u16;
    }
    else {
        e_phoff = u32::from_be_bytes(match elf_header[0x1C..0x20].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_phoff address!")
        }) as u64;

        e_shoff = u32::from_be_bytes(match elf_header[0x20..0x24].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_shoff address!")
        }) as u64;

        e_flags = u32::from_be_bytes(match elf_header[0x24..0x28].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_flags address!")
        }) as u32;

        e_phentsize = u16::from_be_bytes(match elf_header[0x2A..0x2C].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_phentsize address!")
        }) as u16;

        e_phnum = u16::from_be_bytes(match elf_header[0x2C..0x2E].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_phnum address!")
        }) as u16;

        e_shentsize = u16::from_be_bytes(match elf_header[0x2E..0x30].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_shentsize address!")
        }) as u16;

        e_shnum = u16::from_be_bytes(match elf_header[0x30..0x32].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_shnum address!")
        }) as u16;

        e_shstrndx = u16::from_be_bytes(match elf_header[0x32..0x34].try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Can't get e_shstrndx address!")
        }) as u16;
    }
    
    println!("ELF Info:");
    println!("Program Header Offset: {:#X}, Number of Entries: {}", e_phoff, e_phnum);
    println!("Section Header Offset: {:#X}, Number of Entries: {}", e_shoff, e_shnum);

    return 0;
}
