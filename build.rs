use phf_codegen::Map;
use serde::{Deserialize, Serialize};

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct CountryCode {
    name: String,
    dial_code: String,
    code: String,
}

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("country_codes.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    // Read and parse the country codes JSON file
    let country_codes_json = include_str!("data/country_codes.json");
    let country_codes: Vec<CountryCode> =
        serde_json::from_str(country_codes_json).expect("Failed to parse country codes JSON");

    // Create a map from dial code (without +) to country info
    // Handle duplicates by keeping the first occurrence
    let mut dial_code_map = Map::new();
    let mut seen_codes = std::collections::HashSet::new();

    for country in &country_codes {
        let dial_code = country.dial_code.trim_start_matches('+');

        // Skip duplicates - keep the first occurrence only
        if seen_codes.contains(dial_code) {
            continue;
        }
        seen_codes.insert(dial_code.to_string());

        let country_info = format!(
            r#"CountryInfo {{ name: "{}", iso_code: "{}" }}"#,
            country.name, country.code
        );
        dial_code_map.entry(dial_code, &country_info);
    }

    // Generate the code
    writeln!(
        &mut file,
        r#"
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountryInfo {{
    pub name: &'static str,
    pub iso_code: &'static str,
}}

pub static COUNTRY_CODES: phf::Map<&'static str, CountryInfo> = {};
"#,
        dial_code_map.build()
    )
    .unwrap();

    // Create a sorted list of country codes by length for efficient parsing
    let mut codes_by_length: Vec<String> = country_codes
        .iter()
        .map(|c| c.dial_code.trim_start_matches('+').to_string())
        .collect();

    // Sort by length (longest first) then lexicographically
    codes_by_length.sort_by(|a, b| match b.len().cmp(&a.len()) {
        std::cmp::Ordering::Equal => a.cmp(b),
        other => other,
    });

    // Remove duplicates while preserving order
    codes_by_length.dedup();

    // Generate the ordered country codes array
    writeln!(
        &mut file,
        r#"
/// Country codes sorted by length (longest first) for efficient parsing
pub static ORDERED_COUNTRY_CODES: &[&str] = &{:?};
"#,
        codes_by_length
    )
    .unwrap();

    // Generate helper functions
    writeln!(
        &mut file,
        r#"
/// Find the best matching country code for a given number
pub fn find_country_code(number: &str) -> Option<&'static str> {{
    for &code in ORDERED_COUNTRY_CODES.iter() {{
        if number.starts_with(code) && number.len() >= code.len() + 4 {{
            if let Some(_info) = COUNTRY_CODES.get(code) {{
                return Some(code);
            }}
        }}
    }}
    None
}}

/// Find the country information for a given code
pub fn find_country_info(code: &str) -> Option<&'static CountryInfo> {{
    COUNTRY_CODES.get(code)
}}

/// Parse country code and number from E.164 format
pub(crate) fn parse_e164(number: &str) -> Option<(&'static str, &str)> {{
    let number = number.strip_prefix('+').unwrap_or(number);

    if let Some(code) = find_country_code(number) {{
        let national_number = &number[code.len()..];
        if national_number.len() >= 4 {{
            return Some((code, national_number));
        }}
    }}
    None
}}
"#
    )
    .unwrap();

    println!("cargo:rerun-if-changed=data/country_codes.json");
}
