use ct_fp::PrimeField;
use ct_uint::Uint;
use subtle::Choice;

pub trait ScalarField<const LIMBS: usize>: PrimeField<LIMBS> {
    #[inline]
    fn is_valid_scalar(&self) -> Choice {
        let nonzero = !self.ct_is_zero();
        let lt_n = self.to_uint().ct_lt(&Self::MODULUS);
        nonzero & lt_n
    }

    fn from_bytes_mod_order(bytes: &[u8]) -> Self {
        let mut acc = Self::zero();
        let radix = Self::from_uint(Uint::from_u64(256));
        let mut i = 0;
        while i < bytes.len() {
            acc = acc.mul(&radix);
            acc = acc.add(&Self::from_uint(Uint::from_u64(bytes[i] as u64)));
            i += 1;
        }
        acc
    }

    fn from_bytes_wide(bytes: &[u8]) -> Self {
        assert_eq!(
            bytes.len(),
            2 * LIMBS * 8,
            "from_bytes_wide requires exactly 2N bytes"
        );
        Self::from_bytes_mod_order(bytes)
    }
}
