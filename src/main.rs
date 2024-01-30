use chrono::prelude::*;
use dialoguer::{Confirm, Input};
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process::Command;
use regex::Regex;
use sysinfo::{
    Components, Disks, Networks, System,
};
use crate::util::logger::meta::get_system_info;

mod util;
mod logger;
mod api;
mod uploader;

#[tokio::main]
async fn main() {
    let convoy_dir = util::questions::get_info();

    let logs = logger::get_logs(&convoy_dir);

    let link = uploader::upload(logs).await;

    println!("Log files uploaded successfully! Link: {}", link);
/*
    println!("Uploading logs to https://paste.frocdn.com... (this may take a while)");
    let client = Client::new();
    let laravel_log_url = upload_to_hastebin(&client, &log_files.laravel)
        .await
        .unwrap();
    let horizon_log_url = upload_to_hastebin(&client, &log_files.horizon)
        .await
        .unwrap();
    let scheduler_log_url = upload_to_hastebin(&client, &log_files.scheduler)
        .await
        .unwrap();

    // Upload Docker logs
    let ps_log_url = upload_to_hastebin(&client, &docker_logs.ps).await.unwrap();
    let database_log_url = upload_to_hastebin(&client, &docker_logs.database)
        .await
        .unwrap();
    let caddy_log_url = upload_to_hastebin(&client, &docker_logs.caddy)
        .await
        .unwrap();
    let php_log_url = upload_to_hastebin(&client, &docker_logs.php).await.unwrap();
    let redis_log_url = upload_to_hastebin(&client, &docker_logs.redis)
        .await
        .unwrap();
    let workers_log_url = upload_to_hastebin(&client, &docker_logs.workers)
        .await
        .unwrap();

    // Upload environment file
    let environment_file_url = upload_to_hastebin(&client, &format!("{:?}", environment_file))
        .await
        .unwrap();

    // Print output

    let now = Local::now();
    let formatted_date = now.format("%Y-%m-%d %H:%M:%S %:z").to_string();

    let compiled_paste = [
        format!("Convoy Log Collector v{}", env!("CARGO_PKG_VERSION")),
        format!("Convoy Version: {}", convoy_version),
        format!("{APP_URL}", APP_URL = environment_file.APP_URL),
        format!("{:?}", formatted_date),
        format!("\nLaravel Log Files"),
        format!("laravel.log: {}", laravel_log_url),
        format!("horizon.log: {}", horizon_log_url),
        format!("scheduler.log: {}", scheduler_log_url),
        format!("\nDocker Logs"),
        format!("ps: {}", ps_log_url),
        format!("database: {}", database_log_url),
        format!("caddy: {}", caddy_log_url),
        format!("php: {}", php_log_url),
        format!("redis: {}", redis_log_url),
        format!("workers: {}", workers_log_url),
        format!("\nEnvironment file: {}", environment_file_url),
    ];

    let compiled_paste_url = upload_to_hastebin(&client, compiled_paste.join("\n").as_str()).await.unwrap();
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nLog files uploaded successfully! Link: {}", compiled_paste_url);

 */
}