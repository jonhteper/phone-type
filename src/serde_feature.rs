use serde::{
    de::{Error, Unexpected, Visitor},
    Serialize, Deserialize,
};

use crate::Phone;

impl Serialize for Phone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

pub struct PhoneVisitor;

impl<'de> Visitor<'de> for PhoneVisitor {
    
    type Value = Phone;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string whith a phone structure")
    }

    fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
        where
            E: Error,
    {
        let phone_result = Phone::new_with_country(str)
            .map_err(|_| Error::invalid_value(Unexpected::Str(str), &self));

        if let Err(_e) = phone_result {
            return Phone::new_with_country(str)
                .map_err(|_| Error::invalid_value(Unexpected::Str(str), &self));
        }

        phone_result
        
    }

    fn visit_string<E>(self, str: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(&str)
    }




}

impl<'de> Deserialize<'de> for Phone {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PhoneVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    use serde_json::json;

    use crate::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Contact {
        pub name: String,
        pub phone: Phone,
    }

    #[test]
    fn serialize_works() {
        let contact = Contact {
            name: "John Doe".to_string(),
            phone: Phone::new("111 111 1111").unwrap(),
        };

        let result = serde_json::to_string(&contact);
        assert!(result.is_ok());
        println!("{:?}", result.unwrap());
    }

    #[test]
    fn deserialize_works() {
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
        println!("{:?}", &deserialize_result);
    }

     #[test]
    fn deserialize_fails_correctly() {
        let bad_values = [
        json!({
            "name": "John Doe",
            "phone": "+52 111 111 11"
        }),

        json!({
            "name": "John Doe",
            "phone": ""
        }),

        json!({
            "name": "John Doe",
            "phone": "text"
        }),

        json!({
            "name": "John Doe",
            "phone": ["111 111 11111"]
        }),
        
        ];


        for value in bad_values {
            serde_json::from_value::<Contact>(value)
                .expect_err("deserialize must fail");
        }
    }

}
