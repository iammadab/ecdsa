use crate::util::{modular_multiplicative_inverse, modulo};
use num_bigint::BigInt;
use num_traits::identities::{One, Zero};

/// Represents an elliptic curve point
#[derive(Clone, Debug, PartialEq)]
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
            dbg!("count");
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

        dbg!(&result);
        print_point(&result);
        return result;
    }

    /// Adds two curve points
    fn add(&self, other: &Point) -> Point {
        if self.x == other.x && self.y == (&other.y * -1) {
            dbg!("iden");
            // if other is inverse of self, return identity
            // other is inverse if x is the same, but y is negated
            Point::identity(&self)
        } else if self.x == other.x && self.y == other.y {
            dbg!("double");
            // the points are the same so we do a doubling
            self.double()
        } else if self.x == BigInt::zero() && self.y == BigInt::zero() {
            dbg!("other");
            // dbg!(&other);
            print_point(&other);
            // P is the identity element, P = 0
            // hence P + Q = 0 + Q = Q
            other.clone()
        } else if other.x == BigInt::zero() && other.y == BigInt::zero() {
            dbg!("self");
            // Q is the identity element, Q = 0
            // hence P + Q = P + 0 = P
            self.clone()
        } else {
            dbg!("diff");
            // dbg!(&self);
            // print_point(&self);
            // dbg!(&other);
            // print_point(&other);
            // P and Q are different points on the curve
            // dbg!(&other.y - &self.y);
            // dbg!(&other.x - &self.x);
            // dbg!(modular_multiplicative_inverse(&other.x - &self.x, self.fp.clone()));
            let lambda = modulo(
                &((&other.y - &self.y)
                    * modular_multiplicative_inverse(&other.x - &self.x, 1 * &self.fp)),
                &self.fp,
            );
            // dbg!(&lambda);

            let rx = modulo(&(BigInt::pow(&lambda, 2) - &self.x - &other.x), &self.fp);
            let ry = modulo(&(&lambda * (&self.x - &rx) - &self.y), &self.fp);

            let p = Point {
                x: rx,
                y: ry,
                fp: self.fp.clone(),
            };
            // dbg!(&p);
            p
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

fn print_point(p: &Point) {
    println!("{:x}", &p.x);
    println!("{:x}", &p.y);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use std::str::FromStr;

    #[test]
    fn pub_key_generation() {
        let bigint = |num: &str| -> BigInt { BigInt::parse_bytes(num.as_bytes(), 16).unwrap() };
        let generator = Point {
            x: bigint("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798"),
            y: bigint("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8"),
            fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"),
        };

        // let private_key = BigInt::from(1);
        // assert_eq!(
        //     generator.multiply(private_key),
        //     Point {
        //         x: bigint("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798"),
        //         y: bigint("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8"),
        //         fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
        //     }
        // );
        //
        // let private_key = BigInt::from(2);
        // assert_eq!(
        //     generator.multiply(private_key),
        //     Point {
        //         x: bigint("C6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE5"),
        //         y: bigint("1AE168FEA63DC339A3C58419466CEAEEF7F632653266D0E1236431A950CFE52A"),
        //         fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
        //     }
        // );
        //
        // dbg!("THREE");
        // let private_key = BigInt::from(3);
        // assert_eq!(
        //     generator.multiply(private_key),
        //     Point {
        //         x: bigint("F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9"),
        //         y: bigint("388F7B0F632DE8140FE337E62A37F3566500A99934C2231B6CB9FD7584B8E672"),
        //         fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
        //     }
        // );
        //
        // let private_key = BigInt::from(4);
        // assert_eq!(
        //     generator.multiply(private_key),
        //     Point {
        //         x: bigint("E493DBF1C10D80F3581E4904930B1404CC6C13900EE0758474FA94ABE8C4CD13"),
        //         y: bigint("51ED993EA0D455B75642E2098EA51448D967AE33BFBDFE40CFE97BDC47739922"),
        //         fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
        //     }
        // );

        dbg!("FIVE");
        let private_key = BigInt::from(5);
        assert_eq!(
            generator.multiply(private_key),
            Point {
                x: bigint("2F8BDE4D1A07209355B4A7250A5C5128E88B84BDDC619AB7CBA8D569B240EFE4"),
                y: bigint("D8AC222636E5E3D6D4DBA9DDA6C9C426F788271BAB0D6840DCA87D3AA6AC62D6"),
                fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
            }
        );

        let private_key = BigInt::from(20);
        assert_eq!(
            generator.multiply(private_key),
            Point {
                x: bigint("4CE119C96E2FA357200B559B2F7DD5A5F02D5290AFF74B03F3E471B273211C97"),
                y: bigint("12BA26DCB10EC1625DA61FA10A844C676162948271D96967450288EE9233DC3A"),
                fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
            }
        );

        let private_key = BigInt::from(112233445566778899_u64);
        assert_eq!(
            generator.multiply(private_key),
            Point {
                x: bigint("A90CC3D3F3E146DAADFC74CA1372207CB4B725AE708CEF713A98EDD73D99EF29"),
                y: bigint("5A79D6B289610C68BC3B47F3D72F9788A26A06868B4D8E433E1E2AD76FB7DC76"),
                fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
            }
        );

        let private_key = BigInt::from_str("115792089237316195423570985008687907852837564279074904382605163141518161494335").unwrap();
        dbg!(&private_key);
        assert_eq!(
            generator.multiply(private_key),
            Point {
                x: bigint("C6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE5"),
                y: bigint("E51E970159C23CC65C3A7BE6B99315110809CD9ACD992F1EDC9BCE55AF301705"),
                fp: bigint("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
            }
        );
        // pick a private key and do the multiplication
        // let private_key =
        //     bigint("50c57c4965fca50b57fe1a781f6ebb31596d09c27b84f03f00485a203878b8f8");
        // let public_key = generator.multiply(private_key);
        // dbg!(&public_key);
        // dbg!(format!("{:x}", public_key.x));
        // dbg!(format!("{:x}", public_key.y));
        // assert_eq!(
        //     public_key.x,
        //     bigint("8fb5228fbd4f03793b30a37370f4536468ff0bd7a834e612d032c11db2453a05")
        // );
        // assert_eq!(
        //     public_key.y,
        //     bigint("91e931ac2c3a42b28e41b599b8ffa54fe4fb50bbb2460b1ff66944f97788dcee")
        // );
    }
}
