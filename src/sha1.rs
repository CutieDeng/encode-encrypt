use std::{mem::{MaybeUninit, transmute}, fmt::Display};

use byteorder::{BigEndian, ByteOrder};

const fn init_variables() -> [u32; 5] {
    [
        0x67452301, 
        0xEFCDAB89, 
        0x98BADCFE,
        0x10325476, 
        0xC3D2E1F0, 
    ]
}

#[derive(Clone)]
pub struct Sha1 (pub [u32; 5], u64);

impl Default for Sha1 {
   fn default() -> Self {
       Self::new()
   } 
} 

impl Sha1 {
    pub const fn new() -> Self {
        Sha1(init_variables(), 0)
    }

    pub fn clear(&mut self) {
        self.0 = init_variables() 
    }

    pub fn hash_all(&self, values: &[u8]) -> [u32; 5] {
        let mut cache: [MaybeUninit<u32>; 80] = MaybeUninit::uninit_array(); 
        self.hash_all_with_cache(values, unsafe { transmute(&mut cache) } )
    }
    
    pub fn hash_all_with_cache(&self, values: &[u8], cache: &mut [u32; 80]) -> [u32; 5] {
        let mut this = self.clone(); 
        // let r = values.as_ptr_range();
        let mut values = values.array_chunks::<64>(); 
        while let Some(block) = values.next() {
            this.hash_block_with_cache(block, cache) 
        }
        let remain = values.remainder(); 
        this.hash_last_block_with_cache(remain, cache) 
    }

    pub fn hash_last_block_with_cache(&self, values: &[u8], cache: &mut [u32; 80]) -> [u32; 5] {
        let mut new_array: [u8; 64] = [0; 64]; 
        assert! (values.len() < 63); 
        let origin_length = ((self.1 * 64) + values.len() as u64) * 8; 
        let mut i = 0; 
        while i < values.len() {
            new_array[i] = values[i]; 
            i += 1;
        }
        new_array[i] = 0x80u8; 
        i += 1; 
        let mut this = self.clone(); 
        if i >= 56 {
            this.hash_block_with_cache(&new_array, cache); 
            new_array = [0; 64]; 
        }
        BigEndian::write_u64(&mut new_array[56..], origin_length); 
        this.hash_block_with_cache(&new_array, cache); 
        this.0 
    }

    #[deprecated]
    pub fn hash_last_block_directly_with_cache(&self, values: &[u8], _cache: &mut [u32; 80]) -> [u32; 5] {
        let mut new_array: [MaybeUninit<u32>; 16] = MaybeUninit::uninit_array(); 
        let mut i = 0; 
        while i < values.len() / 4 { 
            new_array[i].write( 
                unsafe { values.as_ptr().cast::<u32>().add(i).read() }
            ); 
            i += 1; 
        }
        new_array[i].write(0);
        let mut index = 0; 
        while index < values.len() % 4 {
            new_array[i].write (
            unsafe {
                    new_array[i].assume_init_read() | ((values[i * 4 + index] as u32) << (index * 8))
                }
            ); 
            index += 1; 
        }
        new_array[i].write(
        unsafe {
                new_array[i].assume_init_read() | ((0x80u32) << (index * 8))
            }
        ); 
        while i < 16 {
            new_array[i].write(0); 
            i += 1; 
        }
        // let mut this = self.clone(); 
        // this.hash_block_with_cache(, cache)
        todo!()
    }

    pub fn hash_block_with_cache(&mut self, values: &[u8; 64], cache: &mut [u32; 80]) {
        BigEndian::read_u32_into(values, &mut cache[..16]);
        self.1 = self.1 + 1; 
        for index in 16..80 { 
            let t = cache[index - 3] ^ cache[index - 8] ^ cache[index - 14] 
                ^ cache[index - 16]; 
            cache[index] = t.rotate_left(1); 
        }
        let mut this = self.clone(); 
        let mut k; 
        for i in 0..80usize {
            let f = 
                if i < 20usize {
                    k = 0x5A827999; 
                    (this.0[1] & this.0[2]) | ((! this.0[1]) & this.0[3])
                } else if i < 40usize {
                    k = 0x6ED9EBA1; 
                    this.0[1] ^ this.0[2] ^ this.0[3] 
                } else if i < 60usize {
                    k = 0x8F1BBCDC; 
                    ( this.0[1] & this.0[2] ) | (this.0[1] & this.0[3]) | (this.0[2] & this.0[3]) 
                } else if i < 80usize {
                    k = 0xCA62C1D6; 
                    this.0[1] ^ this.0[2] ^ this.0[3] 
                } else {
                    unreachable!() 
                }; 
            let temp = this.0[0].rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(this.0[4])
                .wrapping_add(k)
                .wrapping_add(cache[i]);
            this.0[4] = this.0[3]; 
            this.0[3] = this.0[2]; 
            this.0[2] = this.0[1].rotate_left(30); 
            this.0[1] = this.0[0]; 
            this.0[0] = temp; 
        }
        for i in 0..5 {
            self.0[i] = self.0[i].wrapping_add(this.0[i])
        }
    }

    pub fn result_display <'a> (&'a self) -> Sha1ResultDisplay<'a> {
        Sha1ResultDisplay(&self.0)
    }
}

pub struct Sha1ResultDisplay <'a> (pub &'a [u32; 5]); 

impl <'a> Display for Sha1ResultDisplay <'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x}{:08x}{:08x}{:08x}{:08x}", self.0[0], self.0[1], self.0[2], self.0[3], self.0[4])
    }
}