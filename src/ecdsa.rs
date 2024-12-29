use crate::ru256::RU256;
use crate::secp256k1::{Point, SECP256K1};
use rand::Rng;
use sha256::digest;
use std::str::FromStr;

/// Represents an ECDSA signature
pub struct Signature {
    s: RU256,
    r: RU256,
}

/// Generate the sha256 hash of a string
fn hash_string(message: &str) -> String {
    digest(message)
}

/// ECDSA algorithm for signing a message
fn sign_message(message: &str, private_key: &RU256) -> Signature {
    // Hash the message to sign
    let hash = RU256::from_str(&hash_string(message)).unwrap();

    // generate a random nonce
    let mut rng = rand::thread_rng();
    let nonce_bytes: [u8; 32] = rng.gen();
    let nonce_num = RU256::from_bytes(&nonce_bytes);

    // map the nonce scalar to a point on the SECP256k1 curve using
    // the generator as the base point
    let nonce_point = SECP256K1::scalar_multiplication(&nonce_num, &SECP256K1::g());

    // r is the x component of the point
    let r = &nonce_point.x;

    // grab the group order
    let n = SECP256K1::n();

    // compute s
    let s = &r
        .mul_mod(&private_key, &n)
        .add_mod(&hash, &n)
        .div_mod(&nonce_num, &n);

    Signature {
        r: r.clone(),
        s: s.clone(),
    }
}

/// ECDSA algorithm for verification of a signed message
fn verify_message(message: &str, pub_key: &Point, signature: &Signature) -> bool {
    // hash the message
    let hash = RU256::from_str(&hash_string(message)).unwrap();

    // grab the group order
    let n = SECP256K1::n();

    // TODO: add comment showing short proof on why this works
    let w = RU256::from_bytes(&[1]).div_mod(&signature.s, &n);
    let u1 = &hash.mul_mod(&w, &n);
    let u2 = &signature.r.mul_mod(&w, &n);
    let u1_point = SECP256K1::scalar_multiplication(&u1, &SECP256K1::g());
    let u2_point = SECP256K1::scalar_multiplication(&u2, &pub_key);

    let verification_point = SECP256K1::add_points(&u1_point, &u2_point);

    verification_point.x == signature.r
}

#[cfg(test)]
mod tests {
    use crate::ecdsa::{sign_message, verify_message};
    use crate::ru256::RU256;
    use crate::secp256k1::SECP256K1;
    use std::str::FromStr;

    #[test]
    fn ecdsa_signing_and_verification() {
        let private_key = &RU256::from_str("3424").unwrap();
        let public_key = SECP256K1::public_key(&private_key);

        let signature = sign_message("hello-world", &private_key);
        let verification_result = verify_message("hello-world", &public_key, &signature);
        assert_eq!(verification_result, true);

        // should not verify for the same public key but different message
        let verification_result = verify_message("different-message", &public_key, &signature);
        assert_eq!(verification_result, false);

        // should not verify if we use a different public key
        let private_key = &RU256::from_str("3425").unwrap();
        let public_key = SECP256K1::public_key(&private_key);

        let verification_result = verify_message("hello-world", &public_key, &signature);
        assert_eq!(verification_result, false);
    }
}
