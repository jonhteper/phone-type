//! Phone type for Rust.

use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::{Arc, LazyLock};

#[cfg(feature = "e164")]
include!(concat!(env!("OUT_DIR"), "/country_codes.rs"));

#[cfg(feature = "e164")]
static E_164_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\+(\d{1,15})$").unwrap());

static WITH_COUNTRY_CODE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\+?(?P<country_code>[1-9]\d{0,2})[\s\-\.]?\(?(\d{2,4})\)?[\s\-\.]?(\d{2,4})[\s\-\.]?(\d{2,4})$").unwrap()
});

static WITHOUT_COUNTRY_CODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\(?(\d{2,4})\)?[\s\-\.]?(\d{2,4})[\s\-\.]?(\d{2,4})$").unwrap());

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub struct Phone {
    country_code: Option<Arc<str>>,
    number: Arc<str>,
}

impl Phone {
    pub fn build(phone: &str) -> Result<Self, Error> {
        match Phone::build_with_country_code(phone) {
            Ok(phone) => Ok(phone),
            Err(_) => Phone::build_without_country_code(phone),
        }
    }

    pub fn build_with_country_code(input: &str) -> Result<Self, Error> {
        let captures = WITH_COUNTRY_CODE_REGEX
            .captures(input)
            .ok_or(Error::InvalidPhoneNumber)?;

        let country_code = captures
            .name("country_code")
            .map(|code| Arc::from(code.as_str()));

        let number: String = [&captures[2], &captures[3], &captures[4]]
            .iter()
            .flat_map(|s| s.chars())
            .collect();
        let number = Arc::from(number);

        let phone = Phone {
            country_code,
            number,
        };

        Ok(phone)
    }

    pub fn build_without_country_code(input: &str) -> Result<Self, Error> {
        let captures = WITHOUT_COUNTRY_CODE_REGEX
            .captures(input)
            .ok_or(Error::InvalidPhoneNumber)?;

        let number: String = [&captures[1], &captures[2], &captures[3]]
            .iter()
            .flat_map(|s| s.chars())
            .collect();
        let number = Arc::from(number);

        let phone = Phone {
            country_code: None,
            number,
        };

        Ok(phone)
    }

    pub fn country_code(&self) -> Option<&str> {
        self.country_code.as_deref()
    }

    pub fn number(&self) -> &str {
        &self.number
    }

    pub fn number_with_separator(&self, separator: char) -> String {
        let number = &self.number;
        let len = number.len();

        if len < 4 {
            return number.to_string();
        }

        // Format based on common patterns
        match len {
            10 => self.ten_digits_with_separator(separator),
            11 => self.eleven_digits_with_separator(separator),
            _ => {
                // For other lengths, insert separator every 3 digits
                let mut result = String::with_capacity(number.len() + (number.len() / 3));
                for (i, c) in number.chars().enumerate() {
                    if i > 0 && i % 3 == 0 {
                        result.push(separator);
                    }
                    result.push(c);
                }
                result
            }
        }
    }

    // Format as XXX-XXX-XXXX
    fn ten_digits_with_separator(&self, separator: char) -> String {
        format!(
            "{}{}{}{}{}",
            &self.number[0..3],
            separator,
            &self.number[3..6],
            separator,
            &self.number[6..10]
        )
    }

    // Format as X-XXX-XXX-XXXX
    fn eleven_digits_with_separator(&self, separator: char) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            &self.number[0..1],
            separator,
            &self.number[1..4],
            separator,
            &self.number[4..7],
            separator,
            &self.number[7..11]
        )
    }

    #[cfg(feature = "e164")]
    pub fn from_e_164(s: &str) -> Result<Self, Error> {
        if !E_164_REGEX.is_match(s) {
            return Err(Error::NotE164Format);
        }

        if let Some((code, national_number)) = parse_e164(s) {
            let phone = Phone {
                country_code: Some(Arc::from(code)),
                number: Arc::from(national_number),
            };
            return Ok(phone);
        }

        Err(Error::NotE164Format)
    }

    #[cfg(feature = "e164")]
    pub fn country_info(&self) -> Option<&'static CountryInfo> {
        self.country_code
            .as_ref()
            .and_then(|code| find_country_info(code))
    }
}

impl Display for Phone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref country_code) = self.country_code {
            write!(f, "+{}-{}", country_code, self.number)
        } else {
            write!(f, "{}", self.number)
        }
    }
}

impl FromStr for Phone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Phone::build(s)
    }
}

impl TryFrom<String> for Phone {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Phone::build(&s)
    }
}

impl From<Phone> for String {
    fn from(p: Phone) -> Self {
        p.to_string()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Invalid phone with country code")]
    InvalidPhoneNumberWithCountryCode,

    #[error("Invalid phone number")]
    InvalidPhoneNumber,

    #[cfg(feature = "e164")]
    #[error("Not E.164 format")]
    NotE164Format,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_works() {
        let phone_result = Phone::build("111-111-1111");
        assert!(phone_result.is_ok(), "Invalid generic phone");

        let phone_result = Phone::build_with_country_code("+52 111 111 1111");
        assert!(phone_result.is_ok(), "Invalid phone with country code");

        let phone_result = Phone::from_str("111 111 1111");
        assert!(phone_result.is_ok());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_works() {
        let phone = Phone::build("111-111-1111").unwrap();
        let serialized = serde_json::to_string(&phone).unwrap();
        let deserialized: Phone = serde_json::from_str(&serialized).unwrap();
        assert_eq!(phone, deserialized);
    }
}

#[cfg(test)]
#[cfg(feature = "e164")]
pub mod e_164_tests {
    use super::*;

    #[test]
    fn test_from_e_164() {
        let phone = Phone::from_e_164("+1234567890").unwrap();
        assert_eq!(phone.country_code(), Some("1"));
        assert_eq!(phone.number(), "234567890");

        let phone = Phone::from_e_164("+521234567890").unwrap();
        assert_eq!(phone.country_code(), Some("52"));
        assert_eq!(phone.number(), "1234567890");
    }

    #[test]
    fn test_new_with_country_code() {
        let phone =
            Phone::build_with_country_code("+52 (55) 1234-5678").expect("Failed to create phone");
        assert_eq!(phone.country_code(), Some("52"));
        assert_eq!(phone.number(), "5512345678");
    }

    #[test]
    fn test_new_without_country_code() {
        let phone = Phone::build_without_country_code("(55) 1234-5678").unwrap();
        assert_eq!(phone.country_code(), None);
        assert_eq!(phone.number(), "5512345678");

        let phone = Phone::build_without_country_code("555 123 4567").unwrap();
        assert_eq!(phone.country_code(), None);
        assert_eq!(phone.number(), "5551234567");
    }

    #[test]
    fn test_with_separator() {
        let phone = Phone::from_e_164("+15551234567").unwrap();
        let formatted = phone.number_with_separator('-');
        assert!(formatted.contains("-"));

        let phone = Phone::build_without_country_code("1234567890").unwrap();
        let formatted = phone.number_with_separator(' ');
        assert!(formatted.contains(" "));
    }

    #[test]
    fn test_error_cases() {
        assert!(Phone::from_e_164("invalid").is_err());
        assert!(Phone::from_e_164("+").is_err());
        assert!(Phone::from_e_164("123456").is_err());
        assert!(Phone::from_e_164("+123ABCDEF").is_err());

        assert!(Phone::build_with_country_code("invalid").is_err());
        assert!(Phone::build_without_country_code("12").is_err());
    }

    #[test]
    fn test_various_formats() {
        // Different E.164 formats
        let phone1 = Phone::from_e_164("+1234567890").unwrap();
        let phone2 = Phone::from_e_164("+521234567890").unwrap();
        let phone3 = Phone::from_e_164("+49123456789").unwrap();

        assert_eq!(phone1.country_code(), Some("1"));
        assert_eq!(phone2.country_code(), Some("52"));
        assert_eq!(phone3.country_code(), Some("49"));

        // Different input formats with country code
        let phone4 = Phone::build_with_country_code("+1 555 123 4567").unwrap();
        let phone5 = Phone::build_with_country_code("52-555-123-4567").unwrap();

        assert_eq!(phone4.country_code(), Some("1"));
        assert_eq!(phone5.country_code(), Some("52"));
    }

    #[test]
    fn test_country_info() {
        // Test that country information is correctly retrieved
        let phone = Phone::from_e_164("+1234567890").unwrap();
        let info = phone.country_info();
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.name, "Canada"); // First occurrence for code "1"
        assert_eq!(info.iso_code, "CA");

        // Test another country
        let phone_mx = Phone::from_e_164("+521234567890").unwrap();
        let info_mx = phone_mx.country_info();
        assert!(info_mx.is_some());
        let info_mx = info_mx.unwrap();
        assert_eq!(info_mx.name, "Mexico");
        assert_eq!(info_mx.iso_code, "MX");
    }

    #[test]
    fn test_compile_time_country_codes() {
        // Test that the compile-time generated functions work
        let result = parse_e164("+1234567890");
        assert!(result.is_some());
        let (code, number) = result.unwrap();
        assert_eq!(code, "1");
        assert_eq!(number, "234567890");

        // Test invalid format
        let invalid = parse_e164("invalid");
        assert!(invalid.is_none());

        // Test short number
        let short = parse_e164("+123");
        assert!(short.is_none());
    }
}
