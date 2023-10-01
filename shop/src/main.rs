// export RUST_LOG=info; cargo watch -x check -x test -x run
extern crate env_logger;
extern crate log;
use tokio::time::{Duration, sleep};
// Import your custom logger module
use anyhow::Result;


mod my_logger;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Use your custom logger.
    log::set_boxed_logger(Box::new(my_logger::MyLogger::new())).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    my_logger::test();

    let webserver_ip = "127.0.0.1";
    let webserver_port = 8080;
    log::info!("🚀 Starting webserver on http://{webserver_ip}:{webserver_port}");
    let webserver_future = web::webserver::start(webserver_ip, webserver_port);
    // Spawn the web server task.
    let webserver_handle = tokio::task::spawn(webserver_future);
    sleep(Duration::from_secs(2)).await;
    // Wait for the spawned web server task to throw error.
    let _ = webserver_handle.await?;
    Ok(())
}
