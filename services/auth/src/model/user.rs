use crate::authentication::password;

pub struct CredentialsModel<'a> {
    db: &'a database::DB,
}

pub struct Credentials {
    pub id: Option<u32>,
    pub email: Option<String>,
    pub name: String,
    pub hash: String,
}

impl<'a> CredentialsModel<'a> {
    pub async fn new(db: &'a mut database::DB) -> CredentialsModel<'a> {
        CredentialsModel { db }
    }
    pub async fn by_name(&mut self, name: &str) -> Result<Credentials, String> {
        let mut client = self.db.client().await?;
        let statement = client
            .prepare("SELECT * FROM auth.credentials WHERE name = $1")
            .await?;
        client.query(&statement, &[name]).await
    }
}
