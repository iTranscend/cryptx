/// Encryption Module
pub mod encrypt {
    use crate::errors::CipherError::{self, InvalidKeyLength, OnlyAlphabetsAllowed};
    use crate::types::LoopingIter;
    use crate::utils::{
        contains_only_alphabets, shift_character_forward, string_to_aplhabetic_vec,
    };

    /// Encryption function for the Caesar cipher.
    ///
    /// NB: Only for educational purposes.
    /// Do not use for any system where data safety is important.
    pub fn caesar(text: String, shift: u8) -> Result<String, CipherError> {
        let mut cipher = String::from("");
        for c in text.chars() {
            let shifted_ascii_number = shift_character_forward(c, shift);
            cipher.push(char::from(shifted_ascii_number));
        }
        Ok(cipher)
    }

    /// Encryption function for the Polyalphabetic cipher
    pub fn polyalphabetic(plaintext: String, key: &str) -> Result<String, CipherError> {
        let mut cipher = String::from("");

        if !contains_only_alphabets(key) {
            return Err(OnlyAlphabetsAllowed);
        }

        if key.is_empty() {
            return Err(InvalidKeyLength);
        }

        // Get numeric key representation for shifting
        let key_digits: Vec<u8> = string_to_aplhabetic_vec(key);

        // convert key_digits into a looping iterator
        let mut looping_key_digits = LoopingIter::new(key_digits);

        // iterate over all plaintext chars and shift using key_digits_looping iterator
        for c in plaintext.chars() {
            let shift = *looping_key_digits.next().unwrap();
            let shifted_ascii_number = shift_character_forward(c, shift);
            cipher.push(char::from(shifted_ascii_number));
        }
        Ok(cipher)
    }
}

/// Decryption functions
pub mod decrypt {
    use crate::errors::CipherError::{self, InvalidKeyLength, OnlyAlphabetsAllowed};
    use crate::types::LoopingIter;
    use crate::utils::{
        contains_only_alphabets, shift_character_backward, string_to_aplhabetic_vec,
    };

    /// Decrypter for a Caesar cipher.
    pub fn caesar(cipher: String, shift: u8) -> Result<String, CipherError> {
        let mut plaintext = String::from("");
        for c in cipher.chars() {
            let shifted_ascii_number = shift_character_backward(c, &shift);
            plaintext.push(char::from(shifted_ascii_number as u8));
        }
        Ok(plaintext)
    }

    /// Decrypter for Polyalphabetic cipher
    pub fn polyalphabetic(cipher: String, key: &str) -> Result<String, CipherError> {
        let mut plaintext = String::from("");

        if !contains_only_alphabets(key) {
            return Err(OnlyAlphabetsAllowed);
        }

        if key.is_empty() {
            return Err(InvalidKeyLength);
        }

        // Get numeric key representation for shifting
        let key_digits: Vec<u8> = string_to_aplhabetic_vec(key);

        // convert key_digits into a looping iterator
        let mut looping_key_digits = LoopingIter::new(key_digits);

        // iterate over all ciphertext chars and shift backwards using key_digits_looping iterator
        for c in cipher.chars() {
            let shift = looping_key_digits.next().unwrap();
            let shifted_ascii_number = shift_character_backward(c, shift);
            plaintext.push(char::from(shifted_ascii_number));
        }

        Ok(plaintext)
    }
}
