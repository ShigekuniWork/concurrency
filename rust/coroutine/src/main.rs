#![feature(coroutine_trait)]

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::time::Instant;

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

use rand::RngExt;

#[allow(dead_code)]
struct WriteCoroutine {
    file_handle: File,
}

#[allow(dead_code)]
impl WriteCoroutine {
    fn new(path: &str) -> io::Result<Self> {
        let file_handle = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { file_handle })
    }
}

impl Coroutine<i32> for WriteCoroutine {
    type Yield = ();
    type Return = ();

    fn resume(mut self: Pin<&mut Self>, arg: i32) -> CoroutineState<Self::Yield, Self::Return> {
        writeln!(self.file_handle, "{}", arg).unwrap();
        CoroutineState::Yielded(())
    }
}

fn main() -> io::Result<()> {
    let mut rng = rand::rng();
    let numbers: Vec<i32> = (0..200000).map(|_| rng.random()).collect();

    let start = Instant::now();
    let mut co = Pin::from(Box::new(WriteCoroutine::new("numbers.txt")?));
    for &number in &numbers {
        match co.as_mut().resume(number) {
            CoroutineState::Yielded(()) => {}
            CoroutineState::Complete(()) => break,
        }
    }
    let duration = start.elapsed();

    println!("Time elapsed in file operations is: {:?}", duration);
    Ok(())
}
