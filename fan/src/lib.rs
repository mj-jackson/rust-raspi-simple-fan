/// Simple Fan trait which only takes a FanControl
pub struct Fan<'a, T: FanControl> {
    pub control: &'a mut T
}

/// Provide new method to instantiate Fan struct
impl <'a, T: FanControl> Fan<'a, T> {
    pub fn new(control: &'a mut T) -> Self {
        Fan {
            control
        }
    }
}

/// Basic FanControl struct to control fan
/// and check current status
pub trait FanControl {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn is_on(&self) -> bool;
}
