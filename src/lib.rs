#![feature(never_type)]
#![feature(array_chunks, core_intrinsics, 
    maybe_uninit_uninit_array)]

pub trait Encode {
    type Output : Default; 
    type Error; 
    fn encode(&mut self, input: &[u8]) -> Result<Self::Output, Self::Error> {
        let mut result = Default::default(); 
        self.encode_in(input, &mut result)?; 
        Ok(result)
    }
    fn encode_in(&mut self, input: &[u8], output: &mut Self::Output) -> Result<(), Self::Error>;
}

pub mod base64;
pub mod sha1; 