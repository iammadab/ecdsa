use crate::util::{modular_multiplicative_inverse, modulo};
use num_bigint::BigInt;
use num_traits::identities::{One, Zero};

/// Represents an elliptic curve point
#[derive(Clone, Debug)]
struct Point {
    x: BigInt,
    y: BigInt,
    /// fp - prime field, every operation will be done modulo this number
    fp: BigInt,
}

impl Point {
    /// Represents the identity point
    fn identity(base_point: &Point) -> Point {
        Point {
            x: BigInt::zero(),
            y: BigInt::zero(),
            fp: base_point.fp.clone(),
        }
    }

    /// Multiply a curve point by some scalar n
    /// this uses the double-and-add algorithm
    fn multiply(&self, n: BigInt) -> Point {
        // we start with the identity, as the result
        let mut result = Point::identity(&self);
        let mut curr_n = n;
        let mut curr_point = self.clone(); // TODO: need to clone?

        while curr_n > BigInt::zero() {
            // if the LSB is set add current point to the result
            // n will be odd is LSB is set
            if &curr_n % 2 != BigInt::zero() {
                result = result.add(&curr_point)
            }

            // double the current point
            curr_point = curr_point.double();

            // perform a right shift (chops off the LSB)
            curr_n = &curr_n >> 1;
        }

        return result;
    }

    /// Adds two curve points
    fn add(&self, other: &Point) -> Point {
        if self.x == other.x && self.y == (&other.y * -1) {
            // if other is inverse of self, return identity
            // other is inverse if x is the same, but y is negated
            Point::identity(&self)
        } else if self.x == other.x && self.y == other.y {
            // the points are the same so we do a doubling
            self.double()
        } else if self.x == BigInt::zero() && self.y == BigInt::zero() {
            // P is the identity element, P = 0
            // hence P + Q = 0 + Q = Q
            other.clone()
        } else if other.x == BigInt::zero() && other.y == BigInt::zero() {
            // Q is the identity element, Q = 0
            // hence P + Q = P + 0 = P
            self.clone()
        } else {
            // P and Q are different points on the curve
            let lambda = modulo(
                &((&other.y - &self.y)
                    * modular_multiplicative_inverse(&other.x - &self.x, 1 * &self.fp)),
                &self.fp,
            );
            let rx = modulo(&(BigInt::pow(&lambda, 2) - &self.x - &other.x), &self.fp);
            let ry = modulo(&(lambda * (&self.x - &rx) - &self.y), &self.fp);

            Point {
                x: rx,
                y: ry,
                fp: self.fp.clone(),
            }
        }
    }

    /// Adds a point to itself
    fn double(&self) -> Point {
        // TODO: point should generalize to any curve
        // for the secp256k1 curve a = 0, so we can exclude it from the eq
        let lambda = modulo(
            &(3 * BigInt::pow(&self.x, 2)
                * modular_multiplicative_inverse(2 * &self.y, 1 * &self.fp)),
            &self.fp,
        );
        let rx = modulo(&(BigInt::pow(&lambda, 2) - &self.x - &self.x), &self.fp);
        let ry = modulo(&(lambda * (&self.x - &rx) - &self.y), &self.fp);

        Point {
            x: rx,
            y: ry,
            fp: self.fp.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn pub_key_generation() {
        let bigint = |num: &str| -> BigInt { BigInt::parse_bytes(num.as_bytes(), 16).unwrap() };
        let generator = Point {
            x: bigint("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798"),
            y: bigint("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8"),
            fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"),
        };
        // pick a private key and do the multiplication
        let private_key =
            bigint("50c57c4965fca50b57fe1a781f6ebb31596d09c27b84f03f00485a203878b8f8");
        let public_key = generator.multiply(private_key);
        dbg!(&public_key);
        dbg!(format!("{:x}", public_key.x));
        dbg!(format!("{:x}", public_key.y));
        assert_eq!(
            public_key.x,
            bigint("8fb5228fbd4f03793b30a37370f4536468ff0bd7a834e612d032c11db2453a05")
        );
        assert_eq!(
            public_key.y,
            bigint("91e931ac2c3a42b28e41b599b8ffa54fe4fb50bbb2460b1ff66944f97788dcee")
        );
    }
}
