mod fan;
mod cpu;

use std::error::Error;
use std::thread;
use std::time::Duration;

use fan::Fan;
use cpu::Cpu;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let cpu = Cpu::new();
        let fan = Fan::new();

        if fan.control.is_on() {
            if cpu.cool_enough() {
                fan.control.turn_off()?
            }
        } else {
            if cpu.too_hot() {
                fan.control.turn_on()?
            }
        }

        println!("Loop done!");
        thread::sleep(Duration::from_millis(3000));
    }
}