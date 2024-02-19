use crate::errors::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct FieldElement {
    pub num: usize,
    pub f_prime: usize,
}

impl FieldElement {
    pub fn new(num: usize, prime: usize) -> anyhow::Result<Self> {
        if num > prime {
            Error::Custom(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }

        Ok(Self {
            num,
            f_prime: prime,
        })
    }
}

impl Display for FieldElement {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
