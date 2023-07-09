#[test]
fn ceasar_encrypt() {
    let result = encryptx::ciphers::encrypt::caesar("grFthaxy.".to_string(), 4);
    assert_eq!(result, "kvJxlebc2".to_string());
}

#[test]
fn ceasar_decrypt() {
    let result = encryptx::ciphers::decrypt::caesar("kvJxlebc2".to_string(), 4);
    assert_eq!(result, "grFthaxy.".to_string());
}
