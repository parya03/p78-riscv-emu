/**
 * Implementation of memory for emulator
 *
 * Conceptually, this works similar to virtual memory paging in a real system.
 * However, since we're (currently) only targeting userspace programs,
 * the mechanics and implmenetation of VM and paging in Risc-V is not super relevant.
 * For us, this is mostly just used to map the (not necessarily contiguous) virtual addresses a 
 * userspace program expects to a contiguous block of memory (array) that we allocate and control.
**/

use std::env;

const MEMORY_SIZE_BYTES: usize = 65536;
const PAGE_SIZE_BYTES: usize = 0x0FFF; // 4096 bytes per page/frame

const L1_PT_MASK: u64 = 0x0000000000FFF000;
const L2_PT_MASK: u64 = 0x0000000FFF000000;
const fn l1_pt_addr(addr: u64) -> usize {
    return ((addr & L1_PT_MASK) >> 12) as usize;
}
const fn l2_pt_addr(addr: u64) -> usize {
    return ((addr & L2_PT_MASK) >> 24) as usize;
}

// Memory organized using multi-level paging, similar to a real system
struct PTE {
    present: bool,
    addr: u16
}

// Create PTE object from its 32-bit representation
impl From<u32> for PTE {
    fn from(item: u32) -> Self {
        PTE {
            present: if item & 0x70000000 > 0 {true} else {false}, // Present bit is MSB
            addr: (item & 0x0FFF) as u16, // Last 12 bits
        }
    }
}

impl From<PTE> for u32 {
    fn from(item: PTE) -> Self {
        return ((if item.present {1} else {0}) << 31) + (item.addr & 0x0FFF) as u32;
    }
}

pub struct Memory {
    pt_l1: [u32; PAGE_SIZE_BYTES/4], // Bottom-most level
    pt_l2: [u32; PAGE_SIZE_BYTES/4], // Currently top-most level
    
    mem_array: [u8; MEMORY_SIZE_BYTES],
    mem_frame_bitmap: [u8; MEMORY_SIZE_BYTES/PAGE_SIZE_BYTES / 8],
}

impl Memory {
    // For a given 64-bit address, we are currently doing a 2-level page table based on:
    // 0x0000000AAABBBXXX
    // Where 0xAAA is the offset in the top level PT and 0xBBB is the offset in the lower-level PT
    // and XXX is the frame offset
    
    // Create from pre-allocated memory
    pub fn create_prealloc(pt_l1_array_in: [u32; PAGE_SIZE_BYTES/4], pt_l2_array_in: [u32; PAGE_SIZE_BYTES/4], mem_array_in: [u8; MEMORY_SIZE_BYTES], mem_frame_bitmap_array_in: [u8; MEMORY_SIZE_BYTES/PAGE_SIZE_BYTES / 8]) -> Memory {
        Memory {
            pt_l1: pt_l1_array_in,
            pt_l2: pt_l2_array_in,

            mem_array: mem_array_in,
            mem_frame_bitmap: mem_frame_bitmap_array_in,
        }
    }

    // Dynamically allocate
    pub fn create() -> Memory {
        Memory {
            pt_l1: *Box::new([0; PAGE_SIZE_BYTES/4]),
            pt_l2: *Box::new([0; PAGE_SIZE_BYTES/4]),

            mem_array: *Box::new([0; MEMORY_SIZE_BYTES]),
            mem_frame_bitmap: *Box::new([0; MEMORY_SIZE_BYTES/PAGE_SIZE_BYTES / 8]),
        }
    }

    pub fn allocate_page(&mut self, addr: u64) {
        let mut frame_num: u16 = 0xFFFF; // Big initial number

        // Find the first frame that is free
        'outer: for (i, byte) in self.mem_frame_bitmap.iter_mut().enumerate() {
            for bit in 0..8 {
                if(*byte & (0x1 << bit) == 0) {
                    // Found
                    frame_num = ((i as u16) * 8) + bit as u16;
                    *byte |= (0x1 << bit);
                    break 'outer;
                }
            }
        }

        if(frame_num == 0xFFFF) {
            panic!("No more allocable pages!");
        }

        self.pt_l1[l1_pt_addr(addr)] = u32::from(PTE {
            present: true,
            addr: frame_num,
        });
        
        self.pt_l2[l2_pt_addr(addr)] = u32::from(PTE {
            present: true,
            addr: l1_pt_addr(addr) as u16,
        });
    }
}
