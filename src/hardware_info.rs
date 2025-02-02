use sysinfo::{System, SystemExt, NetworkExt};
use std::error::Error;
use md5::{Md5, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    name: String,
    mac_address: String,
    is_up: bool,
    interface_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareInfo {
    cpu_info: String,
    motherboard_serial: String,
    disk_serial: String,
    mac_address: String,
    os_info: String,
    memory_serial: String,
    bios_version: String,
    cpu_physical_id: String,
    disk_model: String,
    disk_firmware: String,
    motherboard_uuid: String,
    motherboard_manufacturer: String,
    motherboard_product_name: String,
    bios_vendor: String,
    bios_release_date: String,
    network_interfaces: Vec<NetworkInfo>,
}

impl HardwareInfo {
    pub fn collect() -> Result<Self, Box<dyn Error>> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut network_interfaces = Vec::new();
        for (name, network) in sys.networks() {
            if Self::is_primary_interface(name) {
                let mac = network.mac_address().to_string();
                if !mac.is_empty() && mac != "00:00:00:00:00:00" {
                    network_interfaces.push(NetworkInfo {
                        name: name.to_string(),
                        mac_address: mac,
                        is_up: true,
                        interface_type: Self::detect_interface_type(name),
                    });
                }
            }
        }

        if network_interfaces.len() > 1 {
            network_interfaces.truncate(1);
        }

        network_interfaces.sort_by(|a, b| a.mac_address.cmp(&b.mac_address));

        Ok(Self {
            cpu_info: Self::get_cpu_info()?,
            motherboard_serial: Self::get_motherboard_serial()?,
            disk_serial: Self::get_disk_serial()?,
            mac_address: Self::get_mac_address()?,
            os_info: format!("{} {}", sys.name().unwrap_or_default(), 
                                    sys.os_version().unwrap_or_default()),
            memory_serial: Self::get_memory_serial()?,
            bios_version: Self::get_bios_version()?,
            cpu_physical_id: Self::get_cpu_physical_id()?,
            disk_model: Self::get_disk_model()?,
            disk_firmware: Self::get_disk_firmware()?,
            motherboard_uuid: Self::get_motherboard_uuid()?,
            motherboard_manufacturer: Self::get_motherboard_manufacturer()?,
            motherboard_product_name: Self::get_motherboard_product_name()?,
            bios_vendor: Self::get_bios_vendor()?,
            bios_release_date: Self::get_bios_release_date()?,
            network_interfaces,
        })
    }

    fn is_primary_interface(name: &str) -> bool {
        match name {
            "en0" | "eth0" | "enp0s1" => true,
            _ => false
        }
    }

    fn detect_interface_type(name: &str) -> String {
        if name.starts_with("en") || name.starts_with("eth") {
            "Ethernet".to_string()
        } else if name.starts_with("wl") || name.starts_with("wifi") {
            "Wi-Fi".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    pub fn generate_unique_code(&self) -> Result<String, Box<dyn Error>> {
        if self.motherboard_serial.is_empty() || self.motherboard_uuid.is_empty() {
            return Err("Critical hardware information missing".into());
        }

        let mut hasher = Md5::new();

        hasher.update(self.motherboard_serial.as_bytes());
        hasher.update(self.motherboard_uuid.as_bytes());

        for interface in &self.network_interfaces {
            if interface.is_up && !interface.mac_address.is_empty() {
                hasher.update(interface.mac_address.as_bytes());
            }
        }

        let secondary_info = format!("{}:{}:{}",
            self.cpu_physical_id,
            self.motherboard_product_name,
            self.disk_model
        );
        hasher.update([0xFF]);
        hasher.update(secondary_info.as_bytes());

        let result = hasher.finalize();
        let hash = hex::encode(result);

        Ok(format!("{}-{}-{}-{}", 
            &hash[0..4], 
            &hash[4..8], 
            &hash[8..12], 
            &hash[12..16]
        ))
    }

    #[cfg(target_os = "windows")]
    fn get_cpu_physical_id() -> Result<String, Box<dyn Error>> {
        use std::process::Command;
        let output = Command::new("wmic")
            .args(&["cpu", "get", "processorid"])
            .output()?;
        let id = String::from_utf8_lossy(&output.stdout)
            .lines()
            .nth(1)
            .unwrap_or("")
            .trim()
            .to_string();
        Ok(id)
    }

    #[cfg(target_os = "windows")]
    fn get_disk_model() -> Result<String, Box<dyn Error>> {
        use std::process::Command;
        let output = Command::new("wmic")
            .args(&["diskdrive", "get", "model"])
            .output()?;
        let model = String::from_utf8_lossy(&output.stdout)
            .lines()
            .nth(1)
            .unwrap_or("")
            .trim()
            .to_string();
        Ok(model)
    }

    #[cfg(target_os = "windows")]
    fn get_disk_firmware() -> Result<String, Box<dyn Error>> {
        use std::process::Command;
        let output = Command::new("wmic")
            .args(&["diskdrive", "get", "firmwarerevision"])
            .output()?;
        let firmware = String::from_utf8_lossy(&output.stdout)
            .lines()
            .nth(1)
            .unwrap_or("")
            .trim()
            .to_string();
        Ok(firmware)
    }

    #[cfg(target_os = "linux")]
    fn get_cpu_physical_id() -> Result<String, Box<dyn Error>> {
        use std::fs;
        let id = fs::read_to_string("/proc/cpuinfo")?
            .lines()
            .find(|line| line.starts_with("physical id"))
            .unwrap_or("")
            .split(':')
            .nth(1)
            .unwrap_or("")
            .trim()
            .to_string();
        Ok(id)
    }

    #[cfg(target_os = "linux")]
    fn get_disk_model() -> Result<String, Box<dyn Error>> {
        use std::process::Command;
        let output = Command::new("lsblk")
            .args(&["-no", "MODEL"])
            .output()?;
        let model = String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        Ok(model)
    }

    #[cfg(target_os = "linux")]
    fn get_disk_firmware() -> Result<String, Box<dyn Error>> {
        use std::fs;
        let firmware = fs::read_to_string("/sys/class/block/sda/device/firmware_rev")?
            .trim()
            .to_string();
        Ok(firmware)
    }

    #[cfg(target_os = "macos")]
    fn get_cpu_physical_id() -> Result<String, Box<dyn Error>> {
        use std::process::Command;
        let output = Command::new("sysctl")
            .args(&["-n", "machdep.cpu.brand_string"])
            .output()?;
        let id = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
        Ok(id)
    }

    #[cfg(target_os = "macos")]
    fn get_disk_model() -> Result<String, Box<dyn Error>> {
        use std::process::Command;
        let output = Command::new("diskutil")
            .args(&["info", "disk0"])
            .output()?;
        let model = String::from_utf8_lossy(&output.stdout)
            .lines()
            .find(|line| line.contains("Device / Media Name"))
            .unwrap_or("")
            .split(':')
            .nth(1)
            .unwrap_or("")
            .trim()
            .to_string();
        Ok(model)
    }

    #[cfg(target_os = "macos")]
    fn get_disk_firmware() -> Result<String, Box<dyn Error>> {
        use std::process::Command;
        let output = Command::new("system_profiler")
            .args(&["SPNVMeDataType"])
            .output()?;
        let firmware = String::from_utf8_lossy(&output.stdout)
            .lines()
            .find(|line| line.contains("Firmware Version"))
            .unwrap_or("")
            .split(':')
            .nth(1)
            .unwrap_or("")
            .trim()
            .to_string();
        Ok(firmware)
    }

    fn get_motherboard_uuid() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(&["csproduct", "get", "uuid"])
                .output()?;
            let uuid = String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(uuid)
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let uuid = fs::read_to_string("/sys/class/dmi/id/product_uuid")?
                .trim()
                .to_string();
            Ok(uuid)
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("system_profiler")
                .args(&["SPHardwareDataType"])
                .output()?;
            let uuid = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("Hardware UUID"))
                .unwrap_or("")
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(uuid)
        }
    }

    // CPU 信息获取
    fn get_cpu_info() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(&["cpu", "get", "name"])
                .output()?;
            let info = String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(info)
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let info = fs::read_to_string("/proc/cpuinfo")?
                .lines()
                .find(|line| line.starts_with("model name"))
                .unwrap_or("")
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(info)
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("sysctl")
                .args(&["-n", "machdep.cpu.brand_string"])
                .output()?;
            let info = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            Ok(info)
        }
    }

    // 主板序列号获取
    fn get_motherboard_serial() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(&["baseboard", "get", "serialnumber"])
                .output()?;
            let serial = String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(serial)
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let serial = fs::read_to_string("/sys/class/dmi/id/board_serial")?
                .trim()
                .to_string();
            Ok(serial)
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("system_profiler")
                .args(&["SPHardwareDataType"])
                .output()?;
            let serial = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("Serial Number"))
                .unwrap_or("")
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(serial)
        }
    }

    // 磁盘序列号获取
    fn get_disk_serial() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(&["diskdrive", "get", "serialnumber"])
                .output()?;
            let serial = String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(serial)
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            let output = Command::new("udevadm")
                .args(&["info", "--query=property", "--name=/dev/sda"])
                .output()?;
            let serial = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.starts_with("ID_SERIAL="))
                .unwrap_or("")
                .split('=')
                .nth(1)
                .unwrap_or("")
                .to_string();
            Ok(serial)
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("diskutil")
                .args(&["info", "disk0"])
                .output()?;
            let serial = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("Serial Number"))
                .unwrap_or("")
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(serial)
        }
    }

    // MAC 地址获取
    fn get_mac_address() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("getmac")
                .output()?;
            let mac = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string();
            Ok(mac)
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            let output = Command::new("ip")
                .args(&["link", "show"])
                .output()?;
            let mac = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("link/ether"))
                .and_then(|line| line.split_whitespace().nth(1))
                .unwrap_or("")
                .to_string();
            Ok(mac)
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("ifconfig")
                .output()?;
            let mac = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("ether"))
                .and_then(|line| line.split_whitespace().nth(1))
                .unwrap_or("")
                .to_string();
            Ok(mac)
        }
    }

    // BIOS 版本获取
    fn get_bios_version() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(&["bios", "get", "version"])
                .output()?;
            let version = String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(version)
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let version = fs::read_to_string("/sys/class/dmi/id/bios_version")?
                .trim()
                .to_string();
            Ok(version)
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("system_profiler")
                .args(&["SPHardwareDataType"])
                .output()?;
            let version = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("Boot ROM Version"))
                .unwrap_or("")
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(version)
        }
    }

    // 主板制造商获取
    fn get_motherboard_manufacturer() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(&["baseboard", "get", "manufacturer"])
                .output()?;
            let manufacturer = String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(manufacturer)
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let manufacturer = fs::read_to_string("/sys/class/dmi/id/board_vendor")?
                .trim()
                .to_string();
            Ok(manufacturer)
        }

        #[cfg(target_os = "macos")]
        {
            Ok(String::from("Apple Inc."))
        }
    }

    // 主板产品名称获取
    fn get_motherboard_product_name() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(&["baseboard", "get", "product"])
                .output()?;
            let product = String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(product)
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let product = fs::read_to_string("/sys/class/dmi/id/board_name")?
                .trim()
                .to_string();
            Ok(product)
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("system_profiler")
                .args(&["SPHardwareDataType"])
                .output()?;
            let product = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("Model Identifier"))
                .unwrap_or("")
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(product)
        }
    }

    // BIOS 供应商获取
    fn get_bios_vendor() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(&["bios", "get", "manufacturer"])
                .output()?;
            let vendor = String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(vendor)
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let vendor = fs::read_to_string("/sys/class/dmi/id/bios_vendor")?
                .trim()
                .to_string();
            Ok(vendor)
        }

        #[cfg(target_os = "macos")]
        {
            Ok(String::from("Apple Inc."))
        }
    }

    // BIOS 发布日期获取
    fn get_bios_release_date() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(&["bios", "get", "releasedate"])
                .output()?;
            let date = String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(date)
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let date = fs::read_to_string("/sys/class/dmi/id/bios_date")?
                .trim()
                .to_string();
            Ok(date)
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("system_profiler")
                .args(&["SPHardwareDataType"])
                .output()?;
            let date = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("Boot ROM Version"))
                .unwrap_or("")
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(date)
        }
    }

    // 内存序列号获取
    fn get_memory_serial() -> Result<String, Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(&["memorychip", "get", "serialnumber"])
                .output()?;
            let serial = String::from_utf8_lossy(&output.stdout)
                .lines()
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(serial)
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            // 使用 dmidecode 命令获取内存信息（需要 root 权限）
            let output = Command::new("sudo")
                .args(&["dmidecode", "-t", "memory"])
                .output()?;
            let serial = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("Serial Number:"))
                .unwrap_or("")
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(serial)
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let output = Command::new("system_profiler")
                .args(&["SPMemoryDataType"])
                .output()?;
            let serial = String::from_utf8_lossy(&output.stdout)
                .lines()
                .find(|line| line.contains("Serial Number:"))
                .unwrap_or("")
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            Ok(serial)
        }
    }
} 