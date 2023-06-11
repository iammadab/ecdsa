use hex;
use primitive_types::U256;
use std::str::FromStr;

#[derive(Clone)]
pub(crate) struct RU256 {
    pub(crate) v: U256,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RU256ParseError;

impl FromStr for RU256 {
    type Err = RU256ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // conversion from a hex string
        RU256::from_str_radix(s, 16)
    }
}

impl ToString for RU256 {
    fn to_string(&self) -> String {
        let mut bytes: [u8; 32] = [0; 32];
        self.v.to_big_endian(&mut bytes);
        hex::encode(bytes)
    }
}

impl PartialEq for RU256 {
    fn eq(&self, other: &Self) -> bool {
        return self.v == other.v;
    }
}

impl RU256 {
    /// RU256 from byte slice
    pub fn from_bytes(byte_slice: &[u8]) -> Self {
        // can't be more than 32 bytes
        assert!(byte_slice.len() <= 32);
        Self {
            v: U256::from_big_endian(byte_slice),
        }
    }

    /// RU256 from number string
    pub fn from_str_radix(s: &str, radix: u32) -> Result<Self, RU256ParseError> {
        match U256::from_str_radix(s, radix) {
            Ok(n) => Ok(Self { v: n }),
            Err(_) => Err(RU256ParseError),
        }
    }

    /// RU255 to bytes
    pub fn to_bytes(&self, bytes: &mut [u8]) {
        self.v.to_big_endian(bytes)
    }

    /// Additive Identity
    pub fn zero() -> Self {
        Self { v: U256::zero() }
    }

    /// Check if additive identity
    pub fn is_zero(&self) -> bool {
        self.v == U256::zero()
    }

    /// Multiplicative Identity
    pub fn one() -> Self {
        Self { v: U256::one() }
    }

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
            // we can performed checked operations because we can't get an overflow from
            // adding u to the result
            // worst case result is if the 2 values are (p-1 and p-1)
            // worst case sum is 2p - 2
            // which is less than double max, assuming p < max (which is the case)
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

    /// Modular subtraction
    /// A - B mod p = ((A mod p) - (B mod p)) mod p
    pub fn sub_mod(&self, b: &RU256, p: &RU256) -> Self {
        // A - B == A + (-B)
        // since mod p, we need the additive inverse of B
        // additive inverse is a number what when added gives the identity
        // identity in our case if p as p mod p = 0
        // so B inverse = p - b
        // hence A - B mod p == ((A mod p) + ((p - B) mod p)) mod p
        // this allows us to re-use add mod

        // modularize each input first
        let x1 = self.v.checked_rem(p.v).expect("mod");
        let x2 = b.v.checked_rem(p.v).expect("mod");

        let x2_complement = Self { v: p.v - x2 };
        let x3 = Self { v: x1 }.add_mod(&x2_complement, p);

        x3
    }

    /// Modular multiplication
    pub fn mul_mod(&self, b: &RU256, p: &RU256) -> Self {
        // multiplication can be thought of a repeated addition
        // were a * n = a + a + a .. + a n times
        // the above above algorithm is linear in n
        // we can do this in log(n) time using the
        // double-add algorithm: see: https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication#Double-and-add

        // modularize each input first
        let x1 = self.v.checked_rem(p.v).expect("mod");
        let x2 = b.v.checked_rem(p.v).expect("mod");

        // n * b = b * n
        // we can either repeat b n times or n b times
        // to reduce the number of operations we should repeat the smaller of the 2
        let (seq, adder) = match x1 < x2 {
            true => (x1, x2),
            _ => (x2, x1),
        };

        // set the result to the additive identity element
        let mut result = Self::zero();
        let mut adder = Self { v: adder };

        // Double-Add algorithm
        let seq_bit_size = seq.bits();
        for i in 0..seq_bit_size {
            if seq.bit(i) {
                // current bit is set, add to result
                result = result.add_mod(&adder, &p);
            }
            // double the adder
            adder = adder.add_mod(&adder, &p);
        }

        result
    }

    /// Modular exponentiation
    pub fn exp_mod(&self, e: &RU256, p: &RU256) -> Self {
        // exponentiation can be thought of as repeated multiplication
        // a^e = a * a * a * ... * a  e times (linear)
        // we can make it log(n) by using a variation of the double-add algorithm
        // called the square-multiply algorithm

        // set the result to the multiplicative identity element
        let mut result = Self::one();
        let mut multiplier = Self {
            v: self.v.checked_rem(p.v).expect("mod"),
        };

        // Square multiply algorithm
        let seq_bit_size = e.v.bits();
        for i in 0..seq_bit_size {
            if e.v.bit(i) {
                result = result.mul_mod(&multiplier, &p);
            }
            multiplier = multiplier.mul_mod(&multiplier, &p);
        }

        result
    }

    /// Modular division
    pub fn div_mod(&self, b: &RU256, p: &RU256) -> Self {
        // we can express the division problem as a multiplication problem
        // a / b mod p == a * b^-1 mod p
        // we can also express the multiplicative inverse as a posistive exponent
        // from Fermat's little theorem: https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
        // a^(p-1) == 1 mod p
        // hence b^-1 mod p is congruent with b^-1 * b^p-1 mod p
        // if we simplify we have b^p-2 mod p
        // hence a / b mod p = a * b^(p-2) mod p

        // p must be greater than 2
        assert!(p.v - 2 > U256::from_big_endian(&[0]));

        return self.mul_mod(&b.exp_mod(&RU256 { v: p.v - 2 }, &p), &p);
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
