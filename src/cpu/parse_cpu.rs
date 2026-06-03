use scrtypes::boolean_expansion::boolean_extension::BoolExpansion;
use scrtypes::ExtendedStrings;
use crate::errors::CpuError;

pub fn parse_cpu(cpu_info: String) -> Result<[u64;8], CpuError> {
    let chars: Vec<char> = cpu_info.chars().collect();
    let mut buffer: String = String::new();
    let mut info_arr: [u64;8] = [0;8];
    let mut i: usize = 0;

    for c in chars {
        if i >= 8 {
            return Err(CpuError::InvalidCpuLine);
        }

        if c.is_whitespace() {
            if buffer.is_empty() {
                continue;
            }

            if buffer == "cpu" {
                buffer.clear();
                continue;
            }

            if buffer.is_uint().flip() {
                return Err(CpuError::InvalidValue);
            }

            
            info_arr[i] = match buffer.to_uint() {
                Ok(content) => content,
                Err(_) => return Err(CpuError::InvalidValue),
            };
            i+=1;
            buffer.clear();
            continue;
        }

        buffer += &c.to_string();
    }

    if i >= 8 {
        return Err(CpuError::InvalidCpuLine);
    }

    // i remember to clean up the final buffer this time ^^
    if buffer.is_empty().flip() {
        if i >= 8 {
            return Err(CpuError::InvalidCpuLine);
        }

        if buffer.is_uint().flip() {
            return Err(CpuError::InvalidValue);
        }

        info_arr[i] = match buffer.to_uint() {
            Ok(content) => content,
            Err(_) => return Err(CpuError::InvalidValue),
        };
        i+=1;
    }

    if i != 8 {
        return Err(CpuError::InvalidCpuLine);
    }

    return Ok(info_arr);
}