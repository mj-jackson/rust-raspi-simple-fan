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
    pub fn new() -> Self {
        Fan {
            control: Box::new(GpioFan)
        }
    }
}

pub trait FanControl {
    fn turn_on(&self) -> Result<(), FanError>;
    fn turn_off(&self) -> Result<(), FanError>;
    fn is_on(&self) -> bool;
}

pub struct GpioFan;
impl GpioFan {
    const GPIO_PIN: u8 = 14;
    fn get_fan_pin(&self) -> Result<OutputPin, Error> {
        let pin = Gpio::new()?.get(GpioFan::GPIO_PIN)?.into_output();
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
