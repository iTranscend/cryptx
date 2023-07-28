/// Checks if a string contains only alphabetic characters
pub fn contains_only_alphabets(string: &str) -> bool {
    string.chars().all(|c| c.is_alphabetic())
}

/// Shifts a character backwards in the alphabet
pub fn shift_character_forward(c: char, shift: u8) -> u8 {
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

/// Converts a string to a vector of each  of its characters' numeric alphabetic position
pub fn string_to_aplhabetic_vec(s: &str) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![];
    for c in s.to_ascii_lowercase().chars() {
        vec.push(c as u8 - 96);
    }
    vec
}

/// Shifts a character backwards in the alphabet
pub fn shift_character_backward(c: char, shift: &u8) -> u8 {
    let shifted_ascii_number: u8;
    let char_ascii = c as u8;
    if c.is_lowercase() {
        shifted_ascii_number = (26 - shift + char_ascii - 97) % 26 + 97;
    } else if c.is_uppercase() {
        shifted_ascii_number = (26 - shift + char_ascii - 65) % 26 + 65;
    } else {
        shifted_ascii_number = char_ascii - shift;
    }
    shifted_ascii_number
}
