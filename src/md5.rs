use std::{mem::transmute, ptr::copy_nonoverlapping, fmt::Display};

use byteorder::{LittleEndian, ByteOrder};
use lazy_static::lazy_static; 

#[derive(Clone)]
pub struct Md5 (pub [u32; 4], u64); 

lazy_static! {
    static ref K_ARRAY : [u32; 64] = generate_karray(); 
    static ref S_ARRAY : [u32; 64] = generate_sarray(); 
}

const fn init_variables() -> [u32; 4] {
    [
        0x67452301, 
        0xefcdab89, 
        0x98badcfe, 
        0x10325476, 
    ]
}

impl Md5 {
    pub const fn new () -> Self {
        Md5( init_variables(), 0 )
    }
}

impl const Default for Md5 {
    fn default() -> Self {
        Self::new() 
    }
}

fn generate_karray() -> [u32; 64] {
    let mut result = [0u32; 64]; 
    const MULTIPLIER: f64 = (1u64 << 32) as f64; 
    for i in 0..64 {
        let tmp = ((i+1) as f64).sin().abs() * MULTIPLIER;
        result[i] = tmp.floor() as u32; 
    }
    result 
}

fn generate_sarray() -> [u32; 64] {
    let mut result = [0u32; 64]; 
    let mut r = result.array_chunks_mut::<4>();
    let mut i = 0; 
    while let Some(v) = r.next() {
        *v = 
        match i/4 {
            0 => [7, 12, 17, 22], 
            1 => [5, 9, 14, 20], 
            2 => [4, 11, 16, 23], 
            3 => [6, 10, 15, 21], 
            _ => unreachable!(),
        }; 
        i += 1;  
    }
    result 
}

impl Md5 {
    pub fn clear(&mut self) {
        self.0 = init_variables() 
    }

    pub fn consume_block(&mut self, block: &[u8; 64]) {
        let words: &[u32; 16] = unsafe { transmute(block) }; 
        let mut this = self.clone(); 
        for i in 0..64 {
            let f; 
            let g; 
            match i {
                0..16 => {
                    f = (this.0[1] & this.0[2]) | ((! this.0[1]) & this.0[3]); 
                    g = i; 
                }
                16..32 => {
                    f = (this.0[1] & this.0[3]) | ((this.0[2]) & (!this.0[3]));  
                    g = (i * 5 + 1) % 16; 
                }
                32..48 => {
                    f = this.0[1] ^ this.0[2] ^ this.0[3]; 
                    g = (3 * i + 5) % 16; 
                }
                48..64 => {
                    f = this.0[2] ^ (this.0[1] | (! this.0[3])); 
                    g = (i * 7) % 16; 
                }
                _ => unreachable!(), 
            }
            let f = f.wrapping_add(this.0[0])
            .wrapping_add(K_ARRAY[i])
            .wrapping_add(words[g]); 
            this.0[0] = this.0[3]; 
            this.0[3] = this.0[2]; 
            this.0[2] = this.0[1]; 
            this.0[1] = this.0[1].wrapping_add(f.rotate_left(S_ARRAY[i])); 
        }
        for i in 0..4 {
            self.0[i] = self.0[i].wrapping_add(this.0[i]); 
        }
        self.1 += 1; 
    }

    pub fn consume_last_block(&self, block: &[u8]) -> [u32; 4] {
        let mut actual_block = [0u8; 64];
        assert! (block.len() < 64); 
        let content_length = (self.1 * 64 + block.len() as u64) * 8; 
        unsafe {
            copy_nonoverlapping(block.as_ptr(), actual_block.as_mut_ptr(), block.len());
        }
        actual_block[block.len()] = 0x80; 
        let mut this = self.clone(); 
        if block.len() >= 56 {
            this.consume_block(&actual_block); 
            actual_block = [0u8; 64]; 
        }
        LittleEndian::write_u64(&mut actual_block[56..], content_length); 
        this.consume_block(&actual_block); 
        this.0 
    }

    pub fn consume_blocks(&self, blocks: &[u8]) -> [u32; 4] {
        let mut block_arrays = blocks.array_chunks::<64>();
        let mut this = self.clone(); 
        while let Some (block) = block_arrays.next() {
            this.consume_block(block); 
        }
        this.consume_last_block(block_arrays.remainder())
    }
}

pub struct Md5ResultDisplay<'a> (pub &'a [u32; 4]); 

impl <'a> Display for Md5ResultDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! (f, "{:02x}{:02x}{:02x}{:02x}", self.0[0], self.0[1], self.0[2], self.0[3]) 
    }
}

impl Md5 {
    pub fn display(&self) -> Md5ResultDisplay {
        Md5ResultDisplay(&self.0)
    }
}