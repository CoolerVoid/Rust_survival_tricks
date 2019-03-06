extern crate crypto_hash;
pub use lambda_crypt::crypto_hash::{Algorithm, hex_digest};

pub fn get_hash(input: &str) -> String {
 let hash = hex_digest(Algorithm::SHA512, input.as_bytes());
 hash
}
