use std::error::Error;
use log::{info, error};
mod hardware_info;
use hardware_info::HardwareInfo;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    
    info!("Starting hardware ID collection...");
    
    let hardware_id = match HardwareInfo::collect() {
        Ok(id) => id,
        Err(e) => {
            error!("Failed to collect hardware info: {}", e);
            return Err(e.into());
        }
    };
    
    info!("Hardware information collected successfully");
    println!("收集到的硬件信息：");
    println!("{:#?}", hardware_id);
    
    let unique_code = match hardware_id.generate_unique_code() {
        Ok(code) => code,
        Err(e) => {
            error!("Failed to generate unique code: {}", e);
            return Err(e.into());
        }
    };
    
    info!("Unique code generated successfully");
    println!("\n生成的唯一码: {}", unique_code);
    Ok(())
} 