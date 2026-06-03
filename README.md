procinf or Process Info for short is a lightweight, decoupled system monitoring engine built to eliminate the boilerplate of reading and parsing Linux kernel metrics from `/proc`. To eliminate the pain points of continuously managing structural shifts, tracking baselines, or re-allocating files during system queries, this engine encapsulates state management loops cleanly into standalone drivers that roll up into a single orchestrator.

CPU Subsystem
use procinf::cpu::CPU;

The struct CPU provides structural tracking for raw processor load transformations over time intervals:

pub cpu_per: u8       // The calculated active usage percentage (0% to 100%)
pub cpu_flat: u64     // Total active jiffies consumed between updates
.new()                // Captures baseline jiffies from `/proc/stat` and returns a CPU driver instance
.update()             // Calculates the deltas against the active baseline, updates structural states, and returns a Result<bool, CpuError> indicating if a metric shifted

RAM Subsystem
use procinf::mem::RAM;

The struct RAM tracks active hardware memory profiles using `/proc/meminfo`:

pub total: u64        // Complete system physical memory capacity reported in kB
pub used: u64         // Calculated active memory utilization (Total - Available)
pub free: u64         // The true kernel-available space for fresh allocations
.new()                // Instantiates structural mappings of raw physical memory tracking
.update()             // Rewires raw kernel snapshots, shifting local memory structures and returning a Result<bool, RamError> on variable deviations

System Orchestration Driver
use procinf::SysInfo;

The macro orchestrator SysInfo encapsulates independent subsystem states safely under a unified updating controller loop:

pub cpu_per: u8       // Current structural sync of the underlying CPU driver load percentage
pub cpu_flat: u64     // Current structural sync of active CPU tick differences
pub ram_total: u64    // Active snapshot of overall system memory size
pub ram_free: u64     // Active snapshot of true available application memory space
pub ram_used_per: u8  // Instantaneous calculation of system memory load using robust u64 tracking

### Operational Example
```rust
use procinf::SysInfo;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), procinf::errors::SysInfoError> {
    // Initialize the unified state engine
    let mut system = SysInfo::new()?;

    loop {
        // .update() returns true if EITHER driver registers a statistical metric change
        if system.update()? {
            println!("--- System Shift Detected ---");
            println!("CPU: {}% (Raw Active: {})", system.cpu_per, system.cpu_flat);
            println!("RAM: {}% (Free: {} kB / Total: {} kB)", system.ram_used_per, system.ram_free, system.ram_total);
        }
        sleep(Duration::from_secs(1));
    }
}
```

### Errors

```rust
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
```
Errors are interconnected across modules via automatic trait conversions (From<CpuError> and From<RamError>), meaning subsystem failures propagate cleanly up to the top-level orchestration layer without manual mapping boilerplate.

### Note

This engine was designed by me to run as a lean, low-overhead Linux telemetry state machine, taking advantage of type guarantees by mapping metrics cleanly into explicit 64-bit bounds. Feel free to extend abstractions or write higher-level output displays for it. The README was written by gemini if there is any issue please open an issue on github and be respectful about it.Technical and respectful feedback is always welcome.

- SudoSilver