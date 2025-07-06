use sha2::{Digest, Sha256};

pub fn hash_str(i: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(i);
    format!("{:x}", hasher.finalize())
}
