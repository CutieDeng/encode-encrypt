fn main() {
    for r in 0..16 {
        for c in 0..16 {
            let this_byte : u8 = r * 0x10 | c; 
            let result = this_byte 
                ^ this_byte.rotate_left(1) 
                ^ this_byte.rotate_left(2) 
                ^ this_byte.rotate_left(3)
                ^ this_byte.rotate_left(4) 
                ^ 0x63; 
            // let result = this_byte as u32 * 31 % 257 ; 
            // let result = (result ^ 99) as u8; 
            print!("{this_byte:02x}:{result:02x} "); 
        }
        println!(); 
    }
}