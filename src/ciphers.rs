/// Encryption Module
pub mod encrypt {
    use crate::types::LoopingIter;
    
    /// Encryption function for the Caesar cipher.
    ///
    /// NB: Only for educational purposes.
    /// Do not use for any system where data safety is important.
    pub fn caesar(text: String, shift: u8) -> String {
        let mut cipher = String::from("");
        for c in text.chars() {
            let shifted_ascii_number = shift_character(c, shift);
            cipher.push(char::from(shifted_ascii_number));
        }
        cipher
    }

    /// Encryption function for the Polyalphabetic cipher
    pub fn polyalphabetic(plaintext: String, key: &str) -> String {
        let mut cipher = String::from("");

        if !contains_only_alphabets(key) {
            // Return err OnlyAlphabetsAllowed
        }

        if key.len() <= 0 {
            // Return err invalid key length
        }

        let mut key_digits: Vec<u8> = vec![];

        // Get numeric key representation for shifting
        for c in key.to_ascii_lowercase().chars() {
            key_digits.push(c as u8 - 96);
        }

        // convert key_digits into a looping iterator
        let mut looping_key_digits = LoopingIter::new(key_digits);

        // iterate over all plaintext chars and shift using key_digits_looping iterator
        for c in plaintext.chars() {
            let shift = *looping_key_digits.next().unwrap();
            let shifted_ascii_number = shift_character(c, shift);
            cipher.push(char::from(shifted_ascii_number));
        }
        cipher
    }

    fn contains_only_alphabets(string: &str) -> bool {
        string.chars().all(|c| c.is_alphabetic())
    }

    fn shift_character(c: char, shift: u8) -> u8 {
        let shifted_ascii_number: u8;
        let char_ascii = c as u8;
        if c.is_lowercase() {
            shifted_ascii_number = (char_ascii + shift - 97) % 26 + 97;
        } else if c.is_uppercase() {
            shifted_ascii_number = (char_ascii + shift - 65) % 26 + 65;
        } else {
            shifted_ascii_number = char_ascii + shift;
        }
        shifted_ascii_number
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
