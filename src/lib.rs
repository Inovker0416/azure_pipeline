#[cfg(all(test, feature = "opt_in"))]
#[test]
fn opt_in_test() {}

#[cfg(all(test, feature = "opt_out"))]
#[test]
fn opt_out_test() {}

#[cfg(test)]
#[test]
fn always_test() {}

#[cfg(test)]
#[test]
fn require_env() {
    std::env::var("ENV_IS_SET").unwrap();
}

#[cfg(all(test, not(feature = "ci")))]
fn must_exist() {
    panic!("ci feature was not enabled for test run");
}

#[cfg(all(test, feature = "ci"))]
fn must_exist() {}

#[cfg(test)]
#[test]
fn test_must_exist() {
    must_exist()
}

#[cfg(test)]
#[test]
fn test_service_running() {
    use std::net::ToSocketAddrs;
    let mut addrs_iter = "test_service:22".to_socket_addrs().unwrap();
    assert_ne!(addrs_iter.next(), None);
}

#[cfg(test)]
#[test]
fn require_setup_file() {
    include_str!("setup.rs");
}

#[cfg(test)]
#[test]
#[ignore]
fn ignored() {}

// check that minrust gets set correctly
#[allow(unused_imports)]
use std::sync::atomic::AtomicU64;
