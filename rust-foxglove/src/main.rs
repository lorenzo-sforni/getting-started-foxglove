use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread, time,
};

use foxglove::{McapWriter, log};
use foxglove::schemas::Log;

#[derive(foxglove::Encode)]
struct Message {
    elapsed: f64,
}

fn main() {
    foxglove::WebSocketServer::new()
        .start_blocking()
        .expect("Server failed to start");

    // Keep a reference to the writer. It'll automatically flush and close when it's dropped,
    // or we could call `.close()` to close it manually.
    // We use a named variable here to ensure it's dropped only at the end of the scope.
    let _writer = foxglove::McapWriter::new()
        .create_new_buffered_file("test.mcap")
        .expect("Failed to create writer");

    // Log until interrupted. We need a ctrlc handler here to ensure
    // that main() exits cleanly, dropping _writer before ending the process.
    let done = Arc::new(AtomicBool::default());
    ctrlc::set_handler({
        let done = done.clone();
        move || {
            done.store(true, Ordering::Relaxed);
        }
    })
    .expect("Failed to set SIGINT handler");

    let start = time::SystemTime::now();
    while !done.load(Ordering::Relaxed) {
        log!("/log", Log{
    message: "Hello, Foxglove!".to_string(),
    ..Default::default()
    });
        thread::sleep(time::Duration::from_millis(30));
    }
}