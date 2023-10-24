use crate::print_title;

mod thread_closure;
mod thread_demo;
mod thread_scope;
mod thread_mutex;
pub fn master(show: bool) {
    if show {
        print_title("Chapter 01 - Basics of Rust Concurrency");

        thread_demo::master(false);
        thread_closure::master(false);
        thread_scope::master(false);
        thread_mutex::master(false);
    }
}
