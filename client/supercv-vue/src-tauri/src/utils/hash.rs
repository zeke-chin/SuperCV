use twox_hash::xxh3::hash64;

pub fn hash_str(input: &str) -> String {
    format!("{:x}", hash64(input.as_bytes()))
}

pub fn hash_vec(input: &[u8]) -> String {
    format!("{:x}", hash64(input))
}
