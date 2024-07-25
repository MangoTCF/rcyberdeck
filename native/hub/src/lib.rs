//! This `hub` crate is the
//! entry point of the Rust logic.

mod common;
mod messages;

use std::{thread::sleep, time::Duration};

use crate::common::*;
use messages::basic::{AppDirs, DartReady};
use rinf::debug_print;
use tokio; // Comment this line to target the web.
// use tokio_with_wasm::alias as tokio; // Uncomment this line to target the web.

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    let dirs = get_app_dirs().await.unwrap();
    wait_for_dart().await.unwrap();
    ensure_app_files(dirs).await.unwrap();
    display_bootlog().await.unwrap();
    sleep(Duration::from_millis(500));
    
    tokio::spawn(communicate());
}

async fn get_app_dirs() -> Result<AppDirs> {
    let mut recv = AppDirs::get_dart_signal_receiver()?;
    let dirs = recv.recv().await.expect("Unable to recv AppDirs!").message;
    return Ok(dirs);
}

async fn ensure_app_files(dirs: AppDirs) -> Result<()> {
    debug_print!("Ensuring files");
    debug_print!("tmp_dir is {}, app_support_data_dir is {}, downloads_dir is {}", dirs.tmp_dir, dirs.app_support_data_dir, dirs.downloads_dir);
    Ok(())
}

async fn wait_for_dart() -> Result<()> {
    let mut recv = DartReady::get_dart_signal_receiver()?;
    let _ = recv.recv().await;
    debug_print!("Got DartReady");
    Ok(())
}

async fn display_bootlog() -> Result<()> {
    Ok(())
}

async fn communicate() -> Result<()> {
    use messages::basic::*;
    // Get receivers that listen to Dart signals like below.
    BootLog{ text: "balls".to_owned()}.send_signal_to_dart();
    let mut receiver = BootLogRequest::get_dart_signal_receiver()?;
    while let Some(dart_signal) = receiver.recv().await {
        let message: BootLogRequest = dart_signal.message;
        rinf::debug_print!("{message:?}");
        
    }
    Ok(())
}
