use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader, SeekFrom};
use std::process::Command;

const CHUNK_SIZE: u64 = 5 * 1024 * 1024; // 5 MiB

pub fn axe_logfiles(file_path: &Path) -> String {
    let mut file = File::open(file_path).unwrap();
    let file_size = file.metadata().unwrap().len();

    let start_pos = if file_size > CHUNK_SIZE {
        file_size - CHUNK_SIZE
    } else {
        0
    };

    file.seek(SeekFrom::Start(start_pos)).expect(
        &format!(
            "Failed to move to last 10 MiB of log file: {}",
            file_path.file_name().and_then(OsStr::to_str).unwrap()
        )
    );

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    let mut content = String::from_utf8(buffer).unwrap_or_else(|_| String::new());
    //let mut lines = content.split("\n").collect::<Vec<_>>();
    //lines.reverse();
    //content = lines.join("\n");

    content
}

pub fn axe_dockerlogs(convoy_dir: &Path, container_name: &str) -> String {
    let cmd_output = Command::new("docker")
        .arg("compose")
        .arg("logs")
        .arg(container_name)
        .current_dir(convoy_dir)
        .output()
        .expect(
            format!(
                "Failed to execute command \"docker compose logs {}\"",
                container_name
            )
                .as_str(),
        );

    let output = &cmd_output.stdout;

    let start_pos = if output.len() as u64 > CHUNK_SIZE {
        output.len() as u64 - CHUNK_SIZE
    } else {
        0
    };

    let content = String::from_utf8_lossy(&output[start_pos as usize..]);
    content.to_string()
}