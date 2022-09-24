use std::{env, fs::File, os::unix::prelude::FileExt};

use encode_encrypt::md5::Md5;

fn main() {
    let fname = env::args().skip(1).next().unwrap(); 
    let mut md5 = Md5::default(); 
    let fstream = File::open(fname).unwrap(); 
    let mut offset = 0; 
    let mut buffer = [0u8; 64]; 
    loop {
        let r = fstream.read_at(&mut buffer, offset).unwrap_or(0); 
        if r != 64 {
            let result = md5.consume_last_block(&buffer[..r]); 
            println!("MD5 val: {}", result); 
            break 
        } 
        offset += r as u64; 
        md5.consume_block(&buffer); 
    }
}