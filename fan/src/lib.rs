use std::{fmt, error};

#[derive(Debug, Clone)]
pub struct FanError;
impl fmt::Display for FanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error controlling the fan!")
    }
}
impl error::Error for FanError {}

pub struct Fan {
    pub control: Box<dyn FanControl>
}
impl Fan {
    pub fn new(control: Box<dyn FanControl>) -> Self {
        Fan {
            control
        }
    }
}

pub trait FanControl {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn is_on(&self) -> bool;
}
