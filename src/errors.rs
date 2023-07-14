#[derive(Debug)]
pub enum CipherError {
    /// All characters in `Key` must be from the english alphabet
    OnlyAlphabetsAllowed,
    /// Key length must be greater than zero
    InvalidKeyLength,
}
