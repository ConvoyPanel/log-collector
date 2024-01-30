use std::path::{Path, PathBuf};
use std::process::exit;
use dialoguer::Confirm;

use crate::util;

pub fn get_info() -> PathBuf {
    match Confirm::new()
        .with_prompt("Your logs (potentially sensitive!) will be uploaded https://paste.frocdn.com. Do you consent?")
        .default(true)
        .interact() {
        Ok(true) => {}
        _ => {
            println!("Aborting.");
            exit(0);
        }
    }

    util::helpers::get_directory(
        "Absolute directory of your convoy installation",
        Path::new("/var/www/convoy"),
    )
}