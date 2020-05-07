use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("/home/user/Programming/watcher_test", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
           // Ok(event) => println!("{:?}", event),
            Ok(event) => {
                println!("{:?}", event);
                std::thread::sleep(Duration::from_secs(2));
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}