use bson::oid::ObjectId;
use std::{collections::HashMap, str::FromStr};
use validator::ValidationErrors;

use super::auth::errors::AuthResult;

pub fn transform_errors<'a>(errors: ValidationErrors) -> HashMap<&'a str, Option<String>> {
    errors
        .field_errors()
        .into_iter()
        .map(|(key, error)| {
            (
                key,
                error
                    .iter()
                    .find(|err| err.message.is_some())
                    .map(|val| val.to_string()),
            )
        })
        .collect::<HashMap<&str, Option<String>>>()
}

pub fn string_to_oid(s: &str) -> AuthResult<ObjectId> {
    ObjectId::from_str(s).or(Err(super::auth::errors::AuthError::InvalidCredentials))
}
