use scrtypes::boolean_expansion::boolean_extension::BoolExpansion;
use scrtypes::ExtendedStrings;
use crate::errors::RamError;

pub fn parse_mem(info: String) -> Result<u64, RamError> {
    let mut buffer: String = String::new();
    
    for c in info.chars() {
        if c.is_whitespace() {
            if buffer.is_empty() || buffer == "MemTotal:" || buffer == "MemAvailable:" {
                buffer.clear();
                continue;
            } else {
                break; 
            }
        }
        buffer += &c.to_string();
    }
    
    if buffer.is_uint().flip() {
        return Err(RamError::InvalidValue);
    }

    let mem_total = match buffer.to_uint() {
        Ok(content) => content,
        Err(_) => return Err(RamError::InvalidValue),
    };

    return Ok(mem_total);
}