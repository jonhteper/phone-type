# Phone-type

[![Crates.io](https://shields.io/crates/v/phone_type.svg)](https://crates.io/crates/phone_type)

A comprehensive Rust crate for handling phone numbers with advanced E.164 support and compile-time country code resolution.

## Features

- 🔢 **Basic Phone Validation**: String wrapper with format validation
- 📱 **E.164 Support**: Full E.164 international phone number parsing
- 🌍 **Compile-time Country Codes**: Perfect Hash Functions for zero-runtime-cost country lookup
- 🚀 **Performance**: Country codes resolved at compile time using official data
- 📋 **Rich Metadata**: Country names, ISO codes, and formatting options
- 🔧 **Serde Support**: Built-in serialization/deserialization (feature-gated)

## Install

Add in `Cargo.toml`:

```toml
phone_type = "1.0.0-beta.1"
```

Or run in your project directory:

```bash
cargo add phone_type
```

## Examples

### Basic Usage

Use in structures:

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
        phone: Phone::build("111 111 1111").unwrap(),
    };
    /*...*/
}
```

### E.164 International Phone Numbers
*Important*: Enable `e164` feature first.

```rust
use phone_type::Phone;

fn main() {
    // Parse E.164 format numbers
    let phone = Phone::from_e_164("+1234567890").unwrap();

    println!("Country code: {:?}", phone.country_code());      // Some("1")
    println!("National number: {}", phone.number());           // "234567890"

    // Get country information
    if let Some(info) = phone.country_info() {
        println!("Country: {} ({})", info.name, info.iso_code); // "Canada (CA)"
    }

    // Format with separators
    println!("Formatted: {}", phone.number_with_separator('-'));     // "234-567-890"
}
```

### Advanced E.164 Examples

```rust
use phone_type::Phone;

fn main() {
    let examples = vec![
        "+52155551234",    // Mexico
        "+4915123456789",  // Germany
        "+81312345678",    // Japan
        "+86138123456789", // China
    ];

    for number in examples {
        match Phone::from_e_164(number) {
            Ok(phone) => {
                if let Some(info) = phone.country_info() {
                    println!("{}: {} ({})",
                        number, info.name, info.iso_code);
                }
            }
            Err(e) => println!("Invalid: {} - {:?}", number, e),
        }
    }
}
```

## Features

This crate provides the following features:

- `serde` - Serialization/deserialization support
- `e164` - E.164 international phone number support with compile-time country codes

**Important**: No features are enabled by default. You must explicitly enable the features you need.

### Enable Specific Features

```toml
phone_type = { version = "1.0.0-beta.1", features = ["serde", "e164"] }
```

## Performance

The E.164 functionality uses Perfect Hash Functions (PHF) to resolve country codes at **compile time**. This means:

- 🚀 **Zero runtime cost** for country code lookup
- 📊 **O(1) lookup time** for any country code
- 💾 **Minimal memory footprint** - data embedded in binary
- 🔄 **No external dependencies** at runtime

```rust
// This lookup happens at compile time!
let phone = Phone::from_e_164("+1234567890").unwrap();
let country = phone.country_info().unwrap(); // Instant lookup
```

## Serde Support

Serde support is available behind the `serde` feature:

```rust
use serde::{Serialize, Deserialize};
use serde_json::json;
use phone_type::*;

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

    let contact: Contact = serde_json::from_value(contact_json).unwrap();
    println!("{:?}", contact);
}
```

## Country Data Source

The E.164 country codes are sourced from official telecommunications data and compiled into the binary at build time, ensuring:

- ✅ **Accuracy**: Based on official ITU-T standards
- 🔄 **Completeness**: Covers all active country codes
- 🛡️ **Reliability**: No network dependencies
- 🎯 **Efficiency**: Optimized for performance

## Examples

Run the included examples:

```bash
cargo run --example e164_demo --features e164
```

## Contributing

Contributions are welcome! The country code data can be updated by modifying `data/country_codes.json` and rebuilding.
