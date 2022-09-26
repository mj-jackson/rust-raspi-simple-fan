use std::fs;

trait CpuTempProvider {
    fn get_cpu_temp(&self) -> String;
}

struct CpuTemp;
impl CpuTempProvider for CpuTemp {
    fn get_cpu_temp(&self) -> String {
        let cpu_temp_file = "/sys/class/thermal/thermal_zone0/temp";
        fs::read_to_string(cpu_temp_file).expect("File should be readable")
    }
}

pub struct Cpu {
    provider: Box<dyn CpuTempProvider>,
    hysteresis: u8,
}
impl Cpu {
    const THRESHOLD: u8 = 55;

    pub fn new() -> Cpu {
        Cpu {
            provider: Box::new(CpuTemp),
            hysteresis: 5
        }
    }

    pub fn too_hot(&self) -> bool {
        self.get_temp() > Cpu::THRESHOLD.into()
    }

    pub fn cool_enough(&self) -> bool {
        let hysteresis_temp = Cpu::THRESHOLD - self.hysteresis;

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
            hysteresis: 5
        };

        assert_eq!("45000".to_string(), cpu.temp_string());
    }

    #[test]
    fn is_not_hysteresis() {
        let cpu = Cpu {
            provider: Box::new(MockCpuTemp),
            hysteresis: 5
        };

        assert!(cpu.cool_enough());
    }

    #[test]
    fn is_too_hot() {
        let cpu = Cpu {
            provider: Box::new(TooHotMockCpuTemp),
            hysteresis: 5
        };

        assert!(cpu.too_hot());
    }
}