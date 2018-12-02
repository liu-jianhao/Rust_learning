use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    // thread
    let mut c = vec![];

    for i in 0..10 {
        c.push(thread::spawn(move || {
            println!("thread number {}", i);
        }));
    }

    for j in c {
        j.join();
    }


    // Channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    println!("got {}", rx.recv().unwrap());


    // mutex
    let c = Arc::new(Mutex::new(0));
    let mut hs = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&c);
        let h = thread::spawn(move || {
            let mut num = c.lock().unwrap();

            *num += 1;
            println!("{}", num);
        });
        hs.push(h);
    }

    for h in hs {
        h.join().unwrap();
    }
    println!("Result: {}", *c.lock().unwrap());
}
