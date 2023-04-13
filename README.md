# Phone-type

[![Crates.io](https://shields.io/crates/v/phone_type.svg)](https://crates.io/crates/phone_type)

This crate contains the `Phone` type, who is only a String wrapper and uses 
[phone-number-verifier](https://crates.io/crates/phone-number-verifier) to valid the phone's format.

## Install

Add in `Cargo.toml`:

```toml
phone_type = "0.3.0"
```

Or run in your project directory:

```bash
cargo add phone_type
```



## Examples

Use in structure:

```rust
use phone_type::*;

struct ContactInformation {
    pub name: String,
    pub age: i8,
    pub phone: Phone,
}

fn main() {
    let info = ContactInformation {
        name: "John Doe".to_string(),
        age: 33,
        phone: Phone::new("111 111 1111").unwrap(),
    };
    /*...*/
}
```
Force type in constructor:
```rust
use phone_type::*;

struct ContactInformation {
    name: String,
    age: i8,
    phone: String,
}

impl ContactInformation {
    pub fn new(name: String, age: i8, phone: Phone) -> Self {
        Self {
            name,
            age,
            phone: phone.to_string(),
        }
    }
}


fn main() {
    let info = ContactInformation::new(
        "John Doe".to_string(),
        33,
        Phone::new("111 111 1111").unwrap(),
    );
    /*...*/
}
```

## Serde support

The serde support is available behind `serde` feature, and is actived by default. If you dont want this feature, use:

```toml
phone_type = {version = "0.3.0", default-features = false}
```

### Example

```rust
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Contact {
    pub name: String,
    pub phone: Phone,
}

fn main() {
    let contact_json = json!({
        "name": "John Doe",
        "phone": "111 111 1111"
    });

    let contact = Contact {
        name: "John Doe".to_string(),
        phone: Phone::new("111 111 1111").unwrap(),
    };

    let deserialize_result = serde_json::from_value::<Contact>(contact_json).unwrap();

    assert_eq!(&deserialize_result, &contact);

}
```

