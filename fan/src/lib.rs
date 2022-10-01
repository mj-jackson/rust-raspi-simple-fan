use std::{fmt, error};

use rppal::gpio::{Gpio, OutputPin, Error};

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

pub struct GpioFan(OutputPin);
impl GpioFan {
    pub fn new(fan_pin: u8) -> Result<Self, Error> {
        let pin = Gpio::new()?.get(fan_pin)?.into_output();

        Ok(GpioFan(pin))
    }
}
impl FanControl for GpioFan {
    fn turn_on(&mut self) {
        self.0.set_high();
    }

    fn turn_off(&mut self) {
        self.0.set_low();
    }

    fn is_on(&self) -> bool {
        self.0.is_set_high()
    }
}
