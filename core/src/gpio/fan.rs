use fan::FanControl;

use super::pin::Pin;

/// Simple Abstraction for Pin based GPIO
pub struct GpioFan<T: Pin>(T);

impl<T> GpioFan<T>
where
    T: Pin,
{
    /// Instantiates a new `GpioFan` on the provided `Pin`
    pub fn new(fan_pin: T) -> Self {
        GpioFan(fan_pin)
    }
}

impl<T> FanControl for GpioFan<T>
where
    T: Pin,
{
    fn turn_on(&mut self) {
        self.0.high();
    }

    fn turn_off(&mut self) {
        self.0.low();
    }

    fn is_on(&self) -> bool {
        self.0.is_high()
    }
}
