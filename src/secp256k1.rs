use crate::ru256::RU256;
use std::str::FromStr;

/// Represents a point on an elliptic curve
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
    fn is_zero(&self) -> bool {
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

    /// Add two curve points
    fn add_points(p1: &Point, p2: &Point) -> Point {
        todo!()
    }

    /// Double a curve point
    fn double_point(p: &Point) -> Point {
        todo!()
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
