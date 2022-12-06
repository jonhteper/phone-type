# Phone-type

This crate contains the `Phone` type, who is only a String wrapper and uses 
[phone-number-verifier](https://crates.io/crates/phone-number-verifier) to valid the 
phone's format.

Example
```rust
use phone_type::*;

let phone_result = Phone::new("111-111-1111");
assert!(phone_result.is_ok());

let phone_result = Phone::new_with_country("+52 111 111 1111");
assert!(phone_result.is_ok(), "Invalid phone with country code");
 ```
