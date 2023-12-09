use common::server::logging;

#[test]
pub fn logging_init_works() {
    let handle = logging::init_logger();

    assert!(handle.is_ok());
}
