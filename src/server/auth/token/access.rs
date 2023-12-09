use std::ops::Add;

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    // User's ID
    pub sub: String,
    exp: i64,
    iat: i64,
}

impl AccessToken {
    pub fn new(uuid: String, exp_time: Duration) -> Self {
        let iat = Utc::now();
        let exp = iat.add(exp_time).timestamp();

        Self {
            sub: uuid,
            exp,
            iat: iat.timestamp(),
        }
    }
}
