use rppal::gpio::{Gpio, OutputPin, Error};
use fan::FanControl;

pub struct GpioFan(OutputPin);
impl GpioFan {

    /// Creates a new GPIO output pin which then can be controlled.
    /// 
    /// ```should_panic
    /// let _pin = GpioFan::new(123u8);
    /// ```
    /// Can only create a valid output pin if there is a GPIO pin with the same number.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn no_pin_panic() {
        let _pin = GpioFan::new(123u8);
    }
}