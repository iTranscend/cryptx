#[test]
fn test_ceasar() {
    let result = encryptx::ciphers::encrypt::caesar("grFthaxy.".to_string(), 4);
    assert_eq!(result, "kvJxlebc2".to_string());
}