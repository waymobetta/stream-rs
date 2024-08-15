### stream-rs

<!-- [![Latest Version](https://img.shields.io/crates/v/stream-rs.svg?logo=rust)](https://crates.io/crates/stream-rs) -->
<!-- [![Docs - Latest Version](https://docs.rs/stream-rs/badge.svg)](https://docs.rs/stream-rs) -->
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/waymobetta/stream-rs)

A library for streaming text with a configurable delay.

#### Demo
![demo](demo.gif)

#### Usage
```toml
# Cargo.toml
[dependencies]
stream = { git = "https://github.com/waymobetta/stream-rs" }
```

```rust
// main.rs
use stream::stream_str;

stream_str("we built this city on rock and roll", 50);
```

#### `stream_str()`
```rust
// payload: string literal to print
// delay: milliseconds
pub fn stream_str(payload: &str, delay: u64) {}
```

#### Example
```rust
cargo run --example main
```

[MIT](LICENSE)
