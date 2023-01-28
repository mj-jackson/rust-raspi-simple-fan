use rppal::gpio::{Gpio, OutputPin};

use crate::{error::PinError, gpio::pin::Pin};

impl Pin for OutputPin {
    type Type = OutputPin;
    fn new(fan_pin: u8) -> Result<Self::Type, PinError> {
        Ok(Gpio::new()?.get(fan_pin)?.into_output())
    }
    fn high(&mut self) {
        self.set_high();
    }
    fn low(&mut self) {
        self.set_low();
    }
    fn is_high(&self) -> bool {
        self.is_set_high()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn no_pin_panic() {
        let _pin = OutputPin::new(123u8);
    }
}
