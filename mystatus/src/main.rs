use chrono::Local;
use std::{thread, time};

fn main() {
    let sleep_time = time::Duration::from_secs(5);
    
    loop {
        let local_time = Local::now();
        println!("{}", local_time.format("%Y-%m-%d W%V %H:%M:%S"));
        thread::sleep(sleep_time);
    }
}
