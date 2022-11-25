mod cli;
mod gpio_fan;
mod raspi_cpu;

use std::error::Error;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use cpu::Cpu;
use fan::{Fan, FanControl};
use gpio_fan::GpioFan;
use raspi_cpu::RaspiCpuTemp;

const TEMP_THRESHOLD: u8 = 55;
const GPIO_PIN: u8 = 14;
const SLEEP_INT: u64 = 3000;

fn main() -> Result<(), Box<dyn Error>> {
    let cpu_temp: u8 = get_val_or_def("-t", TEMP_THRESHOLD);
    let fan_pin: u8 = get_val_or_def("-p", GPIO_PIN);
    let sleep_millis: u64 = get_val_or_def("-i", SLEEP_INT);
    
    let mut fan_control = GpioFan::new(fan_pin)?;
    let fan = Fan::new(&mut fan_control);
    let cpu = Cpu::new(cpu_temp, &RaspiCpuTemp);
    loop {

        if fan.control.is_on() {
            if cpu.cool_enough() {
                fan.control.turn_off();
            }
        } else if cpu.too_hot() {
            fan.control.turn_on();
        }

        thread::sleep(Duration::from_millis(sleep_millis));
    }
}

fn get_val_or_def<T: ToString + FromStr>(arg: &str, def: T) -> T {
    let val_opt = cli::get_argument_value(arg);
    
    match val_opt {
        Some(val) => val.parse().unwrap_or(def),
        None => def
    }
}
