pub mod cpu;
pub mod errors;
pub mod mem;

pub use crate::cpu::CPU;
pub use crate::errors::{ CpuError, SysInfoError };
pub use crate::mem::RAM;

pub struct SysInfo {
    pub cpu_per: u8,
    pub cpu_flat: u64,
    cpu_driver: CPU,
    
    pub ram_total: u64,
    pub ram_free: u64,
    pub ram_used_per: u8, 
    ram_driver: RAM,    
}

impl SysInfo {
    pub fn new() -> Result<Self, SysInfoError> {
        let cpu = CPU::new()?;
        let cpu_per = cpu.cpu_per;
        let cpu_flat = cpu.cpu_flat;

        let ram = RAM::new()?;
        let ram_free = ram.free;
        let ram_total = ram.total;
        let ram_used_per = ((ram.used * 100) / ram.total) as u8;

        return Ok(Self {
            cpu_per,
            cpu_flat,
            cpu_driver: cpu,
            ram_total,
            ram_free,
            ram_used_per,
            ram_driver: ram,
        });
    } 
    pub fn update(&mut self) -> Result<bool, SysInfoError> {
        let cpu_changed = self.cpu_driver.update()?;
        let ram_changed = self.ram_driver.update()?;

        self.cpu_per = self.cpu_driver.cpu_per;
        self.cpu_flat = self.cpu_driver.cpu_flat;
        
        self.ram_total = self.ram_driver.total;
        self.ram_free = self.ram_driver.free;
        self.ram_used_per = ((self.ram_driver.used * 100) / self.ram_driver.total) as u8;

        return Ok(cpu_changed || ram_changed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_sys_info_lifecycle() {
        let mut system = SysInfo::new().expect("Failed to initialize SysInfo engine");
        
        assert_eq!(system.cpu_per, 0); 
        assert!(system.ram_total > 0, "RAM total should be greater than 0");

        sleep(Duration::from_millis(200));

        let update_result = system.update();
        assert!(update_result.is_ok(), "Update loop threw an unexpected error");
        
        println!("Test Run Success! CPU: {}%, RAM Used: {}%", system.cpu_per, system.ram_used_per);
    }
}
