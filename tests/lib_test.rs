use libinjection::{sqli, xss};

#[test]
fn test_sqli() {
    let (is_sqli, fingerprint) = sqli("' OR '1'='1' --").unwrap();
    assert!(is_sqli);
    assert_eq!("s&sos", fingerprint);

    let (is_sqli, fingerprint) = sqli("SELECT * FROM users").unwrap();
    assert!(!is_sqli);
    assert!(fingerprint.is_empty());
}

#[test]
fn test_xss() {
    let is_xss = xss("<script type='text/javascript'>alert('xss');</script>").unwrap();
    assert!(is_xss);

    let is_xss = xss("is not XSS").unwrap();
    assert!(!is_xss);
}
