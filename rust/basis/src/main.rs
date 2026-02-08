use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::Mutex,
    thread,
    time::Duration,
};

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread!");

    t1.join().unwrap();
    t2.join().unwrap();

    let numbers = vec![1, 2, 3, 4, 5];

    thread::spawn(move || {
        for n in numbers {
            println!("Number: {}", n);
        }
    })
    .join()
    .unwrap();

    let a = Rc::new([1, 2, 3, 4, 5]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());

    // cell
    let x = Cell::new(5);
    let y = &x;
    let z = &x;

    y.set(6);
    println!("x: {}", x.get());
    println!("z: {}", z.get());

    // RefCell
    let v = RefCell::new(vec![1, 2, 3, 4, 5]);
    let v_ref = &v;

    v_ref.borrow_mut().push(6);
    println!("v: {:?}", v_ref.borrow());

    // Mutex
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                thread::sleep(Duration::from_secs(1));
            });
        }
    });

    assert_eq!(n.into_inner().unwrap(), 1000);
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("Thread id this thread is {:?}", id);
}
