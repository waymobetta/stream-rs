use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

pub fn stream_str(payload: &str, delay: u64) {
    let delay_duration = Duration::from_millis(delay);

    for char in payload.chars() {
        sleep(delay_duration);
        print!("{}", char);
        io::stdout().flush().unwrap();
    }
    println!("\r");
}
