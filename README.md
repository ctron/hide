# hide

Hide secrets from logs.

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
