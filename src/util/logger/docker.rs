use std::path::Path;
use std::process::Command;
use crate::util::lumberjack::axe_dockerlogs;

#[derive(Debug)]
pub struct DockerLogs {
    pub ps: String,
    pub database: String,
    pub caddy: String,
    pub php: String,
    pub redis: String,
    pub workers: String,
}

pub fn get_dockerlogs(convoy_dir: &Path) -> DockerLogs {
    let mut docker_logs = DockerLogs {
        ps: String::new(),
        database: String::new(),
        caddy: String::new(),
        php: String::new(),
        redis: String::new(),
        workers: String::new(),
    };

    let ps_cmd_output = Command::new("docker")
        .args(&["compose", "ps"])
        .current_dir(convoy_dir)
        .output()
        .expect("Failed to execute command \"docker compose ps\"");
    docker_logs.ps = String::from_utf8_lossy(&ps_cmd_output.stdout).to_string();

    docker_logs.database = axe_dockerlogs(convoy_dir, "database");
    docker_logs.caddy = axe_dockerlogs(convoy_dir, "caddy");
    docker_logs.php = axe_dockerlogs(convoy_dir, "php");
    docker_logs.redis = axe_dockerlogs(convoy_dir, "redis");
    docker_logs.workers = axe_dockerlogs(convoy_dir, "workers");

    docker_logs
}