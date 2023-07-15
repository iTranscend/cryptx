use cryptx::errors::CipherError;

#[test]
fn ceasar_encrypt() {
    let result = cryptx::ciphers::encrypt::caesar("grFthaxy.^".to_string(), 4);
    assert_eq!(result.unwrap(), "kvJxlebc2b".to_string());
}

#[test]
fn ceasar_decrypt() {
    let result = cryptx::ciphers::decrypt::caesar("kvJxlebc2b".to_string(), 4);
    assert_eq!(result.unwrap(), "grFthaxy.^".to_string());
}

#[test]
fn polyalphabetic_encrypt() {
    let result = cryptx::ciphers::encrypt::polyalphabetic("Theendisalmosthere".into(), "stand");
    assert_eq!(result.unwrap(), "Mbfsrwctopfithlxlf");
}
}
