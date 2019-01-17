#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt;

pub struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    pub fn new<T>(data: &'a T) -> HexSlice<'a>
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexSlice(data.as_ref())
    }
}

// can format Vec<u8> as hex string using println!("{:x}", HexSlice::new(&metadata_data.data));
impl std::fmt::LowerHex for HexSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            // Decide if you want to pad the value or have spaces inbetween, etc.
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
