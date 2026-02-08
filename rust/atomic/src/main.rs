use std::time::Duration;
use std::{sync::atomic::AtomicUsize, thread};

use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..100 {
                num_done.fetch_add(1, Relaxed);
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Not done, n = {}", n);
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done")
}
