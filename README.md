# hide (from debug)

[![crates.io](https://img.shields.io/crates/v/hide.svg)](https://crates.io/crates/hide)
[![docs.rs](https://docs.rs/hide/badge.svg)](https://docs.rs/hide)

Hide secrets from logs.

## Seriously?

You might ask: "A crate, for a simple feature like this?". Yes, maybe this type will be shared between crates. Compared
to multiple different types in different crates.

## Add to your project

```tom
hide = "0.1
```

## Usage

```rust
use hide::Hide;

#[derive(Debug)]
pub struct MyStruct {
    username: String,
    password: Hide<String>,
}

fn example1() {
    let data = MyStruct {
        username: "user".to_string(),
        password: "password".to_string().into(),
    };
    println!("{data:#?}");
}
```

Will give you:

```
MyStruct {
    username: "user",
    password: ***,
}
```
