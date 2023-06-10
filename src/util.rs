use num_bigint::BigInt;
use num_traits::identities::{One, Zero};

/// Finds the multiplicative inverse
pub(crate) fn modular_multiplicative_inverse(n: BigInt, order: BigInt) -> BigInt {
    let mut ab = vec![n, order.clone()];

    let mut ta: [BigInt; 2] = [BigInt::zero(), BigInt::one()];

    while ab[0] != BigInt::zero() {
        let q = &ab[1] / &ab[0];
        let rem = &ab[1] % &ab[0];
        ab[1] = ab[0].clone();
        ab[0] = rem;

        let t1 = &ta[0] - &q * &ta[1];

        ta[0] = ta[1].clone();
        ta[1] = t1;
    }

    modulo(&ta[0], &order)
}

pub(crate) fn modulo(n: &BigInt, order: &BigInt) -> BigInt {
    let rem = n % order;
    return if rem < BigInt::zero() {
        rem + order
    } else {
        rem
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_inverse() {
        assert_eq!(
            modular_multiplicative_inverse(3.into(), 26.into()),
            9.into()
        );
        assert_eq!(
            modular_multiplicative_inverse(345.into(), 76408.into()),
            48281.into()
        );
    }
}
