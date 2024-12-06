use std::fmt::{Debug, Display};

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}
impl ToBytes for i8 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for i16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for i64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for i128 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for u16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for u64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for u128 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for f32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for f64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for isize {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}
impl ToBytes for usize {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}


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
impl From<&dyn ToBytes> for BinaryUnit {
    fn from(value: &dyn ToBytes) -> Self {
        Self {
            data: value.to_bytes()
        }
    }
}
impl BinaryUnit {
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn expose(&self) -> &Vec<u8> {
        &self.data
    }
}