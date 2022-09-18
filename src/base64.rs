use std::{mem::MaybeUninit, slice::ArrayChunks};

use crate::Encode;

pub struct Base64; 

impl Encode for Base64 {
    type Output = Vec<u8>; 
    type Error = !;

    fn encode_in(&mut self, input: &[u8], output: &mut Self::Output) -> Result<(), Self::Error> {
        let mut chunks = input.array_chunks::<3>(); 
        let guess_size = match chunks.size_hint().1 {
            None => {
                ( input.len() + 2 ) / 3
            }
            Some(val) => {
                val + 1
            }
        }; 
        let result = output; 
        result.reserve_exact(guess_size * 4); 
        while let Some(chunk) = chunks.next() {
            result.push(chunk[0] >> 2); 
            result.push(((chunk[0] & 0b11) << 4) | (chunk[1] >> 4)); 
            result.push(((chunk[1] & 0b1111) << 2) | (chunk[2] >> 6)); 
            result.push(chunk[2] & 0b111111); 
        }
        let remainder = chunks.remainder(); 
        if remainder.len() != 0 {
            let mut cache: [MaybeUninit<u8>; 3] = unsafe { MaybeUninit::uninit().assume_init() } ;
            let mut i = 0; 
            while i < remainder.len() { 
                cache[i].write(remainder[i]); 
                i += 1; 
            }
            unsafe {
                match i {
                    1 => {
                        result.push(cache[0].assume_init_read() >> 2); 
                        result.push((cache[0].assume_init_read() & 0b11) << 4);
                        result.push(64); 
                        result.push(64); 
                    }
                    2 => {
                        result.push(cache[0].assume_init_read() >> 2); 
                        result.push(((cache[0].assume_init_read() & 0b11) << 4) | (cache[1].assume_init_read() >> 4));
                        result.push((cache[1].assume_init_read() & 0b1111) << 2); 
                        result.push(64); 
                    }
                    _ => unreachable!() 
                }
            }
        }
        for v in result {
            *v = transform(*v);
        }
        Ok(())
    }
}

#[inline]
fn transform (input: u8) -> u8 {
    assert!( input <= 64 ); 
    if input < 26 {
        'A' as u8 + ( input ) 
    } else if input < 52 {
        'a' as u8 + ( input - 26 )
    } else if input < 62 {
        '0' as u8 + ( input - 52 )
    } else if input == 62 {
        '+' as u8
    } else if input == 63 {
        '/' as u8
    } else if input == 64 {
        '=' as u8
    } else {
        // unsafe { hint::unreachable_unchecked() }
        unreachable!() 
    }
}

pub struct Base64Decoder;


#[derive(Debug)]
pub enum Base64DecodeError {
    InvalidCharacter {
        index: usize, 
        identifier: char, 
    }
}


fn decode_transform(input: u8) -> Option<u8> {
    if input >= 'A' as u8 && input <= 'Z' as u8 {
        Some (input - 'A' as u8)
    } else if input >= 'a' as u8 && input <= 'z' as u8 {
        Some (input - 'a' as u8 + 26) 
    } else if input >= '0' as u8 && input <= '9' as u8 {
        Some (input - '0' as u8 + 52) 
    } else if input == '+' as u8 {
        Some (62) 
    } else if input == '/' as u8 {
        Some (63) 
    } else {
        None 
    }
}

impl Encode for Base64Decoder {

    type Output = Vec<u8>; 

    type Error = Base64DecodeError;

    fn encode_in(&mut self, input: &[u8], output: &mut Self::Output) -> Result<(), Self::Error> {
        let chunks: ArrayChunks<_, 4> = input.array_chunks();
        let remain = chunks.remainder(); 
        assert_eq ! (remain.len(), 0); 
        let mut index = 0usize;
        let mut cache: [MaybeUninit<u8>; 4] = MaybeUninit::uninit_array(); 
        for chunk in chunks {
            // check in. 
            let mut v = 0usize; 
            'inner_loop: 
            while v < 4 {
                if chunk[v] == '=' as u8 {
                    break 'inner_loop 
                }
                let decode_id = decode_transform(chunk[v]); 
                match decode_id {
                    None => 
                        return Err ( Base64DecodeError::InvalidCharacter { index: index * 4 + v , identifier: chunk[v].into() }), 
                    Some (o) => 
                        { cache[v].write(o); }
                }
                v += 1; 
            }
            match v {
                4 => unsafe {
                    output.push((cache[0].assume_init() << 2) | ((cache[1].assume_init() >> 4))); 
                    output.push((cache[1].assume_init() << 4) | (cache[2].assume_init() >> 2)); 
                    output.push((cache[2].assume_init() << 6) | (cache[3].assume_init())); 
                }
                3 => unsafe {
                    output.push((cache[0].assume_init() << 2) | ((cache[1].assume_init() >> 4))); 
                    output.push((cache[1].assume_init() << 4) | (cache[2].assume_init() >> 2)); 
                }
                2 => unsafe {
                    output.push((cache[0].assume_init() << 2) | ((cache[1].assume_init() >> 4))); 
                }
                _ => unreachable!()
            }
            index += 1; 
        }
        Ok(())
    } 
}