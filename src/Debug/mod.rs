use std::io::{stdout, Write};

static mut INIT: bool = false;
static mut FILTER: Filter = Filter::None;
static mut WARN: u8 = 0;

pub fn init(active: bool, filter: Filter<'static>) {
    unsafe {
        INIT = active;
        FILTER = filter;
        if INIT {
            write_log(&["Engine"], format!("Debug initilaized with '{:?}' filter", FILTER));
        }
    }
}

pub fn log<T: std::fmt::Display + std::fmt::Debug>(tags: &[&str], message: T) {
    unsafe {
        if INIT {
            match &FILTER {
                Filter::None => write_log(tags, message),
                Filter::Allow(filter) => {
                    if tags.iter().any(|item| filter.contains(item)) {
                        write_log(tags, message);
                    }
                },
                Filter::Deny(filter) => {
                    if !tags.iter().any(|item| filter.contains(item)) {
                        write_log(tags, message);
                    }
                },
                Filter::Only(filter) => {
                    if tags.iter().all(|item| filter.contains(item)) {
                        write_log(tags, message);
                    }
                },
                Filter::Not(filter) => {
                    if !tags.iter().all(|item| filter.contains(item)) {
                        write_log(tags, message);
                    }
                },
            }
        } else {
            match WARN {
                1 => write_log(&["Engine"], "Debug not initilaized!"),
                _ => { if !(WARN >= u8::MAX) { WARN += 1; } },
            }
        }
    }
}

#[derive(Debug)]
pub enum Filter<'a> {
    Allow(&'a [&'a str]),
    Deny(&'a [&'a str]),
    Only(&'a [&'a str]),
    Not(&'a [&'a str]),
    None,
}

fn write_log<T: std::fmt::Display + std::fmt::Debug>(tags: &[&str], message: T) {
    let log = format!("[{0}]: {1}\n", tags.join("]["), message);
    let stdout = stdout();
    let mut lock = stdout.lock();
    lock.write_all(log.as_bytes())
        .expect("failed to write whole buffer");
    lock.flush().expect("failed to flush stdout");
}