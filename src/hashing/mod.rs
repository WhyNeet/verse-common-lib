use ::argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    PasswordHash, PasswordHasher, PasswordVerifier,
};
use anyhow::anyhow;

mod argon2;

pub fn hash_password(password: &[u8]) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2::argon2().unwrap();

    let password_hash = argon2
        .hash_password(password, &salt)
        .map_err(|err| anyhow!(err))?;

    Ok(password_hash.to_string())
}

pub fn verify_password(password: &[u8], hash: &str) -> anyhow::Result<()> {
    let argon2 = argon2::argon2().unwrap();

    let password_hash = PasswordHash::new(hash).map_err(|err| anyhow!(err))?;

    argon2
        .verify_password(password, &password_hash)
        .map_err(|err| anyhow!(err))
}
