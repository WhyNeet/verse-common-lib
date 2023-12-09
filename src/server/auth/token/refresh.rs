use std::ops::Add;

use bson::oid::ObjectId;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshToken {
    // Refresh token ID (for blacklisting)
    pub sub: String,
    // User's ID
    pub uuid: String,
    exp: i64,
    iat: i64,
}

impl RefreshToken {
    pub fn new(uuid: String, max_age: Duration) -> Self {
        let iat = Utc::now();
        let exp = iat.add(max_age);

        Self {
            /// Generate token ID using Mongo's ObjectId for later storage.
            sub: ObjectId::new().to_hex(),
            uuid,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        }
    }
}
