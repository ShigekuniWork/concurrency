use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            for _ in 0..1000 {
                let _ = 1 + 1;
            }
        }
    });

    println!("Please enter a command: help, stop");
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("Available commands: help, stop"),
            "stop" => {
                STOP.store(true, Relaxed);
                break;
            }
            cmd => println!("Unknown command: {}", cmd),
        }
    }

    STOP.store(true, Relaxed);
    background_thread.join().unwrap();
}
