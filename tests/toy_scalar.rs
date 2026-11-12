use ct_fp::PrimeField;
use ct_scalar::ScalarField;
use ct_uint::Uint;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

/// Toy scalar field: n = 2^64 - 59 (prime).
#[derive(Clone, Copy, Debug)]
struct ToyScalar(u64);

impl ConstantTimeEq for ToyScalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl ConditionallySelectable for ToyScalar {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        ToyScalar(u64::conditional_select(&a.0, &b.0, choice))
    }
}

const TOY_N: u64 = 0xFFFF_FFFF_FFFF_FFC5; // 2^64 - 59

impl PrimeField<1> for ToyScalar {
    const MODULUS: Uint<1> = Uint::from_u64(TOY_N);

    fn mul(&self, rhs: &Self) -> Self {
        // Test-only: `%` is not constant-time; correctness is what we check.
        let r = ((self.0 as u128) * (rhs.0 as u128)) % (TOY_N as u128);
        ToyScalar(r as u64)
    }

    fn from_uint(v: Uint<1>) -> Self {
        ToyScalar(v.to_u64().unwrap())
    }

    fn to_uint(&self) -> Uint<1> {
        Uint::from_u64(self.0)
    }
}

// Use all default ScalarField methods.
impl ScalarField<1> for ToyScalar {}

#[test]
fn scalar_range_check() {
    assert!(!bool::from(ToyScalar(0).is_valid_scalar()));
    assert!(bool::from(ToyScalar(1).is_valid_scalar()));
    assert!(bool::from(ToyScalar(TOY_N - 1).is_valid_scalar()));
    assert!(!bool::from(ToyScalar(TOY_N).is_valid_scalar()));
    assert!(!bool::from(ToyScalar(u64::MAX).is_valid_scalar()));
}

#[test]
fn from_bytes_mod_order_small() {
    // big-endian [0x30, 0x39] = 12345
    let s = ToyScalar::from_bytes_mod_order(&[0x30, 0x39]);
    assert_eq!(s.0, 12345);
}

#[test]
fn from_bytes_mod_order_overflow() {
    // 0x01 00 00 00 00 00 00 00 00 = 2^64, which is > n
    let bytes = [0x01, 0, 0, 0, 0, 0, 0, 0, 0];
    let s = ToyScalar::from_bytes_mod_order(&bytes);
    // 2^64 mod (2^64 - 59) = 59
    assert_eq!(s.0, 59);
}

#[test]
fn from_bytes_wide_16_bytes() {
    // LIMBS = 1, so from_bytes_wide requires 2 * 1 * 8 = 16 bytes.
    let bytes = [0xFFu8; 16];
    let s = ToyScalar::from_bytes_wide(&bytes);

    let mut expected: u128 = 0;
    for _ in 0..16 {
        expected = (expected << 8) | 0xFF;
    }
    expected %= TOY_N as u128;
    assert_eq!(s.0, expected as u64);
}

#[test]
#[should_panic(expected = "from_bytes_wide requires exactly 2N bytes")]
fn from_bytes_wide_wrong_length() {
    let _ = ToyScalar::from_bytes_wide(&[0u8; 8]);
}
