use crate::print_title;

mod thread_demo;
mod thread_closure;

pub fn master(show: bool) {
    if show {
        print_title("Chapter 01 - Basics of Rust Concurrency");

        thread_demo::master(false);
        thread_closure::master(true);
    }
}