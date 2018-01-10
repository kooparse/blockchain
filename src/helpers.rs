extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

pub fn hash_from_string(value: String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(&value);
    return hasher.result_str();
}
