/// Simple trait for providing the CPU temperature
pub trait CpuTempProvider {
    fn get_cpu_temp(&self) -> String;
}

/// The CPU in regards of the temperature is represented here.
/// # Arguments
/// * `provider` - A boxed instance of a CpuTempProvider Trait implementation
/// * `target_temp` - The temperature at which the CPU state is "too_hot"
/// * `hysteresis` - target_temp minus hysteresis is needed for the CPU to count as cool_enough again
pub struct Cpu {
    provider: Box<dyn CpuTempProvider>,
    target_temp: u8,
    hysteresis: u8,
}
impl Cpu {

    /// Right now only the `target_temp` is needed, `provider` and `hysteresis` is hard coded for now
    pub fn new(target_temp: u8, temp_prov: Box<dyn CpuTempProvider>) -> Cpu {
        Cpu {
            provider: temp_prov,
            target_temp,
            hysteresis: 5
        }
    }

    pub fn too_hot(&self) -> bool {
        self.get_temp() > self.target_temp.into()
    }

    pub fn cool_enough(&self) -> bool {
        let hysteresis_temp = self.target_temp - self.hysteresis;

        self.get_temp() <= hysteresis_temp.into()
    }

    pub fn get_temp(&self) -> u32 {
        match self.temp_string().parse::<u32>() {
            Ok(temp) => temp / 1000,
            Err(_) => 0
        }
    }

    fn temp_string(&self) -> String {
        self.provider.get_cpu_temp().trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockCpuTemp;
    impl CpuTempProvider for MockCpuTemp {
        fn get_cpu_temp(&self) -> String {
            "
             45000 
            ".to_string()
        }
    }

    struct TooHotMockCpuTemp;
    impl CpuTempProvider for TooHotMockCpuTemp {
        fn get_cpu_temp(&self) -> String {
            "60000".to_string()
        }
    }

    #[test]
    fn return_string_without_whitespaces_or_linebreaks() {
        let cpu = Cpu {
            provider: Box::new(MockCpuTemp),
            target_temp: 55u8,
            hysteresis: 5
        };

        assert_eq!("45000".to_string(), cpu.temp_string());
    }

    #[test]
    fn is_not_hysteresis() {
        let cpu = Cpu {
            provider: Box::new(MockCpuTemp),
            target_temp: 55u8,
            hysteresis: 5
        };

        assert!(cpu.cool_enough());
    }

    #[test]
    fn is_too_hot() {
        let cpu = Cpu {
            provider: Box::new(TooHotMockCpuTemp),
            target_temp: 55u8,
            hysteresis: 5
        };

        assert!(cpu.too_hot());
    }
}