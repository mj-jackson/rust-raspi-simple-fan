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
    fn turn_on(&self) -> Result<(), FanError>;
    fn turn_off(&self) -> Result<(), FanError>;
    fn is_on(&self) -> bool;
}

pub struct GpioFan(u8);
impl GpioFan {
    pub fn new (fan_pin: u8) -> Self {
        GpioFan(fan_pin)
    }

    fn get_fan_pin(&self) -> Result<OutputPin, Error> {
        let mut pin = Gpio::new()?.get(self.0)?.into_output_low();
        pin.set_reset_on_drop(false);

        Ok(pin)
    }
}
impl FanControl for GpioFan {
    fn turn_on(&self) -> Result<(), FanError> {
        match self.get_fan_pin() {
            Ok(mut pin) => Ok(pin.set_high()),
            Err(_) => Err(FanError)
        }
    }

    fn turn_off(&self) -> Result<(), FanError> {
        match self.get_fan_pin() {
            Ok(mut pin) => Ok(pin.set_low()),
            Err(_) => Err(FanError)
        }
    }

    fn is_on(&self) -> bool {
        match self.get_fan_pin() {
            Ok(pin) => pin.is_set_high(),
            Err(_) => false
        }
    }
}
