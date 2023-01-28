use crate::error::PinError;

/// Simple abstraction for Pin-based controls
/// Implement for a specific type to add support
/// `high` and `low` methods activate and deactivate the pin respectivly
pub trait Pin {
    /// type this `Pin` should be implemented for
    type Type;

    /// There are limited pins on a board, so initializing a pin should be able to fail
    /// Let the program decide what happens when it fails, return an `PinError` in this case
    fn new(fan_pin: u8) -> Result<Self::Type, PinError>;

    /// Simple method to mut self to set the appropriate pin to `high`
    fn high(&mut self);

    /// Simple method to mut self to set the appropriate pin to `low`
    fn low(&mut self);

    /// Simple check to see if the Pin is set to `high` or not.
    fn is_high(&self) -> bool;
}
