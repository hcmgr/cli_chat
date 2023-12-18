// miscellaneous helper functions
use sha256::digest;

/*
Returns a sha256 digest for the given array of bytes
*/
pub fn sha256_from_bytes(input: Vec<u8>) -> String {
    return digest(input);
}
