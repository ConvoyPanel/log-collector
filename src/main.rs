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

#[tokio::main]
async fn main() {
    println!(
        "
 ██████  ██████  ███    ██ ██    ██  ██████  ██    ██
██      ██    ██ ████   ██ ██    ██ ██    ██  ██  ██
██      ██    ██ ██ ██  ██ ██    ██ ██    ██   ████
██      ██    ██ ██  ██ ██  ██  ██  ██    ██    ██
 ██████  ██████  ██   ████   ████    ██████     ██
    "
    );
    println!(
        "Convoy Log Collector\nVersion: {}\n",
        env!("CARGO_PKG_VERSION")
    );
    println!("View the source code at https://github.com/convoypanel/log-collector\n\n\n");

    get_consent();

    let path = get_directory_of_convoy();

    let convoy_version = get_convoy_version(&path);

    println!("Getting log files...");
    let log_files = get_log_files_contents(&path);

    println!("Getting docker logs...");
    let docker_logs = get_docker_logs(&path);

    println!("Getting environment file...");
    println!("Don't worry. We are only taking the values of APP_ENV, APP_DEBUG, APP_URL, DB_CONNECTION, DB_HOST, DB_PORT, CACHE_DRIVER, FILESYSTEM_DISK, QUEUE_CONNECTION, SESSION_DRIVER, SESSION_LIFETIME, REDIS_HOST, and REDIS_PORT.");
    let environment_file = get_environment_file(&path);

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
}

async fn upload_to_hastebin(client: &Client, content: &str) -> Result<String, Box<dyn Error>> {
    let resp = client
        .post("https://paste.frocdn.com/documents")
        .header("Content-Type", "application/json")
        .body(content.to_string())
        .send()
        .await?
        .text()
        .await?;

    let response_json: serde_json::Value = serde_json::from_str(&resp)?;

    let key = response_json["key"].as_str().unwrap();
    Ok(format!("https://paste.frocdn.com/{}", key))
}

fn get_consent() {
    let consent_obtained = Confirm::new()
        .with_prompt("This utility tool will upload your logs to https://paste.frocdn.com so you can easily send the logs. Do you consent?")
        .default(true)
        .interact()
        .unwrap();

    if !consent_obtained {
        println!("You did not consent. Exiting...");
        std::process::exit(0);
    }
}

fn get_directory_of_convoy() -> String {
    loop {
        let path = Input::new()
            .with_prompt("Please enter the absolute path to the folder where you installed Convoy")
            .default("/var/www/convoy".into())
            .interact_text()
            .unwrap();

        if Path::new(&path).exists() {
            return path;
        } else {
            println!("Directory does not exist: {}", path);
        }
    }
}

struct LogFiles {
    laravel: String,
    horizon: String,
    scheduler: String,
}

fn get_log_files_contents(root_directory: &String) -> LogFiles {
    let mut log_files = LogFiles {
        laravel: String::new(),
        horizon: String::new(),
        scheduler: String::new(),
    };

    let log_file_names = ["laravel.log", "horizon.log", "scheduler.log"];

    for log_file_name in log_file_names {
        let log_file_path = format!("{}/storage/logs/{}", root_directory, log_file_name);
        match File::open(&log_file_path) {
            Ok(file) => {
                // create a buffer reader to read the file from the bottom
                let reader = BufReader::new(file);
                let mut lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();
                lines.reverse();

                // get the last 10 MB of the file
                let mut bytes_read = 0;
                let mut contents = vec![];
                for line in lines {
                    bytes_read += line.len();
                    if bytes_read > 10_000_000 {
                        break;
                    }
                    contents.push(line);
                }

                let joined_contents = contents.into_iter().rev().collect::<Vec<_>>().join("\n");

                match log_file_name.as_ref() {
                    "laravel.log" => log_files.laravel = joined_contents,
                    "horizon.log" => log_files.horizon = joined_contents,
                    "scheduler.log" => log_files.scheduler = joined_contents,
                    _ => {}
                }
            }
            Err(_) => {}
        }
    }

    log_files
}

struct DockerLogs {
    ps: String,
    database: String,
    caddy: String,
    php: String,
    redis: String,
    workers: String,
}

fn get_docker_logs(root_directory: &String) -> DockerLogs {
    let mut docker_logs = DockerLogs {
        ps: String::new(),
        database: String::new(),
        caddy: String::new(),
        php: String::new(),
        redis: String::new(),
        workers: String::new(),
    };

    // get docker compose ps logs
    let ps_logs = Command::new("docker")
        .args(&["compose", "ps"])
        .current_dir(root_directory)
        .output()
        .expect("Failed to execute command \"docker compose ps\"");

    docker_logs.ps = String::from_utf8_lossy(&ps_logs.stdout).to_string();

    // get docker compose logs for each container
    let containers = ["database", "caddy", "php", "redis", "workers"];

    for container in containers.iter() {
        let logs = Command::new("docker")
            .args(&["compose", "logs", container])
            .current_dir(root_directory)
            .output()
            .expect(
                format!(
                    "Failed to execute command \"docker compose logs {}\"",
                    container
                )
                .as_str(),
            );

        let reader = BufReader::new(&logs.stdout[..]);
        let mut lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();
        lines.reverse();

        let mut contents = vec![];
        let mut total_size = 0;
        for line in lines {
            let line_size = line.as_bytes().len();
            total_size += line_size;
            if total_size > 10_000_000 {
                break;
            }
            contents.push(line);
        }

        let joined_contents = contents.into_iter().rev().collect::<Vec<_>>().join("\n");

        match container {
            &"database" => docker_logs.database = joined_contents,
            &"caddy" => docker_logs.caddy = joined_contents,
            &"php" => docker_logs.php = joined_contents,
            &"redis" => docker_logs.redis = joined_contents,
            &"workers" => docker_logs.workers = joined_contents,
            _ => (),
        }
    }

    docker_logs
}

#[derive(Debug)]
#[allow(non_snake_case)]
struct EnvironmentFile {
    APP_ENV: String,
    APP_DEBUG: String,
    APP_URL: String,
    DB_CONNECTION: String,
    DB_HOST: String,
    DB_PORT: String,
    CACHE_DRIVER: String,
    FILESYSTEM_DISK: String,
    QUEUE_CONNECTION: String,
    SESSION_DRIVER: String,
    SESSION_LIFETIME: String,
    REDIS_HOST: String,
    REDIS_PORT: String,
}

fn get_environment_file(root_directory: &String) -> EnvironmentFile {
    let env_file_path = format!("{}/.env", root_directory);
    let env_file = File::open(env_file_path).expect("failed to open .env file");

    let mut env_file_lines = BufReader::new(env_file).lines();

    let mut env_file = EnvironmentFile {
        APP_ENV: String::new(),
        APP_DEBUG: String::new(),
        APP_URL: String::new(),
        DB_CONNECTION: String::new(),
        DB_HOST: String::new(),
        DB_PORT: String::new(),
        CACHE_DRIVER: String::new(),
        FILESYSTEM_DISK: String::new(),
        QUEUE_CONNECTION: String::new(),
        SESSION_DRIVER: String::new(),
        SESSION_LIFETIME: String::new(),
        REDIS_HOST: String::new(),
        REDIS_PORT: String::new(),
    };

    while let Some(line) = env_file_lines.next() {
        if let Ok(line_contents) = line {
            let mut line_parts = line_contents.splitn(2, "=");
            let key = line_parts.next().unwrap().trim();
            let value = line_parts.next().unwrap_or("").trim();

            match key {
                "APP_ENV" => env_file.APP_ENV = value.to_string(),
                "APP_DEBUG" => env_file.APP_DEBUG = value.to_string(),
                "APP_URL" => env_file.APP_URL = value.to_string(),
                "DB_CONNECTION" => env_file.DB_CONNECTION = value.to_string(),
                "DB_HOST" => env_file.DB_HOST = value.to_string(),
                "DB_PORT" => env_file.DB_PORT = value.to_string(),
                "CACHE_DRIVER" => env_file.CACHE_DRIVER = value.to_string(),
                "FILESYSTEM_DISK" => env_file.FILESYSTEM_DISK = value.to_string(),
                "QUEUE_CONNECTION" => env_file.QUEUE_CONNECTION = value.to_string(),
                "SESSION_DRIVER" => env_file.SESSION_DRIVER = value.to_string(),
                "SESSION_LIFETIME" => env_file.SESSION_LIFETIME = value.to_string(),
                "REDIS_HOST" => env_file.REDIS_HOST = value.to_string(),
                "REDIS_PORT" => env_file.REDIS_PORT = value.to_string(),
                _ => (),
            }
        }
    }

    env_file
}

fn get_convoy_version(root_directory: &String) -> String {
    let app_config_path = format!("{}/config/app.php", root_directory);
    let app_config_contents = fs::read_to_string(app_config_path).unwrap();

    let re = Regex::new(r"'version'\s*=>\s*'(.+?)'").unwrap();
    let captures = re.captures(&app_config_contents).unwrap();
    captures[1].to_string()
}