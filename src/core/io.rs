use std::fmt::{Debug, Display};
use crate::conversion_error;
use super::errors::Error;
//use crate::{make_error, core::errors::Error};


#[derive(Clone, PartialEq)]
pub struct BinaryUnit {
    data: Vec<u8>
}
impl Display for BinaryUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "len={}: {:?}", self.data.len(), &self.data)
    }
}
impl Debug for BinaryUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} byte(s) of data", self.data.len())
    }
}
impl Default for BinaryUnit {
    fn default() -> Self {
        Self {
            data: vec![]
        }
    }
}
impl From<Vec<u8>> for BinaryUnit {
        fn from(value: Vec<u8>) -> Self {
            Self {
                data: value
            }
        }
    }
impl BinaryUnit {
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn expose(&self) -> &[u8] {
        &self.data
    }
}
impl TryInto<i8> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<i8, Self::Error> {
        if self.len() != 1 {
            return Err(conversion_error!("expected 1 byte, got {}", self.len()));
        }

        Ok( i8::from_ne_bytes([ self.data[0] ]) )
    }
}
impl TryInto<i16> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<i16, Self::Error> {
        if self.len() != 2 {
            return Err(conversion_error!("expected 2 bytes, got {}", self.len()));
        }

        Ok( i16::from_ne_bytes([ self.data[0], self.data[1] ]) )
    }
}
impl TryInto<i32> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<i32, Self::Error> {
        if self.len() != 4 {
            return Err(conversion_error!("expected 4 bytes, got {}", self.len()));
        }

        Ok( i32::from_ne_bytes([ self.data[0], self.data[1], self.data[2], self.data[3] ]) )
    }
}
impl TryInto<i64> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<i64, Self::Error> {
        if self.len() != 8 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( i64::from_ne_bytes([ self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7] ]) )
    }
}
impl TryInto<i128> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<i128, Self::Error> {
        if self.len() != 16 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        let mut extracted: [u8; 16] = [0; 16];
        for i in 0..15 {
            extracted[i] = self.data[i];
        }

        Ok( i128::from_ne_bytes(extracted) )
    }
}
impl TryInto<u8> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<u8, Self::Error> {
        if self.len() != 1 {
            return Err(conversion_error!("expected 1 byte, got {}", self.len()));
        }

        Ok( u8::from_ne_bytes([ self.data[0] ]) )
    }
}
impl TryInto<u16> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<u16, Self::Error> {
        if self.len() != 2 {
            return Err(conversion_error!("expected 2 bytes, got {}", self.len()));
        }

        Ok( u16::from_ne_bytes([ self.data[0], self.data[1] ]) )
    }
}
impl TryInto<u32> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<u32, Self::Error> {
        if self.len() != 4 {
            return Err(conversion_error!("expected 4 bytes, got {}", self.len()));
        }

        Ok( u32::from_ne_bytes([ self.data[0], self.data[1], self.data[2], self.data[3] ]) )
    }
}
impl TryInto<u64> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<u64, Self::Error> {
        if self.len() != 8 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( u64::from_ne_bytes([ self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7] ]) )
    }
}
impl TryInto<u128> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<u128, Self::Error> {
        if self.len() != 16 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        let mut extracted: [u8; 16] = [0; 16];
        for i in 0..15 {
            extracted[i] = self.data[i];
        }

        Ok( u128::from_ne_bytes(extracted) )
    }
}
impl TryInto<f32> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<f32, Self::Error> {
        if self.len() != 4 {
            return Err(conversion_error!("expected 4 bytes, got {}", self.len()));
        }

        Ok( f32::from_ne_bytes([ self.data[0], self.data[1], self.data[2], self.data[3] ]) )
    }
}
impl TryInto<f64> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<f64, Self::Error> {
        if self.len() != 8 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( f64::from_ne_bytes([ self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7] ]) )
    }
}
impl TryInto<usize> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<usize, Self::Error> {
        if self.len() != 8 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( usize::from_ne_bytes([ self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7] ]) )
    }
}
impl TryInto<isize> for BinaryUnit {
    type Error = Error;
    fn try_into(self) -> Result<isize, Self::Error> {
        if self.len() != 8 {
            return Err(conversion_error!("expected 8 bytes, got {}", self.len()));
        }

        Ok( isize::from_ne_bytes([ self.data[0], self.data[1], self.data[2], self.data[3], self.data[4], self.data[5], self.data[6], self.data[7] ]) )
    }
}

impl Into<BinaryUnit> for i8 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for i16 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for i32 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for i64 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for i128 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for u8 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for u16 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for u32 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for u64 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for u128 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for f32 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for f64 {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for isize {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}
impl Into<BinaryUnit> for usize {
    fn into(self) -> BinaryUnit {
        BinaryUnit::from(self.to_ne_bytes().to_vec())
    }
}

#[macro_export]
macro_rules! binary_unit {
    ($x:expr) => {
        {
            let tmp: BinaryUnit = $x.into();
            tmp
        }
    }
}

#[test]
fn test_into_binary_unit() {
    let x = 43;
    let x_binary = binary_unit!(x);
    let dx: Result<i32, _> = x_binary.try_into();
    assert_eq!(dx.unwrap(), x);
}