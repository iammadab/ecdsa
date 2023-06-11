use crate::ru256::RU256;
use sha256::digest;
// I take a series of bytes, and calculate the hash of that

/// Generate the sha256 hash of a string
fn hash_string(message: &str) -> String {
    digest(message)
}

fn sign_message(message: &str, private_key: &RU256) {
    let hash = hash_string(message);
    dbg!(hash);
}

#[cfg(test)]
mod tests {
    use crate::ecdsa::sign_message;

    #[test]
    fn sign() {
        todo!()
        // sign_message("hello");
    }
}
