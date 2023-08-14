#[derive(Debug, PartialEq, Eq)]
pub enum CipherError {
    /// All characters in `Key` must be from the english alphabet
    OnlyAlphabetsAllowed,
    /// Key length must be greater than zero
    InvalidKeyLength,
    /// Inputs must be the same length
    MustBeSameLength,
    /// Bytes to string conversion error
    BytesToStringError,
}
