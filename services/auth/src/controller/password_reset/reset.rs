use crate::{repository, Result, utilities::hash, model};

pub enum ResetResult {
    Success(model::Credentials),
    InvalidToken,
    NotFound,
}

pub async fn reset_password<R: repository::PasswordResetRequest, C: repository::Credentials>(
    reset_request: &R,
    credentials: &C,
    id: &str,
    reset_token: &str,
    password: &str,
) -> Result<ResetResult> {
    if let Some(request) = reset_request.by_id(id).await? {
        if hash::authenticate(reset_token, &request.reset_token)? {
            let hashed_password = hash::generate(password)?;
            Ok(ResetResult::Success(credentials.update_password_hash(&request.user_id, &hashed_password).await?))
        } else {
            Ok(ResetResult::InvalidToken)
        }
    } else {
        Ok(ResetResult::NotFound)
    }
}