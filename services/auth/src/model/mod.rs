use postgres::Connection;
pub mod user;

pub struct Auth {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct DbExecuter {
    //    connection: Connection,
}

//impl Actor for DbExecuter {}
