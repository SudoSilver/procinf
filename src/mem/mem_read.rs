use crate::errors::RamError;
use scrtypes::read_lines;
use crate::mem::parse_mem::parse_mem;

pub struct RAM {
    pub total: u64,
    pub used: u64,
    pub free: u64,
}

impl RAM {
    pub fn new() -> Result<Self, RamError> {
        let (total, free) = get_raw()?;
        let used = total - free;

        return Ok(Self {
            total,
            used,
            free,
        });
    }
 
    pub fn update(&mut self) -> Result<bool, RamError> {
        let (total, free) = get_raw()?;
        let used = total - free;

        let has_changed = self.total != total || self.free != free || self.used != used;

        if has_changed {
            self.total = total;
            self.free = free;
            self.used = used;
        }

        return Ok(has_changed);
    }
}

fn get_raw() -> Result<(u64,u64), RamError> {
    let lines = match read_lines("/proc/meminfo") {
        Ok(content) => content,
        Err(_) => return Err(RamError::UnableToReadRam),
    };

    let mut total_mem: u64 = 0;
    let mut available_mem: u64 = 0;

    for line in lines {
        if line.starts_with("MemTotal:") {
            total_mem = parse_mem(line)?;
        }else if line.starts_with("MemAvailable:") {
            available_mem = parse_mem(line)?;
        }else{
            continue;
        }
    }
    
    if total_mem == 0 || available_mem == 0 {
        return Err(RamError::UnableToReadRam);
    }

    return Ok((total_mem, available_mem));
}