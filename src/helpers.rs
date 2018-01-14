extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

pub type Index = i32;

pub fn prepare_string_to_hash(index: Index, previous_hash: &str, data: &str) -> String {
    format!("{}{}{}", index, previous_hash, data)
}

pub fn hash_from_string(value: String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(&value);
    return hasher.result_str();
}
