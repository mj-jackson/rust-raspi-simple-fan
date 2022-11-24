use std::{fs, path::Path};

/// The CPU temperature
pub struct RaspiCpuTemp;
impl cpu::CpuTempProvider for RaspiCpuTemp {
    /// A simple function to return the CPU temperature as a String
    fn get_cpu_temp(&self) -> String {
        let cpu_temp_path = Path::new("/sys/class/thermal/thermal_zone0/temp");
        fs::read_to_string(cpu_temp_path).expect("File should be readable")
    }
}