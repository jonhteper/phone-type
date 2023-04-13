use serde::Serialize;

use crate::Phone;


impl Serialize for Phone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::*;
    
    #[derive(Serialize, Debug)]
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
}