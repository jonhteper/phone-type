//! Phone type for Rust.

use phone_number_verifier::{
    verify_phone_number_with_country_code, verify_phone_number_without_country_code,
};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ErrorInvalidPhone;

impl Display for ErrorInvalidPhone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: invalid phone format")
    }
}

impl std::error::Error for ErrorInvalidPhone {}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
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

impl FromStr for Phone {
    type Err = ErrorInvalidPhone;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Phone::new(s) {
            Ok(p) => Ok(p),
            Err(_) => Phone::new_with_country(s),
        }
    }
}

impl TryFrom<String> for Phone {
    type Error = ErrorInvalidPhone;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Phone::from_str(&s)
    }
}

impl From<Phone> for String {
    fn from(p: Phone) -> Self {
        p.to_string()
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

        let phone_result = Phone::from_str("111 111 1111");
        assert!(phone_result.is_ok());
    }
}
