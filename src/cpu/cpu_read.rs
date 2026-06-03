use scrtypes::file_expansion::smart_files::read_lines;
use crate::errors::CpuError;
use crate::cpu::parse_cpu::parse_cpu;

pub struct CPU {
    pub cpu_per: u8,
    pub cpu_flat: u64,
    baseline_total: u64,
    baseline_idle: u64,
}

impl CPU {
    pub fn new() -> Result<Self, CpuError> {
        let (btotal, bidle) = get_raw()?;
        
        return Ok(Self {
            cpu_per: 0,
            cpu_flat: 0,
            baseline_total: btotal,
            baseline_idle: bidle,
        });
    }

    pub fn update(&mut self) -> Result<bool, CpuError> {
        let (new_btotal, new_bidle) = get_raw()?;
        
        // Calculate the deltas
        let delta_total = new_btotal - self.baseline_total;
        let delta_idle = new_bidle - self.baseline_idle;
        
        if delta_total == 0 {
            return Err(CpuError::InvalidValue);
        }
        
        let new_flat = delta_total - delta_idle;
        let new_per = ((new_flat * 100) / delta_total) as u8;

        if self.cpu_per != new_per || self.cpu_flat != new_flat {
            self.cpu_per = new_per;
            self.cpu_flat = new_flat;
            self.baseline_total = new_btotal;
            self.baseline_idle = new_bidle;

            return Ok(true);    
        }
        
        self.baseline_total = new_btotal;
        self.baseline_idle = new_bidle;
        
        return Ok(false);
    }
}

fn get_raw() -> Result<(u64,u64), CpuError> {
    let lines = match read_lines("/proc/stat") {
        Ok(content) => content,
        Err(_) => return Err(CpuError::UnableToReadCpu),
    };

    for line in lines {
        if line.starts_with("cpu ") {
            let cpu_data = parse_cpu(line)?;

            let total: u64 = cpu_data.iter().sum();
            
            return Ok((
                total,
                cpu_data[3] + cpu_data[4],
            ));
        }
    }
    return Err(CpuError::UnableToReadCpu);
}