use crate::{model, Result, utilities::hash, model::{credentials, password_reset}};
use async_trait::async_trait;
use std::marker::{Send, Sync};

pub type AppPasswordReset = PasswordReset<model::DatabaseConnection>;

#[derive(Clone, Debug)]
pub struct PasswordReset<T: model::Database> {
    db: T,
}

impl<T: model::Database> PasswordReset<T> {
    pub fn new (db: T) -> Self { PasswordReset { db } }
    pub async fn generate(&self, email: &str) -> Result<Option<model::PasswordResetRequest>> {
        let client = self.db.client().await?;
        let reset_token = hash::token();
        let id = hash::token();
        let hashed_token = hash::generate(&reset_token)?;
        let credentials_by_email = client.prepare(credentials::query::EMAIL).await?;
        let password_rest_request = client.prepare(password_reset::query::CREATE_REQUEST).await?;
        if let Some(credentials) = client.query::<model::Credentials>(&credentials_by_email, &[&email])
            .await?
            .first() {
            Ok(client.query::<model::PasswordResetRequest>(&password_rest_request, &[&id, &credentials.id, &hashed_token])
                .await?
                .first()
                .map(| request | model::PasswordResetRequest {
                    id,
                    reset_token,
                    user_id: credentials.id,
                    created_at: request.created_at,
                }))
        } else {
            Ok(None)
        }
    }
    pub async fn by_id(&self, id: &str) -> Result<Option<model::PasswordResetRequest>> {
        let client = self.db.client().await?;
        let request_by_id = client.prepare(password_reset::query::GET_REQUEST_BY_ID).await?;
        Ok(Some(client.query::<model::PasswordResetRequest>(&request_by_id, &[&id])
            .await?
            .remove(0)))
    }
}

#[async_trait]
pub trait PasswordResetRequest: Send + Sync + Clone {
    async fn generate(&self, email: &str) -> Result<Option<model::PasswordResetRequest>>;
    async fn by_id(&self, id: &str) -> Result<Option<model::PasswordResetRequest>>;
}

#[async_trait]
impl<T: model::Database> PasswordResetRequest for PasswordReset<T> {
    async fn generate(&self, email: &str) -> Result<Option<model::PasswordResetRequest>> {
        let client = self.db.client().await?;
        let reset_token = hash::token();
        let id = hash::token();
        let hashed_token = hash::generate(&reset_token)?;
        let credentials_by_email = client.prepare(credentials::query::EMAIL).await?;
        let password_rest_request = client.prepare(password_reset::query::CREATE_REQUEST).await?;
        if let Some(credentials) = client.query::<model::Credentials>(&credentials_by_email, &[&email])
            .await?
            .first() {
            Ok(client.query::<model::PasswordResetRequest>(&password_rest_request, &[&id, &credentials.id, &hashed_token])
                .await?
                .first()
                .map(| request | model::PasswordResetRequest {
                    id,
                    reset_token,
                    user_id: credentials.id,
                    created_at: request.created_at,
                }))
        } else {
            Ok(None)
        }
    }
    async fn by_id(&self, id: &str) -> Result<Option<model::PasswordResetRequest>> {
        let client = self.db.client().await?;
        let request_by_id = client.prepare(password_reset::query::GET_REQUEST_BY_ID).await?;
        Ok(Some(client.query::<model::PasswordResetRequest>(&request_by_id, &[&id])
            .await?
            .remove(0)))
    }
}