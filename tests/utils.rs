#[test]
pub fn check_port_util_errors() {
    std::env::set_var("PORT", "abc");

    let port = common::utils::get_server_port();
    assert!(port.is_err());
}

#[test]
pub fn check_port_util() {
    std::env::set_var("PORT", "3000");

    let port = common::utils::get_server_port();
    assert_eq!(port.unwrap(), 3000);
}
