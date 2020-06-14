use crate::{utilities::{password, hash}, model, repository, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SaveResults {
    WeakPassword(password::PasswordIssues),
    Success(model::Credentials),
    Conflict,
}

pub async fn create<C: repository::Credentials>(
    credentials: &C,
    request: &model::FullRequest,
) -> Result<SaveResults> {
    let model::FullRequest {
        name,
        email,
        password,
    }: &model::FullRequest = request;
    if let password::Strength::Weak(problems) = password::strength(name, email, password)? {
        Ok(SaveResults::WeakPassword(problems))
    } else {
        match credentials.get_status(name, email).await? {
            repository::CredentialStatus::None => Ok(SaveResults::Success(
                credentials
                    .save_credentials(&model::FullRequest {
                        name: String::from(name),
                        email: String::from(email),
                        password: hash::generate(&password)?,
                    })
                    .await?,
            )),
            _ => Ok(SaveResults::Conflict),
        }
    }
}

#[cfg(test)]
mod credentials_create_test {
    use super::*;
    use crate::utilities::test::fake;
    use actix_rt;

    const WEAK_PASSWORD: &str = "password";

    #[actix_rt::test]
    async fn returns_weak_password_if_the_password_is_too_weak() {
        let mut request = fake::full_request();
        let state = fake::service_state();
        request.password = WEAK_PASSWORD.to_string();
        let result = create(&state.credentials, &request).await.unwrap();
        match result {
            SaveResults::WeakPassword(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[actix_rt::test]
    async fn returns_conflict_if_credentials_already_exist() {
        let request = fake::full_request();
        let mut state = fake::service_state();
        state
            .credentials
            .get_status
            .returns(repository::CredentialStatus::Exists);
        let result = create(&state.credentials, &request).await.unwrap();
        assert_eq!(result, SaveResults::Conflict);
    }

    #[actix_rt::test]
    async fn returns_conflict_if_credentials_are_deleted() {
        let request = fake::full_request();
        let mut state = fake::service_state();
        state
            .credentials
            .get_status
            .returns(repository::CredentialStatus::Deleted);
        let result = create(&state.credentials, &request).await.unwrap();
        assert_eq!(result, SaveResults::Conflict);
    }

    #[actix_rt::test]
    async fn returns_success_if_no_record_exists() {
        let request = fake::full_request();
        let mut state = fake::service_state();
        let credentials = fake::credentials();
        state
            .credentials
            .get_status
            .returns(repository::CredentialStatus::None);
        state
            .credentials
            .save_credentials
            .returns(credentials.clone());
        let result = create(&state.credentials, &request).await.unwrap();
        assert_eq!(result, SaveResults::Success(credentials.clone()));
    }
}
