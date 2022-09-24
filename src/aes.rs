use std::mem::transmute;
use lazy_static::lazy_static; 

pub struct AES<const SECRET_SIZE: usize> ([u32; SECRET_SIZE]); 

lazy_static! {
    static ref S_BOX: [u8; 256] = generate_s_box(); 
}

fn generate_s_box() -> [u8; 256] {
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
            break sbox.map(Option::unwrap)
        }
    }
}

impl <const S: usize> AES<S> {
    pub fn with_key(keys: [u32; S]) -> Self {
        Self(keys)
    }
}

impl AES<4> {
    pub fn encrypt_block_with_cache(&self, input: &[u8; 16], out: &mut [u8; 16]) {
        let key: &[u8; 16] = unsafe { transmute(&self.0) }; 
        let mut result = [0u8; 16]; 
        // init transformation. 
        for i in 0..16 {
            result[i] = input[i] ^ key[i]; 
        }
        // sub bytes. 
        let sub_bytes = |bytes: &mut [u8]| {
            for i in bytes {
                *i = S_BOX[*i as usize]; 
            }
        }; 
        let shift_rows = |bytes: &mut [u8; 16]| {
            {
                let value = bytes[1]; 
                bytes[1] = bytes[5];
                bytes[5] = bytes[9]; 
                bytes[9] = bytes[13]; 
                bytes[13] = value; 
            }
            {
                let value = bytes[2]; 
                bytes[2] = bytes[10]; 
                bytes[10] = value; 
                let value = bytes[6]; 
                bytes[6] = bytes[14]; 
                bytes[14] = value; 
            }
            {
                let value = bytes[3]; 
                bytes[3] = bytes[15]; 
                bytes[15] = bytes[11]; 
                bytes[11] = bytes[7]; 
                bytes[7] = value; 
            }
        }; 
        let mix_columns = |bytes: &mut [u8; 16]| {
            let cache : [[u8; 4]; 4]= [[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]]; 
            let mut result = [0u8; 16]; 
            for r in 0..4 {
                for c in 0..4 {
                    for i in 0..4 {
                        result[r+c*4] = result[r+c*4] ^ (
                            cache[r][i].wrapping_mul(bytes[i+c*4])
                        ); 
                    }
                }
            } 
            *bytes = result; 
        }; 
        let add_round_keys = |bytes: &mut [u8; 16]| {
            let secrets = key; 
            for ele in bytes.iter_mut().zip(secrets.iter()) {
                *ele.0 ^= *ele.1; 
            }
        }; 
        sub_bytes(&mut result); 
    }
}

impl AES<6> {
    pub fn encrypt_with_bits(&self ) {

    }
}

impl AES <8> {
    pub fn encrypt_with_bits(&self ) {
        
    }
}