#[derive(Debug)]
pub enum CpuError {
    UnableToReadCpu,
    InvalidCpuLine,
    InvalidValue,
}

#[derive(Debug)]
pub enum RamError {
    UnableToReadRam,
    InvalidValue,
}

#[derive(Debug)]
pub enum SysInfoError {
    Cpu(CpuError),
    Ram(RamError),
}

impl From<CpuError> for SysInfoError {
    fn from(err: CpuError) -> Self {
        return SysInfoError::Cpu(err);
    }
}

impl From<RamError> for SysInfoError {
    fn from(err: RamError) -> Self {
        return SysInfoError::Ram(err);
    }
}