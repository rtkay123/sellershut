use nanoid::nanoid;
/// Alphabet of characters making up an ID
pub const ID_ALPHABET: [char; 36] = [
    'i', '_', '-', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Length of characters in ID
pub const ID_LENGTH: usize = 21;

/// Generates a nanoid (21 characters)
pub fn generate_id() -> String {
    nanoid!(ID_LENGTH, &ID_ALPHABET)
}
