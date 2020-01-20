pub struct User {
    pub id: u32,
    pub name: String,
    pub hash: String,
}

pub fn by_name(name: &str) -> User {
    User {
        id: 50,
        name: name.to_string(),
        hash: String::from("blahblahblah"),
    }
}
