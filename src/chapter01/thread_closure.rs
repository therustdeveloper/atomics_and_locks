use crate::print_title;
use std::thread;

pub fn master(show: bool) {
    if show {
        print_title("Threads");

        demo(false);
        demo_closure(false);
    }
}

fn demo(show: bool) {
    if show {
        print_title("Thread Demo");

        let t1 = thread::spawn(f);
        let t2 = thread::spawn(f);

        println!("Hello from the main thread.");

        t1.join().unwrap();
        t2.join().unwrap();
    }
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {:?}", id);
}

fn demo_closure(show: bool) {
    if show {
        print_title("Thread Demo Closure");

        let numbers = vec![1, 2, 3, 4, 5, 6, 7];

        // the ownership of numbers is transferred to the newly spawned thread
        thread::spawn(move || {
            for n in numbers {
                println!("{}", n);
            }
        })
        .join()
        .unwrap();
    }
}
