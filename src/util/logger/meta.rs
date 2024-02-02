use std::fs;
use std::path::Path;
use regex::Regex;
use sysinfo::{
    System,
};

pub fn get_convoy_version(convoy_dir: &Path) -> String {
    let app_config_path = convoy_dir.join("config/app.php");
    let app_config_contents = fs::read_to_string(app_config_path).unwrap();

    let re = Regex::new(r"'version'\s*=>\s*'(.+?)'").unwrap();
    let captures = re.captures(&app_config_contents).unwrap();
    captures[1].to_string()
}

#[derive(Debug)]
pub struct SystemInfo {
    pub os: String,
    pub os_version: String,
    pub kernel_version: String,
    pub arch: String,
    pub cpu: String,
    pub cpu_count: u32,
    pub physical_memory_mebibytes: u32,
    pub swap_memory_mebibytes: u32,
}

pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    SystemInfo {
        os: System::name().expect("Failed to get OS name"),
        os_version: System::os_version().expect("Failed to get OS version"),
        kernel_version: System::kernel_version().expect("Failed to get kernel version"),
        arch: System::cpu_arch().expect("Failed to get CPU architecture"),
        cpu: sys.cpus().iter().nth(0).unwrap().brand().to_owned(),
        cpu_count: sys.cpus().len() as u32,
        physical_memory_mebibytes: (sys.total_memory() / 1024 / 1024) as u32,
        swap_memory_mebibytes: (sys.total_swap() / 1024 / 1024) as u32,
    }
}