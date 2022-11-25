/// Simple trait for providing the CPU temperature
/// Implement this trait and return the temperature as an
/// `unsigned 8 bit integer` in degree `Celcius`.
pub trait CpuTempProvider {
    fn get_cpu_temp(&self) -> u8;
}

/// The CPU in regards of the temperature is represented here.
/// ### Arguments
/// * `provider` - A boxed instance of a CpuTempProvider Trait implementation
/// * `target_temp` - The temperature at which the CPU state is "too_hot"
/// * `hysteresis` - target_temp minus hysteresis is needed for the CPU to count as cool_enough again
pub struct Cpu<'a> {
    provider: &'a dyn CpuTempProvider,
    target_temp: u8,
    hysteresis: u8,
}
impl <'a> Cpu<'a> {

    /// Provide the monitored temperature and the provider of the CPU temperature
    pub fn new(target_temp: u8, provider: &'a dyn CpuTempProvider) -> Cpu {
        Cpu {
            provider,
            target_temp,
            hysteresis: 5
        }
    }

    /// Simple check to see if the `current temperature` is
    /// greater than `target temperature`
    pub fn too_hot(&self) -> bool {
        self.get_temp() > self.target_temp
    }

    /// Check to see if the `current temperature` is
    /// smaller than the `target temperature` minus `hysteresis`.
    /// This happens in order to have a buffer zone.
    /// Otherwise the fan could be in a state where it starts
    /// and stops all the time.
    pub fn cool_enough(&self) -> bool {
        let hysteresis_temp = self.target_temp - self.hysteresis;

        self.get_temp() <= hysteresis_temp
    }

    /// Get the `current temperature`.
    pub fn get_temp(&self) -> u8 {
        self.provider.get_cpu_temp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockCpuTemp;
    impl CpuTempProvider for MockCpuTemp {
        fn get_cpu_temp(&self) -> u8 {
            45u8
        }
    }

    struct TooHotMockCpuTemp;
    impl CpuTempProvider for TooHotMockCpuTemp {
        fn get_cpu_temp(&self) -> u8 {
            60u8
        }
    }

    #[test]
    fn return_string_without_whitespaces_or_linebreaks() {
        let cpu = Cpu {
            provider: &MockCpuTemp,
            target_temp: 55u8,
            hysteresis: 5
        };

        assert_eq!(45u8, cpu.get_temp());
    }

    #[test]
    fn is_not_hysteresis() {
        let cpu = Cpu {
            provider: &MockCpuTemp,
            target_temp: 55u8,
            hysteresis: 5
        };

        assert!(cpu.cool_enough());
    }

    #[test]
    fn is_too_hot() {
        let cpu = Cpu {
            provider: &TooHotMockCpuTemp,
            target_temp: 55u8,
            hysteresis: 5
        };

        assert!(cpu.too_hot());
    }
}