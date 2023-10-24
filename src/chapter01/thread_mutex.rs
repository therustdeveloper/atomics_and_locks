use crate::print_title;
use std::sync::Mutex;
use std::thread;

pub fn master(show: bool) {
    if show {
        print_title("Thread Mutex");

        let n = Mutex::new(0);
        thread::scope(|s| {
            for _ in 0..10 {
                s.spawn(|| {
                    let mut guard = n.lock().unwrap();
                    for _ in 0..100 {
                        *guard += 1;
                    }
                });
            }
        });
        println!("{}", n.into_inner().unwrap());
    }
}