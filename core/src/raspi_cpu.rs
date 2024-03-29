use cpu::CpuTempProvider;
use std::{fs, path::Path};

/// The CPU temperature
pub struct RaspiCpuTemp;

impl CpuTempProvider for RaspiCpuTemp {
    /// Reading the cpu temperature for Raspberry Pi
    fn get_cpu_temp(&self) -> u8 {
        let cpu_temp_path = Path::new("/sys/class/thermal/thermal_zone0/temp");
        let val = fs::read_to_string(cpu_temp_path).expect("File should be readable");

        match val.trim().parse::<u32>() {
            Ok(temp) => (temp / 1000)
                .try_into()
                .expect("Input should be parsable into u32"),
            Err(_) => 0,
        }
    }
}
