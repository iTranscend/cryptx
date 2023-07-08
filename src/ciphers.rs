pub mod encrypt {
    pub fn caesar(text: String, shift: u8) -> String {
        let mut cipher = String::from("");
        for c in text.chars() {
            let shifted_ascii_number: u8;
            let char_ascii = c as u8;
            println!("{char_ascii}");
            if c.is_lowercase() {
                shifted_ascii_number = (char_ascii + shift - 97) % 26 + 97;
            } else if c.is_uppercase() {
                shifted_ascii_number = (char_ascii + shift - 65) % 26 + 65;
            } else {
                shifted_ascii_number = char_ascii + shift;
                println!("{shifted_ascii_number}");
            }
            cipher.push(char::from(shifted_ascii_number));
        }
        cipher
    }
}
