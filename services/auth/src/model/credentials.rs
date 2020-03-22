pub type CredentialId = i32;

#[derive(Debug)]
pub struct Credentials {
    pub id: CredentialId,
    pub email: String,
    pub name: String,
    pub hash: String,
}

impl Credentials {
    fn from_results(rows: database::Results) -> Option<Credentials> {
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

type CredentialResults = Result<Option<Credentials>, database::Error>;

const GET_CREDENTIALS_BY_NAME: &str =
    "SELECT id, email, name, hash FROM auth.credentials WHERE name = $1";
const GET_CREDENTIALS_BY_EMAIL: &str =
    "SELECT id, email, name, hash FROM auth.credentials WHERE name = $1";

pub struct Model<'a> {
    client: database::Client<'a>,
}

impl<'a> Model<'a> {
    pub fn new(client: database::Client<'a>) -> Model {
        Model { client }
    }
    async fn get_by_single_param(&mut self, query: &str, param: &str) -> CredentialResults {
        let statement = self.client.prepare(query).await?;
        let rows = self.client.query(&statement, &[&param]).await?;
        Ok(Credentials::from_results(rows))
    }
    pub async fn by_name(&mut self, name: &str) -> CredentialResults {
        self.get_by_single_param(GET_CREDENTIALS_BY_NAME, name)
            .await
    }
    pub async fn by_email(&mut self, email: &str) -> CredentialResults {
        self.get_by_single_param(GET_CREDENTIALS_BY_EMAIL, email)
            .await
    }
}
