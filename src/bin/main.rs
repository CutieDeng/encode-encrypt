use code_support::{base64, Encode};

fn main() {
    // println!("Hello, world!");
    let mut input = String::new(); 
    std::io::stdin().read_line(&mut input).unwrap();
    let mut encoder = base64::Base64; 
    // let mut encoder = base64::Base64Decoder; 
    let result = encoder.encode(input.trim_end().as_bytes()).unwrap(); 
    let output = String::from_utf8(result).unwrap(); 
    println!("{}", output); 
}
