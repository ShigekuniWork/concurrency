use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let num = AtomicU32::new(0);
    increment(&num);
    println!("num: {}", num.load(Relaxed));
    increment(&num);
    println!("num: {}", num.load(Relaxed));
    increment(&num);
    println!("num: {}", num.load(Relaxed));
}

fn increment(num: &AtomicU32) {
    let mut current = num.load(Relaxed);
    loop {
        let new = current + 1;
        match num.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => break,
            Err(x) => current = x,
        }
    }
}
