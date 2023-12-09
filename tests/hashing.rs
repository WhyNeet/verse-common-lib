use common::hashing;

#[test]
pub fn password_hashing_works() {
    let password = b"12345678";

    let password_hash = hashing::hash_password(password);
    assert!(password_hash.is_ok());

    let password_hash = password_hash.unwrap();

    let password_verified = hashing::verify_password(password, &password_hash);
    assert!(password_verified.is_ok());
}
