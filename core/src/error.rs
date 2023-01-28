#[derive(Debug)]
pub enum PinError {
    PinNotFound,
}

impl From<rppal::gpio::Error> for PinError {
    fn from(_value: rppal::gpio::Error) -> Self {
        PinError::PinNotFound
    }
}
