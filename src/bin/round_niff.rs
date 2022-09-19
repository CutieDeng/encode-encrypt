fn main() {
    let mut p: u8 = 1; 
    let mut q: u8 = 1; 
    let mut sbox = [None; 256]; 
    sbox[0] = Some ( 0x63 ); 
    loop {
        p = p ^ (p << 1) ^ if p & 0x80 != 0 { 0x1B } else { 0 };
        q ^= q << 1; 
        q ^= q << 2; 
        q ^= q << 4; 
        q ^= if q & 0x80 != 0 { 0x09 } else { 0 }; 
        // assert_eq!( p.wrapping_mul(q), 1, "p={p}, but q={q}, and p * q = {}", p.wrapping_mul(q)); 
        assert! (sbox[p as usize].is_none()); 
        let x_formed = q ^ q.rotate_left(1) ^ q.rotate_left(2) ^ q.rotate_left(3) ^ q.rotate_left(4); 
        sbox[ p as usize ] = Some ( x_formed ^ 0x63 ); 
        // println! ("p = {p:02x}, q = {q:02x}, xformed = {x_formed:02x}. "); 
        if p == 1 {
            break 
        }
    }
    for index in 0..sbox.len() {
        if (index + 1) % 16 == 0 {
            println!("{:02x}", sbox[index].unwrap()); 
        } else {
            print!("{:02x} ", sbox[index].unwrap()); 
        }
    }
}