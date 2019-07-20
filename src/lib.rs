use wasm_bindgen::prelude::*;

include!(concat!(env!("OUT_DIR"), "/password_hash.rs"));

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
