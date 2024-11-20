use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ZyppError {
    details: String
}

impl ZyppError {
    pub fn new(msg: &str) -> ZyppError {
        ZyppError{details: msg.to_string()}
    }
}

impl fmt::Display for ZyppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ZyppError {
    fn description(&self) -> &str {
        &self.details
    }
}