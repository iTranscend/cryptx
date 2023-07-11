#[test]
fn ceasar_encrypt() {
    let result = cryptx::ciphers::encrypt::caesar("grFthaxy.".to_string(), 4);
    assert_eq!(result, "kvJxlebc2".to_string());
}

#[test]
fn ceasar_decrypt() {
#[test]
fn polyalphabetic_encrypt() {
    let result =
        cryptx::ciphers::encrypt::polyalphabetic("Theendisalmosthere".into(), "stand");
    assert_eq!(result, "Mbfsrwctopfithlxlf")
}
}
