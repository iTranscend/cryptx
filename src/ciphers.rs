pub mod encrypt {
    pub fn caesar(text: String, shift: u8) -> String {
        let mut cipher = String::from("");
        for c in text.chars() {
            let shifted_ascii_number: u8;
            let char_ascii = c as u8;
            if c.is_lowercase() {
                shifted_ascii_number = (char_ascii + shift - 97) % 26 + 97;
            } else if c.is_uppercase() {
                shifted_ascii_number = (char_ascii + shift - 65) % 26 + 65;
            } else {
                shifted_ascii_number = char_ascii + shift;
            }
            cipher.push(char::from(shifted_ascii_number));
        }
        cipher
    }
}

/// Decryption functions 
pub mod decrypt {
    /// Decrypter for a Caesar cipher.
    pub fn caesar(cipher: String, shift: i8) -> String {
        let mut plaintext = String::from("");
        for c in cipher.chars() {
            let shifted_ascii_number: i8;
            let char_ascii = c as i8;
            if c.is_lowercase() {
                let direction = (char_ascii - shift - 97) % 26;
                if direction.is_negative() {
                    shifted_ascii_number = 122 + 1 + direction;
                } else {
                    shifted_ascii_number = 97 + direction;
                }
            } else if c.is_uppercase() {
                let direction = (char_ascii - shift - 65) % 26;
                if direction.is_negative() {
                    shifted_ascii_number = 90 + 1 + direction;
                } else {
                    shifted_ascii_number = 65 + direction;
                }
            } else {
                shifted_ascii_number = char_ascii - shift;
            }
            plaintext.push(char::from(shifted_ascii_number as u8));
        }
        plaintext
    }
}
