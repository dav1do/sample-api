use std::collections::HashMap;

// would normally use uuid instead of Strings for some of these fields but not storing in a DB or normalizing
#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String, // should be hashed in practice (with salt and maybe pepper) but skipping for now
    pub favorite_cities: HashMap<String, City>, // name -> City
}

impl User {
    /// This should use hashing when we store and check but simple for now
    pub fn verify_password(&self, input_password: &str) -> bool {
        if self.password == input_password {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub struct City {
    pub name: String,
    pub country: String,
}
