use std::time::Duration;
use std::{sync::atomic::AtomicUsize, thread};

use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..100 {
                for _ in 0..1000 {
                    let _ = 1 + 1;
                }
                num_done.fetch_add(1, Relaxed);
                main_thread.unpark();
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working... {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done")
}
