use argon2::{Argon2, Params};

pub fn argon2() -> Result<Argon2<'static>, argon2::Error> {
    Ok(Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::new(12288, 3, 2, Some(32))?,
    ))
}
