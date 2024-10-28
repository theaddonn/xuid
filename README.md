# xuid

[![Crates.io Version](https://img.shields.io/crates/v/xuid)](https://crates.io/crates/xuid)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/xuid)](https://crates.io/crates/xuid)
[![Crates.io License](https://img.shields.io/crates/l/xuid)](https://github.com/theaddonn/xuid/blob/main/LICENSE)

XUID library for Rust

_What even is an XUID?_

XUIDs, or Xbox User IDs, are unique 64-bit unsigned integers assigned to Xbox Live accounts to identify users across Microsoft's gaming platforms. These IDs are used in various services, such as Xbox Live multiplayer games, cloud storage, and profile management, to reference a user without relying on a username, which can change over time. XUIDs ensure that every user is uniquely identified and remain consistent even if a gamertag is updated.

They're widely used in games like Minecraft: Bedrock Edition, where XUIDs can link in-game players to their Xbox Live accounts, enabling cross-platform play and access to Xbox Live services.

## Usage Example

```rust
fn main() {
    let xuid = xuid::xuid!(1234567890);

    println!("{}", xuid);
}
```
