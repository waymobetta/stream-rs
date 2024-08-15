use stream::stream_str;

fn main() {
    let payload: &str = "we built this city on rock and roll";
    let delay: u64 = 50;

    stream_str(payload, delay);
}
