use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct EnvironmentFile {
    pub APP_ENV: String,
    pub APP_DEBUG: String,
    pub APP_URL: String,
    pub DB_CONNECTION: String,
    pub DB_HOST: String,
    pub DB_PORT: String,
    pub DB_PASSWORD_IS_SET: bool,
    pub CACHE_DRIVER: String,
    pub FILESYSTEM_DISK: String,
    pub QUEUE_CONNECTION: String,
    pub SESSION_DRIVER: String,
    pub SESSION_LIFETIME: String,
    pub REDIS_HOST: String,
    pub REDIS_PORT: String,
    pub REDIS_PASSWORD_IS_SET: bool,
}

pub fn get_variables(convoy_dir: &Path) -> EnvironmentFile {
    let env_file = File::open(convoy_dir.join(".env")).expect("failed to open .env file");

    let mut env_file_lines = BufReader::new(env_file).lines();

    let mut env_file = EnvironmentFile {
        APP_ENV: String::new(),
        APP_DEBUG: String::new(),
        APP_URL: String::new(),
        DB_CONNECTION: String::new(),
        DB_HOST: String::new(),
        DB_PORT: String::new(),
        DB_PASSWORD_IS_SET: false,
        CACHE_DRIVER: String::new(),
        FILESYSTEM_DISK: String::new(),
        QUEUE_CONNECTION: String::new(),
        SESSION_DRIVER: String::new(),
        SESSION_LIFETIME: String::new(),
        REDIS_HOST: String::new(),
        REDIS_PORT: String::new(),
        REDIS_PASSWORD_IS_SET: false,
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
                "DB_PASSWORD" => env_file.DB_PASSWORD_IS_SET = value.len() > 0,
                "CACHE_DRIVER" => env_file.CACHE_DRIVER = value.to_string(),
                "FILESYSTEM_DISK" => env_file.FILESYSTEM_DISK = value.to_string(),
                "QUEUE_CONNECTION" => env_file.QUEUE_CONNECTION = value.to_string(),
                "SESSION_DRIVER" => env_file.SESSION_DRIVER = value.to_string(),
                "SESSION_LIFETIME" => env_file.SESSION_LIFETIME = value.to_string(),
                "REDIS_HOST" => env_file.REDIS_HOST = value.to_string(),
                "REDIS_PORT" => env_file.REDIS_PORT = value.to_string(),
                "REDIS_PASSWORD" => env_file.REDIS_PASSWORD_IS_SET = value.len() > 0,
                _ => (),
            }
        }
    }

    env_file
}