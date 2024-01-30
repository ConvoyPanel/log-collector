use std::path::{Path, PathBuf};
use dialoguer::Input;

pub fn get_directory(prompt: &str, default: &Path) -> PathBuf {
    loop {
        let path = Input::new()
            .with_prompt(prompt)
            .default(default.display().to_string())
            .interact_text()
            .unwrap();

        let path = PathBuf::from(&path);

        if path.exists() {
            return path;
        } else {
            println!("Directory does not exist: {}", path.as_path().display());
        }
    }
}