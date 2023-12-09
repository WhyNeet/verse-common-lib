use super::constants::{JWT_AT_MAX_AGE, JWT_RT_MAX_AGE, JWT_SECRET};

pub fn setup_env() {
    std::env::set_var("JWT_SECRET", JWT_SECRET);
    std::env::set_var("JWT_AT_MAX_AGE", JWT_AT_MAX_AGE);
    std::env::set_var("JWT_RT_MAX_AGE", JWT_RT_MAX_AGE);
}
