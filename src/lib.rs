//! Phone type for Rust.

use phone_number_verifier::{
    verify_phone_number_with_country_code, verify_phone_number_without_country_code,
};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[cfg(feature = "serde")]
pub mod serde_feature;


#[derive(Debug)]
pub struct ErrorInvalidPhone;

impl Display for ErrorInvalidPhone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: Invalid phone format")
    }
}

impl std::error::Error for ErrorInvalidPhone {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phone(String);

impl Phone {
    pub fn new(phone: &str) -> Result<Self, ErrorInvalidPhone> {
        if !verify_phone_number_without_country_code(phone) {
            return Err(ErrorInvalidPhone);
        }

        Ok(Self(phone.to_string()))
    }

    pub fn new_with_country(phone: &str) -> Result<Self, ErrorInvalidPhone> {
        if !verify_phone_number_with_country_code(phone) {
            return Err(ErrorInvalidPhone);
        }

        Ok(Self(phone.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Phone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Deref for Phone {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_works() {
        let phone_result = Phone::new("111-111-1111");
        assert!(phone_result.is_ok(), "Invalid generic phone");

        let phone_result = Phone::new_with_country("+52 111 111 1111");
        assert!(phone_result.is_ok(), "Invalid phone with country code");
    }
}
