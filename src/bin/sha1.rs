use std::io::stdin;

use encode_encrypt::sha1::{Sha1, Sha1ResultDisplay};

fn main() {
    let sha1 = Sha1::default(); 
    let mut input = String::new(); 
    stdin().read_line(&mut input).unwrap(); 
    let result = sha1.hash_all(input.trim_end().as_bytes());
    // println!("{:08x}{:08x}{:08x}{:08x}{:08x}", result[0], result[1], result[2], result[3], result[4]); 
    let show = Sha1ResultDisplay(&result); 
    println!("{}", show); 
}