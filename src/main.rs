mod util;
mod logger;
mod api;
mod uploader;

#[tokio::main]
async fn main() {
    util::branding::show_msg();

    let convoy_dir = util::questions::get_info();

    let logs = logger::get_logs(&convoy_dir);

    let link = uploader::upload(logs).await;

    println!("Log files uploaded successfully! Link: {}", link);
}