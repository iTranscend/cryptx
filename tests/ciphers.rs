use cryptx::errors::CipherError;

#[test]
fn ceasar_encrypt() {
    let result = cryptx::ciphers::encrypt::caesar("grFthaxy.^".to_string(), 4);
    assert_eq!(result.unwrap(), "kvJxlebc2b".to_string());
}

#[test]
fn ceasar_decrypt() {
    let result = cryptx::ciphers::decrypt::caesar("kvJxlebc2".to_string(), 4);
    assert_eq!(result.unwrap(), "grFthaxy.".to_string());
}

#[test]
fn polyalphabetic_encrypt() {
    let result = cryptx::ciphers::encrypt::polyalphabetic("Theendisalmosthere".into(), "stand");
    assert_eq!(result.unwrap(), "Mbfsrwctopfithlxlf");
}

#[test]
fn polyalphabetic_encrypt_invalid_key_length() {
    let result = cryptx::ciphers::encrypt::polyalphabetic("Theendisalmosthere".into(), "");
    assert_eq!(result, Err(CipherError::InvalidKeyLength));
}

#[test]
fn polyalphabetic_decrypt() {
    let result = cryptx::ciphers::decrypt::polyalphabetic("Mbfsrwctopfithlxlf".into(), "stand");
    assert_eq!(result.unwrap(), "Theendisalmosthere");
}

#[test]
fn one_time_pad_encrypt() {
    let result = cryptx::ciphers::encrypt::one_time_pad("hello", "world");
    assert_eq!(result.unwrap(), "\u{1f}\n\u{1e}\0\u{b}");
}
