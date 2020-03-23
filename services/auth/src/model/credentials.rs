pub type CredentialId = i32;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub id: CredentialId,
    pub email: String,
    pub name: String,
    pub hash: String,
}

impl Credentials {
    pub fn from(rows: database::Results) -> Option<Credentials> {
        match rows.first() {
            Some(result) => Some(Credentials {
                id: result.get(0),
                email: result.get(1),
                name: result.get(2),
                hash: result.get(3),
            }),
            None => None,
        }
    }
}
