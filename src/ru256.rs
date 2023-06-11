use hex;
use primitive_types::U256;
use std::str::FromStr;

struct RU256 {
    v: U256,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RU256ParseError;

impl FromStr for RU256 {
    type Err = RU256ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // conversion from a hex string
        match U256::from_str_radix(s, 16) {
            Ok(n) => Ok(Self { v: n }),
            Err(_) => Err(RU256ParseError),
        }
    }
}

impl ToString for RU256 {
    fn to_string(&self) -> String {
        let mut bytes: [u8; 32] = [0; 32];
        self.v.to_big_endian(&mut bytes);
        hex::encode(bytes)
    }
}

impl RU256 {
    /// Modular addition
    /// A + B mod p == ((A mod p) + (B mod p)) mod p
    /// also handle overflow results
    pub fn add_mod(&self, b: &RU256, p: &RU256) -> Self {
        // A + B mod p == ((A mod p) + (B mod p)) mod p
        // this forces the added inputs to be less than p
        // but potentially, they could still cause an overflow
        // if say they are really close p and p is really close
        // to U256::MAX
        // There is some distance U between P and MAX before wrapping occurs
        // during overflow addition, we only get the wrapped value, but we
        // lose U
        // Accurate Result = P + U + S where S is the wrapped value
        // we can drop the P since we are in modulo space
        // so accurate result = U + S
        // what we get = S
        // hence if overflow we need to add U back to the result
        // U = MAX - P

        // modularize each input first
        let x1 = self.v.checked_rem(p.v).expect("mod");
        let x2 = b.v.checked_rem(p.v).expect("mod");

        // add, allow for overflow
        let (mut x3, has_overflow) = x1.overflowing_add(x2);

        if has_overflow {
            x3 = x3
                .checked_add(
                    // parity's U256 is one less than actual, so we need to add 1 to get the accurate result
                    U256::MAX
                        .checked_sub(p.v)
                        .expect("sub")
                        .checked_add(U256::from_big_endian(&[1]))
                        .expect("actual u256"),
                )
                .expect("add");
        }

        x3 = x3.checked_rem(p.v).expect("mod");

        Self { v: x3 }
    }

    pub fn sub_mod(&self, b: &RU256, p: &RU256) -> Self {
        return RU256::from_str("0").unwrap();
    }
    pub fn mul_mod(&self, b: &RU256, p: &RU256) -> Self {
        return RU256::from_str("0").unwrap();
    }
    pub fn exp_mod(&self, b: &RU256, p: &RU256) -> Self {
        return RU256::from_str("0").unwrap();
    }
    pub fn div_mod(&self, b: &RU256, p: &RU256) -> Self {
        return RU256::from_str("0").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::ru256::RU256;
    use std::str::FromStr;

    #[test]
    fn ru256_addition_case_1() {
        let a = RU256::from_str("0xBD").unwrap();
        let b = RU256::from_str("0x2B").unwrap();
        let p = RU256::from_str("0xB").unwrap();

        let r = a.add_mod(&b, &p);

        assert_eq!(
            r.to_string(),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );
    }

    #[test]
    fn ru256_addition_case_2() {
        let a = RU256::from_str("0xa167f055ff75c").unwrap();
        let b = RU256::from_str("0xacc457752e4ed").unwrap();
        let p = RU256::from_str("0xf9cd").unwrap();

        let r = a.add_mod(&b, &p);

        assert_eq!(
            r.to_string(),
            "0000000000000000000000000000000000000000000000000000000000006bb0"
        );
    }

    #[test]
    fn ru256_addition_case_3() {
        let a = RU256::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E")
            .unwrap();
        let b = RU256::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E")
            .unwrap();
        let p = RU256::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
            .unwrap();

        let r = a.add_mod(&b, &p);

        assert_eq!(
            r.to_string(),
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2d"
        );
    }

    #[test]
    fn ru256_subtraction_case_1() {
        let a = RU256::from_str("0x1ce606").unwrap(); // a = 189389.unwrap();
        let b = RU256::from_str("0xacc12484").unwrap(); // b = 289833894.unwrap();
        let p = RU256::from_str("0xf3fa3").unwrap(); // p = 99933.unwrap();

        let r = a.sub_mod(&b, &p);

        assert_eq!(
            r.to_string(),
            "000000000000000000000000000000000000000000000000000000000009645b"
        );
    }

    #[test]
    fn ru256_subtraction_case_2() {
        let a = RU256::from_str("0xacc12484").unwrap(); // a = 289833894.unwrap();
        let b = RU256::from_str("0x1ce606").unwrap(); // b = 189389.unwrap();
        let p = RU256::from_str("0xf3fa3").unwrap(); // p = 99933.unwrap();

        let r = a.sub_mod(&b, &p);

        assert_eq!(
            r.to_string(),
            "000000000000000000000000000000000000000000000000000000000005db48"
        );
    }

    #[test]
    fn ru256_multiplication_case() {
        let a = RU256::from_str("0xa167f055ff75c").unwrap(); // a = 283948457393954.unwrap();
        let b = RU256::from_str("0xacc457752e4ed").unwrap(); // b = 303934849383754.unwrap();
        let p = RU256::from_str("0xf9cd").unwrap(); // p = 6394.unwrap();

        let r = a.mul_mod(&b, &p);

        assert_eq!(
            r.to_string(),
            "000000000000000000000000000000000000000000000000000000000000e116"
        );
    }

    #[test]
    fn ru256_exponentiation_case() {
        let a = RU256::from_str("0x1ce606").unwrap(); // a = 189389.unwrap();
        let b = RU256::from_str("0xacc12484").unwrap(); // b = 289833894.unwrap();
        let p = RU256::from_str("0xf3fa3").unwrap(); // p = 99933.unwrap();

        let r = a.exp_mod(&b, &p);

        assert_eq!(
            r.to_string(),
            "000000000000000000000000000000000000000000000000000000000002a0fd"
        );
    }

    #[test]
    fn ru256_division_case() {
        let a = RU256::from_str("0x1ce606").unwrap(); // a = 189389.unwrap();
        let b = RU256::from_str("0xacc12484").unwrap(); // b = 289833894.unwrap();
        let p = RU256::from_str("0xf3fa3").unwrap(); // p = 99933.unwrap();

        let r = a.div_mod(&b, &p);

        assert_eq!(
            r.to_string(),
            "0000000000000000000000000000000000000000000000000000000000061f57"
        );
    }
}
