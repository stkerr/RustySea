#[derive(Debug)]
pub enum Error { NotImplementedError, HexParsingError }

#[derive(Eq)]
#[derive(PartialEq)]
pub struct BigInt {
    pub negative:bool,
    pub data: Vec<u64>
}

pub mod display;
pub mod operators;
pub mod utilities;
