use diesel::deserialize::Queryable;
use secrecy::Secret;
use serde::Deserialize;
use uuid::Uuid;
use std::convert::TryFrom;
use crate::types::User;
use crate::types::response::IdentityError;
use secrecy::ExposeSecret;

#[derive(Deserialize, Queryable)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: Secret<String>,
}

// // Implement FromRequest

impl TryFrom<CreateUser> for User {
    type Error = IdentityError;

    fn try_from(value: CreateUser) -> Result<Self, Self::Error> {
        Ok(User {
            email: value.email,
            name: value.name,
            hashed_password: value.password.expose_secret().clone(),
            id: Uuid::new_v4()
        })
    }
}

