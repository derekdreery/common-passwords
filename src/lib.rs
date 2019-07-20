use fst::Set;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

const FST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/password_hash.fst"));

lazy_static! {
    static ref PASSWORDS: Set = Set::from_bytes(FST.to_owned()).unwrap();
}

#[wasm_bindgen]
pub fn is_compromised(test_password: &str) -> bool {
    PASSWORDS.contains(test_password)
}

#[cfg(test)]
mod tests {
    use super::is_compromised;

    #[test]
    fn it_works() {
        assert!(is_compromised("elizabeth"));
        assert!(!is_compromised("veryVERYsecret"));
    }
}
