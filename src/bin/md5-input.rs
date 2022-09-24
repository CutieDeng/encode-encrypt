use std::io::stdin;

use encode_encrypt::md5::Md5;

fn main() {
    let mut input = String::new(); 
    stdin().read_line(&mut input).unwrap(); 
    let m = Md5::new();
    let buffer = input.trim_end().as_bytes(); 
    let result = m.consume_blocks(buffer);  
    println!("read bytes: {}. \n{}", buffer.len(), result); 
}