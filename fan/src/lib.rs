enum FanStatus {
    Off,
    On,
}

/// Simple Fan trait which only takes a FanControl
pub struct Fan<'a, T: FanControl> {
    control: &'a mut T,
    status: FanStatus,
}

/// Provide new method to instantiate Fan struct
impl<'a, T: FanControl> Fan<'a, T> {
    pub fn new(control: &'a mut T) -> Self {
        let status = match control.is_on() {
            true => FanStatus::On,
            false => FanStatus::Off,
        };

        Fan { control, status }
    }

    pub fn on(&mut self) {
        self.control.turn_on();
        self.status = FanStatus::On;
    }

    pub fn off(&mut self) {
        self.control.turn_off();
        self.status = FanStatus::Off;
    }

    pub fn is_on(&self) -> bool {
        match self.status {
            FanStatus::On => true,
            FanStatus::Off => false,
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
