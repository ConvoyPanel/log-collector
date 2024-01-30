use crate::logger::Logs;
use crate::api::hastebin::upload as hupload;

pub struct LogLinks {
    docker_ps: String,
    docker_database: String,
    docker_caddy: String,
    docker_php: String,
    docker_redis: String,
    docker_workers: String,
    convoy_env: String,
    laravel_log: String,
    laravel_worker_log: String,
    laravel_schedule_log: String,
}

pub async fn upload(logs: Logs) -> String {
    let log_links = LogLinks {
        docker_ps: hupload(&logs.docker_logs.ps).await.unwrap(),
        docker_database: hupload(&logs.docker_logs.database).await.unwrap(),
        docker_caddy: hupload(&logs.docker_logs.caddy).await.unwrap(),
        docker_php: hupload(&logs.docker_logs.php).await.unwrap(),
        docker_redis: hupload(&logs.docker_logs.redis).await.unwrap(),
        docker_workers: hupload(&logs.docker_logs.workers).await.unwrap(),
        convoy_env: hupload(&format!("{:#?}", &logs.env)).await.unwrap(),
        laravel_log: hupload(&logs.logfiles.laravel).await.unwrap(),
        laravel_worker_log: hupload(&logs.logfiles.horizon).await.unwrap(),
        laravel_schedule_log: hupload(&logs.logfiles.scheduler).await.unwrap(),
    };

    let final_output = format!(
        "Convoy Log Collector v{collector_version}
Convoy Version: {convoy_version}
{APP_URL}
{date:?}


Laravel Log Files
laravel.log: {laravel_log}
horizon.log: {laravel_horizon_log}
scheduler.log: {laravel_scheduler_log}

Docker Logs
ps: {docker_ps}
caddy: {docker_caddy}
php: {docker_php}
workers: {docker_workers}
redis: {docker_redis}
database: {docker_database}
Environment file: {convoy_env}

System info
OS: {os}
OS version: {os_version}
Kernel version: {kernel_version}
Architecture: {arch}
CPU: {cpu}
CPU count: {cpu_count}
Physical memory: {physical_memory_mebibytes} MiB
Swap memory: {swap_memory_mebibytes} MiB",
        collector_version = env!("CARGO_PKG_VERSION"),
        convoy_version = logs.convoy_version,
        APP_URL = logs.env.APP_URL,
        date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S %:z").to_string(),

        laravel_log = log_links.laravel_log,
        laravel_horizon_log = log_links.laravel_worker_log,
        laravel_scheduler_log = log_links.laravel_schedule_log,

        docker_ps = log_links.docker_ps,
        docker_caddy = log_links.docker_caddy,
        docker_php = log_links.docker_php,
        docker_workers = log_links.docker_workers,
        docker_redis = log_links.docker_redis,
        docker_database = log_links.docker_database,
        convoy_env = log_links.convoy_env,

        os = logs.system_info.os,
        os_version = logs.system_info.os_version,
        kernel_version = logs.system_info.kernel_version,
        arch = logs.system_info.arch,
        cpu = logs.system_info.cpu,
        cpu_count = logs.system_info.cpu_count,
        physical_memory_mebibytes = logs.system_info.physical_memory_mebibytes,
        swap_memory_mebibytes = logs.system_info.swap_memory_mebibytes,
    );

    hupload(&final_output).await.unwrap()
}