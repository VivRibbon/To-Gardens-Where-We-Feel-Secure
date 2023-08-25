use std::time::Duration;
use std::thread::sleep;
use std::io::{stdout, Write};

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}


pub fn printout(text: &str, delay: u64) {
    for c in text.chars() {
        print!("{}", c);
        stdout().flush().unwrap();
        sleep(Duration::from_millis(delay));
    }
}

