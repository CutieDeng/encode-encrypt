use std::{env, fs::File, os::unix::prelude::FileExt};

use code_support::sha1::{Sha1, Sha1ResultDisplay};

fn main() {
    // let mut fname = String::new(); 
    // stdin().read_line(&mut fname).unwrap(); 
    let fname = env::args().skip(1).take(1).next().unwrap();
    let f = File::open(fname).unwrap();
    let mut offset: usize = 0; 
    // while offset < 
    let mut result = Sha1::default(); 
    let mut c = [0u8; 64]; 
    let mut cache = [0u32; 80]; 
    loop {
        let r = f.read_at(&mut c, offset as u64 ).unwrap_or(0); 
        offset += r; 
        if r == 64 {
            result.hash_block_with_cache(&c, &mut cache); 
        } else {
            let result = result.hash_last_block_with_cache(&c[..r], &mut cache); 
            let result = Sha1ResultDisplay(&result); 
            println!("{result}"); 
            break 
        }
    }
}