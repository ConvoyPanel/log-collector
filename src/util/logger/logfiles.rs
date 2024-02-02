use std::path::Path;
use crate::util::lumberjack::axe_logfiles;

#[derive(Debug)]
pub struct AppLogfiles {
    pub laravel: String,
    pub horizon: String,
    pub scheduler: String,
}

pub fn get_app_logfiles(convoy_dir: &Path) -> AppLogfiles {
    let laravel_logs = axe_logfiles(&convoy_dir.join("storage/logs/laravel.log")).unwrap_or("Couldn't read laravel.log. Does it exist?".to_string());
    let horizon_logs = axe_logfiles(&convoy_dir.join("storage/logs/horizon.log")).unwrap_or("Couldn't read horizon.log. Does it exist?".to_string());
    let scheduler_logs = axe_logfiles(&convoy_dir.join("storage/logs/scheduler.log")).unwrap_or("Couldn't read scheduler.log. Does it exist?".to_string());

    AppLogfiles {
        laravel: laravel_logs,
        horizon: horizon_logs,
        scheduler: scheduler_logs,
    }
}