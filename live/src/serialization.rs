use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, Read, Write};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    name: String,
    age: u8,
    gender: Gender,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Gender {
    Undefined = 0,
    Female = 1,
    Male = 2,
}

impl User {
    fn new(name: String, age: u8, gender: Gender) -> Self {
        User { name, age, gender }
    }

    pub fn presist(&self, filename: &str) -> Result<usize, Error> {
        let mut file = File::create(filename)?;
        let serialized = serde_json::to_string(self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(serialized.len())
    }

    pub fn load(&self, filename: &str) -> Result<Self, Error> {
        let mut file = File::open(filename)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let user = serde_json::from_str(&buf)?;
        Ok(user)
    }
}

impl Default for User {
    fn default() -> Self {
        User::new("Unkonw User".to_string(), 0, Gender::Undefined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let filename = "/tmp/user1";
        let user = User::default();
        let _ = user.presist(filename);

        let user1 = user.load(filename).unwrap();
        assert_eq!(user, user1);
    }
}
