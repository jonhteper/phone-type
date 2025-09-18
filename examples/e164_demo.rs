use phone_type::e_164::Phone;

fn main() {
    println!("=== E.164 Phone Number Parser Demo ===\n");

    // Test various E.164 phone numbers
    let test_numbers = vec![
        "+1234567890",     // US/Canada
        "+52155551234",    // Mexico
        "+4915123456789",  // Germany
        "+33123456789",    // France
        "+81312345678",    // Japan
        "+86138123456789", // China
        "+447912345678",   // UK
        "+61212345678",    // Australia
        "+5511987654321",  // Brazil
        "+919876543210",   // India
    ];

    for number in test_numbers {
        match Phone::from_e_164(number) {
            Ok(phone) => {
                println!("📱 Number: {}", number);
                println!("   Country Code: {:?}", phone.country_code());
                println!("   National Number: {}", phone.number());

                if let Some(info) = phone.country_info() {
                    println!("   Country: {} ({})", info.name, info.iso_code);
                }

                println!("   Formatted with '-': {}", phone.with_separator('-'));
                println!("   Formatted with ' ': {}", phone.with_separator(' '));
                println!();
            }
            Err(e) => {
                println!("❌ Failed to parse {}: {:?}\n", number, e);
            }
        }
    }

    // Demonstrate error handling
    println!("=== Error Cases ===\n");
    let invalid_numbers = vec![
        "123456789", // No country code
        "+123",      // Too short
        "+invalid",  // Non-numeric
        "",          // Empty
    ];

    for invalid in invalid_numbers {
        match Phone::from_e_164(invalid) {
            Ok(_) => println!("✅ Unexpectedly parsed: {}", invalid),
            Err(e) => println!("❌ Correctly rejected '{}': {:?}", invalid, e),
        }
    }

    // Demonstrate compile-time efficiency
    println!("\n=== Performance Demo ===");
    use std::time::Instant;

    let start = Instant::now();
    for _ in 0..10000 {
        let _ = Phone::from_e_164("+1234567890");
    }
    let duration = start.elapsed();

    println!("Parsed 10,000 numbers in {:?}", duration);
    println!("Average: {:?} per number", duration / 10000);
    println!("\n🚀 Country codes are resolved at compile-time using Perfect Hash Functions!");
    println!("💾 No runtime overhead for country code lookup!");
}
