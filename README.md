# xuid

[![Crates.io Version](https://img.shields.io/crates/v/xuid)](https://crates.io/crates/xuid)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/xuid)](https://crates.io/crates/xuid)
[![Crates.io License](https://img.shields.io/crates/l/xuid)](https://github.com/theaddonn/xuid/blob/main/LICENSE)

XUID library for Rust

Well, there is not really much more to say...

## Usage Example

```rust
fn main() {
    let xuid = xuid::xuid!(1234567890);

    println!("{}", xuid);
}
```
