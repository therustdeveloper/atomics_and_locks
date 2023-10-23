use crate::print_title;
use std::thread;

pub fn master(show: bool) {
    if show {
        print_title("Threads");

        let numbers = vec![1, 2, 3, 4, 5, 6, 7];

        thread::scope(|s| {
            s.spawn(|| {
                println!("length: {}", numbers.len());
            });
            s.spawn(|| {
                for n in &numbers {
                    println!("{n}");
                }
            });
        });
    }
}
