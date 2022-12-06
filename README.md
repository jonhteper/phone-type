# Phone-type

[![Crates.io](https://shields.io/crates/v/phone_type.svg)](https://crates.io/crates/phone_type)


This crate contains the `Phone` type, who is only a String wrapper and uses 
[phone-number-verifier](https://crates.io/crates/phone-number-verifier) to valid the 
phone's format.

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


