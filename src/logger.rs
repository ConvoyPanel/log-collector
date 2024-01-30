use std::path::Path;
use crate::util::logger::env::EnvironmentFile;
use crate::util::logger::meta::SystemInfo;
use crate::util::logger::{env, meta};
use crate::util::logger::docker::{DockerLogs, get_dockerlogs};
use crate::util::logger::logfiles::{AppLogfiles, get_app_logfiles};

#[derive(Debug)]
pub struct Logs {
    pub convoy_version: String,
    pub system_info: SystemInfo,
    pub env: EnvironmentFile,
    pub logfiles: AppLogfiles,
    pub docker_logs: DockerLogs,
}

pub fn get_logs(convoy_dir: &Path) -> Logs {
    let convoy_version = meta::get_convoy_version(convoy_dir);
    let system_info = meta::get_system_info();
    let env = env::get_variables(convoy_dir);
    let logfiles = get_app_logfiles(convoy_dir);
    let docker_logs = get_dockerlogs(convoy_dir);

    Logs {
        convoy_version,
        system_info,
        env,
        logfiles,
        docker_logs,
    }
}