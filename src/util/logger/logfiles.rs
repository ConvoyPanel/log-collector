use std::path::Path;
use crate::util::lumberjack::axe_logfiles;

#[derive(Debug)]
pub struct AppLogfiles {
    pub laravel: String,
    pub horizon: String,
    pub scheduler: String,
}

pub fn get_app_logfiles(convoy_dir: &Path) -> AppLogfiles {
    let laravel_logs = axe_logfiles(&convoy_dir.join("storage/logs/laravel.log"));
    let horizon_logs = axe_logfiles(&convoy_dir.join("storage/logs/horizon.log"));
    let scheduler_logs = axe_logfiles(&convoy_dir.join("storage/logs/scheduler.log"));

    AppLogfiles {
        laravel: laravel_logs,
        horizon: horizon_logs,
        scheduler: scheduler_logs,
    }
}