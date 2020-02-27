pub struct Model<'a> {
    db: &'a database::DB,
}

pub struct Credentials {
    pub id: Option<u32>,
    pub email: Option<String>,
    pub name: String,
    pub hash: String,
}

impl Credentials {
    fn from_results(rows: database::Results) -> Credentials {
        rows.iter()
            .map(|row| Credentials {
                id: Some(row.get(0)),
                email: Some(row.get(1)),
                name: row.get(2),
                hash: row.get(3),
            })
            .collect()[0]
    }
}

const GET_CREDENTIALS_BY_NAME: &str = "SELECT * FROM auth.credentials WHERE name = $1";

impl<'a> Model<'a> {
    pub fn new(db: &'a mut database::DB) -> Model<'a> {
        Model { db }
    }
    pub async fn by_name(&mut self, name: &str) -> Result<Credentials, database::DBError> {
        let mut client = self.db.client().await?;
        let statement = client.prepare(GET_CREDENTIALS_BY_NAME).await?;
        let rows = client.query(&statement, &[name]).await.unwrap();
        Ok(Credentials::from_results(rows))
    }
}
