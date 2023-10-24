use crate::print_title;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
pub fn master(show: bool) {
    if show {
        print_title("Thread Mutex");

        mutex_without_dropping_guard(false);

        mutex_with_dropping_guard(true);
    }
}

fn mutex_without_dropping_guard(show: bool) {
    if show {
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
        println!("{}", n.into_inner().unwrap());
    }
}

fn mutex_with_dropping_guard(show: bool) {
    if show {
        let n = Mutex::new(0);
        thread::scope(|s| {
            for _ in 0..10 {
                s.spawn(|| {
                    let mut guard = n.lock().unwrap();
                    for _ in 0..100 {
                        *guard += 1;
                    }
                    drop(guard);
                    thread::sleep(Duration::from_secs(1));
                });
            }
        });
        println!("{}", n.into_inner().unwrap());
    }
}