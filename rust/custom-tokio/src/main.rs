use tokio_util::task::LocalPoolHandle;
use std::cell::RefCell;

thread_local! {
    pub static COUNTER: RefCell<u32> = RefCell::new(1);
}

async fn something(number: u32) -> u32 {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
        println!("Counter: {} for: {}", *counter.borrow(), number);
    });
    number
}
async fn cleanup() {
    println!("cleanup background task started");
    tokio::signal::ctrl_c().await.unwrap();
    println!("ctrl-c received!");
    std::process::exit(0);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(cleanup());

    let pool = LocalPoolHandle::new(3);
    let one = pool.spawn_pinned(|| async {
        println!("one");
        something(1).await
    });
    let two = pool.spawn_pinned(|| async {
        println!("two");
        something(2).await
    });
    let three = pool.spawn_pinned(|| async {
        println!("three");
        something(3).await
    });

    let result = async {
        let one = one.await.unwrap();
        let two = two.await.unwrap();
        let three = three.await.unwrap();
        one + two + three
    };
    println!("result: {}", result.await);
}