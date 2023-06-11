use crate::ru256::RU256;
use std::str::FromStr;

/// Represents a point on an elliptic curve
#[derive(PartialEq, Clone)]
pub(crate) struct Point {
    x: RU256,
    y: RU256,
}

impl Point {
    /// Build a point from hex strings
    fn from_hex_coordinates(x: &str, y: &str) -> Self {
        return Point {
            x: RU256::from_str(x).unwrap(),
            y: RU256::from_str(y).unwrap(),
        };
    }

    /// Return the uncompressed version of a point
    fn to_hex_string(&self) -> String {
        return format!("04{}{}", self.x.to_string(), self.y.to_string());
    }

    /// Determines if a point is the identity element
    fn is_zero_point(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

pub struct SECP256K1;

impl SECP256K1 {
    // Curve parameter specification
    // see: https://www.secg.org/sec2-v2.pdf

    /// Prime value
    /// 2^256 - 2^23 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1
    fn p() -> RU256 {
        RU256::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F").unwrap()
    }

    /// Generator point
    fn g() -> Point {
        Point {
            x: RU256::from_str("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798")
                .unwrap(),
            y: RU256::from_str("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8")
                .unwrap(),
        }
    }

    /// Group order
    fn n() -> RU256 {
        RU256::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141").unwrap()
    }

    /// Zero point
    fn zero_point() -> Point {
        Point {
            x: RU256::zero(),
            y: RU256::zero(),
        }
    }

    /// Add two different curve points
    fn add_points(p1: &Point, p2: &Point) -> Point {
        // two points P = (xp, yp) and Q = (xq, yq)
        // lambda = (yq - yp) / (xq - xp)
        // x3 = lambda^2 - xp - xq
        // y3 = lambda(xp - x3) - yp

        // we need to make sure the points are not the same,
        // if the same when calculating lambda, we will have
        // a division by zero error
        assert!(p1 != p2);

        // if any of the point is the identity, we return the
        // other point
        // as P + O = P
        if p1.is_zero_point() {
            return p2.clone();
        }
        if p2.is_zero_point() {
            return p1.clone();
        }

        // get the field prime
        let p = Self::p();

        // calculate slope
        let y_diff = p1.y.sub_mod(&p2.y, &p);
        let x_diff = p1.x.sub_mod(&p2.x, &p);
        let lambda = y_diff.div_mod(&x_diff, &p);

        // calculate point values
        let x3 = &lambda
            .mul_mod(&lambda, &p)
            .sub_mod(&p1.x, &p)
            .sub_mod(&p2.x, &p);
        let y3 = &p1
            .x
            .sub_mod(&x3, &p)
            .mul_mod(&lambda, &p)
            .sub_mod(&p1.y, &p);

        Point {
            x: x3.clone(),
            y: y3.clone(),
        }
    }

    /// Double a curve point
    fn double_point(p1: &Point) -> Point {
        // only one point (x, y)
        // lambda = (3x^2 + a) / 2y
        // x3 = lambda^2 - x - x
        // y3 = lambda(xp - x) - y

        // doubling the identity point, returns the identity point
        // O + O = O
        if p1.is_zero_point() {
            return Self::zero_point();
        };

        // if only y is zero, we are at the non-symmetrical point
        // on the curve, drawing a tangent line from this point will
        // lead to infinity (hence we return the identity point)
        if p1.y.is_zero() {
            return Self::zero_point();
        };

        // get the field prime
        let p = Self::p();

        // formula includes constant 2 and 3
        // to simply formula description, we define
        // them as here first
        let const_2 = RU256::from_str("0x2").unwrap();
        let const_3 = RU256::from_str("0x3").unwrap();

        // calculate the slope
        // for the secp256k1 curve a = 0 so no need to include that in the formula description
        let three_x_square = &p1.x.mul_mod(&p1.x, &p).mul_mod(&const_3, &p);
        let two_y = &p1.y.mul_mod(&const_2, &p);
        let lambda = three_x_square.div_mod(two_y, &p);

        // calculate point values
        let x3 = &lambda
            .mul_mod(&lambda, &p)
            .sub_mod(&p1.x, &p)
            .sub_mod(&p1.x, &p);
        let y3 = &p1
            .x
            .sub_mod(&x3, &p)
            .mul_mod(&lambda, &p)
            .sub_mod(&p1.y, &p);

        Point {
            x: x3.clone(),
            y: y3.clone(),
        }
    }

    // TODO: make more efficient
    /// Derive the public key from a given private key
    fn public_key(private_key: &RU256) -> Point {
        // We generate the public key by doing a scalar
        // multiplication of the generator point by the
        // private key
        // i.e public_key = nG where n = private_key
        // this can be seen as repeated addition, so the
        // double-add algorithm will be useful here
        // see: mul_mod function in ru256.rs

        // the result starts at the identity
        let mut result = Self::zero_point();
        // base point is the generator point
        let mut adder = Self::g();

        let seq_bit_size = private_key.v.bits();
        for i in 0..seq_bit_size {
            if private_key.v.bit(i) {
                // TODO: is there a proof that result is never equal to adder
                //  with this setup?
                result = Self::add_points(&result, &adder);
            }
            // here we always double
            adder = Self::double_point(&adder);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secp256k1_add_poins() {
        let pt1 = Point::from_hex_coordinates(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
        );
        let pt2 = Point::from_hex_coordinates(
            "C6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE5",
            "1AE168FEA63DC339A3C58419466CEAEEF7F632653266D0E1236431A950CFE52A",
        );
        let pt3 = SECP256K1::add_points(&pt1, &pt2);

        assert_eq!(pt3.to_hex_string(), "04f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9388f7b0f632de8140fe337e62a37f3566500a99934c2231b6cb9fd7584b8e672");
    }

    #[test]
    fn secp256k1_double_point() {
        let pt1 = Point::from_hex_coordinates(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
        );

        let pt2 = SECP256K1::double_point(&pt1);
        let pt3 = SECP256K1::double_point(&pt2);

        assert_eq!(pt3.to_hex_string(), "04e493dbf1c10d80f3581e4904930b1404cc6c13900ee0758474fa94abe8c4cd1351ed993ea0d455b75642e2098ea51448d967ae33bfbdfe40cfe97bdc47739922");
    }
}
