use ark_ff::{fields::*, MontFp, QuadExt};

use crate::*;

pub type Fq2 = Fp2<Fq2Config>;

pub struct Fq2Config;

impl Fp2Config for Fq2Config {
    type Fp = Fq;

    /// NONRESIDUE = -1
    const NONRESIDUE: Fq = MontFp!(Fq, "-1");

    /// QUADRATIC_NONRESIDUE = U+2
    const QUADRATIC_NONRESIDUE: Fq2 = QuadExt!(MontFp!(Fq, "2"), MontFp!(Fq, "1"));

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP2_C1: &'static [Fq] = &[
        // NONRESIDUE**(((q^0) - 1) / 2)
        MontFp!(Fq, "1"),
        // NONRESIDUE**(((q^1) - 1) / 2)
        MontFp!(Fq, "-1"),
    ];

    #[inline(always)]
    fn mul_fp_by_nonresidue(fe: &Self::Fp) -> Self::Fp {
        -(*fe)
    }
}

pub const FQ2_ZERO: Fq2 = QuadExt!(FQ_ZERO, FQ_ZERO);
pub const FQ2_ONE: Fq2 = QuadExt!(FQ_ONE, FQ_ZERO);
