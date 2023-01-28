mod cli;
mod error;
mod gpio;
mod raspi_cpu;
mod rppal_pin;

use std::str::FromStr;
use std::thread;
use std::time::Duration;

use cpu::Cpu;
use error::PinError;
use fan::Fan;
use raspi_cpu::RaspiCpuTemp;
use rppal::gpio::OutputPin;

use crate::gpio::fan::GpioFan;
use crate::gpio::pin::Pin;

const TEMP_THRESHOLD: u8 = 55;
const GPIO_PIN: u8 = 14;
const SLEEP_INT: u64 = 3000;

fn main() -> Result<(), PinError> {
    let cpu_temp: u8 = get_val_or_def("-t", TEMP_THRESHOLD);
    let fan_pin: u8 = get_val_or_def("-p", GPIO_PIN);
    let sleep_millis: u64 = get_val_or_def("-i", SLEEP_INT);

    let pin = OutputPin::new(fan_pin)?;
    let mut fan_control = GpioFan::new(pin);
    let mut fan = Fan::new(&mut fan_control);
    let cpu = Cpu::new(cpu_temp, &RaspiCpuTemp);
    loop {
        if fan.is_on() {
            if cpu.cool_enough() {
                fan.off();
            }
        } else if cpu.too_hot() {
            fan.on();
        }

        thread::sleep(Duration::from_millis(sleep_millis));
    }
}

fn get_val_or_def<T: ToString + FromStr>(arg: &str, def: T) -> T {
    let val_opt = cli::get_argument_value(arg);

    match val_opt {
        Some(val) => val.parse().unwrap_or(def),
        None => def,
    }
}
