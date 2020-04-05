use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct CredentialsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl CredentialsRequest {
    pub fn new(
        name: &Option<String>,
        email: &Option<String>,
        password: &Option<String>,
    ) -> CredentialsRequest {
        CredentialsRequest {
            name: match name {
                Some(name) => Some(String::from(name)),
                None => None,
            },
            email: match email {
                Some(email) => Some(String::from(email)),
                None => None,
            },
            password: match password {
                Some(password) => Some(String::from(password)),
                None => None,
            },
        }
    }
}

impl From<web::Json<CredentialsRequest>> for CredentialsRequest {
    fn from(json: web::Json<CredentialsRequest>) -> CredentialsRequest {
        CredentialsRequest {
            password: match &json.password {
                Some(password) => Some(String::from(password)),
                None => None,
            },
            name: match &json.name {
                Some(name) => Some(String::from(name)),
                None => None,
            },
            email: match &json.email {
                Some(email) => Some(String::from(email)),
                None => None,
            },
        }
    }
}

#[cfg(test)]
mod credential_updates_test {
    use super::*;
    use crate::utilities::test as test_helper;
    use actix_rt;
    use serde_json;

    #[actix_rt::test]
    async fn sets_password_to_none_if_not_present_in_json() {
        let (name, email, ..) = test_helper::fake_credentials();
        let json = format!("{{ \"name\": \"{}\", \"email\": \"{}\" }}", &name, &email);
        let result: CredentialsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(
            result,
            CredentialsRequest::new(&Some(name), &Some(email), &None)
        )
    }

    #[actix_rt::test]
    async fn sets_email_to_none_if_not_present_in_json() {
        let (name, _, password) = test_helper::fake_credentials();
        let json = format!(
            "{{ \"name\": \"{}\", \"password\": \"{}\" }}",
            &name, &password
        );
        let result: CredentialsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(
            result,
            CredentialsRequest::new(&Some(name), &None, &Some(password))
        )
    }

    #[actix_rt::test]
    async fn sets_name_to_none_if_not_present_in_json() {
        let (_, email, password) = test_helper::fake_credentials();
        let json = format!(
            "{{ \"password\": \"{}\", \"email\": \"{}\" }}",
            &password, &email
        );
        let result: CredentialsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(
            result,
            CredentialsRequest::new(&None, &Some(email), &Some(password))
        )
    }

    #[actix_rt::test]
    async fn sets_name_and_email_to_none_if_not_present_in_json() {
        let (_, _, password) = test_helper::fake_credentials();
        let json = format!("{{ \"password\": \"{}\" }}", &password);
        let result: CredentialsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(
            result,
            CredentialsRequest::new(&None, &None, &Some(password))
        )
    }

    #[actix_rt::test]
    async fn sets_email_and_password_to_none_if_not_present_in_json() {
        let (_, name, ..) = test_helper::fake_credentials();
        let json = format!("{{ \"name\": \"{}\" }}", &name);
        let result: CredentialsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(result, CredentialsRequest::new(&Some(name), &None, &None))
    }

    #[actix_rt::test]
    async fn sets_name_and_password_to_none_if_not_present_in_json() {
        let (_, email, ..) = test_helper::fake_credentials();
        let json = format!("{{ \"email\": \"{}\" }}", &email);
        let result: CredentialsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(result, CredentialsRequest::new(&None, &Some(email), &None))
    }

    #[actix_rt::test]
    async fn sets_all_fields_to_none_if_none_are_present_in_json() {
        let result: CredentialsRequest = serde_json::from_str("{}").unwrap();
        assert_eq!(result, CredentialsRequest::new(&None, &None, &None))
    }

    #[actix_rt::test]
    async fn sets_all_fields_to_present_values_in_json() {
        let (name, email, password) = test_helper::fake_credentials();
        let json = format!(
            "{{ \"name\": \"{}\", \"email\": \"{}\", \"password\": \"{}\" }}",
            &name, &email, &password
        );
        let result: CredentialsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(
            result,
            CredentialsRequest::new(&Some(name), &Some(email), &Some(password))
        )
    }
}
