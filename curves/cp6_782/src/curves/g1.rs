use ark_ec::{
    models::{short_weierstrass::SWCurveConfig, CurveConfig},
    short_weierstrass::{Affine, Projective},
    AffineRepr, CurveGroup,
};
use ark_ff::MontFp;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::vec::*;

use crate::{Fq, Fr};

pub type G1Affine = Affine<Config>;
pub type G1Projective = Projective<Config>;

#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct G1Prepared(pub G1Affine);

impl From<G1Affine> for G1Prepared {
    fn from(other: G1Affine) -> Self {
        G1Prepared(other)
    }
}

impl From<G1Projective> for G1Prepared {
    fn from(q: G1Projective) -> Self {
        q.into_affine().into()
    }
}

impl<'a> From<&'a G1Affine> for G1Prepared {
    fn from(other: &'a G1Affine) -> Self {
        G1Prepared(*other)
    }
}

impl<'a> From<&'a G1Projective> for G1Prepared {
    fn from(q: &'a G1Projective) -> Self {
        q.into_affine().into()
    }
}

impl G1Prepared {
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl Default for G1Prepared {
    fn default() -> Self {
        G1Prepared(G1Affine::generator())
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

impl CurveConfig for Config {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR =
    /// 86482221941698704497288378992285180119495364068003923046442785886272123124361700722982503222189455144364945735564951561028
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x5657b9b57b942344,
        0x84f9a65f3bd54eaf,
        0x5ea4214e35cd127,
        0xe3cbcbc14ec1501d,
        0xf196cb845a3092ab,
        0x7e14627ad0e19017,
        0x217db4,
    ];

    /// COFACTOR^(-1) mod r =
    /// 163276846538158998893990986356139314746223949404500031940624325017036397274793417940375498603127780919653358641788
    const COFACTOR_INV: Fr = MontFp!("163276846538158998893990986356139314746223949404500031940624325017036397274793417940375498603127780919653358641788");
}

impl SWCurveConfig for Config {
    /// COEFF_A = 5
    const COEFF_A: Fq = MontFp!("5");

    /// COEFF_B = 17764315118651679038286329069295091506801468118146712649886336045535808055361274148466772191243305528312843236347777260247138934336850548243151534538734724191505953341403463040067571652261229308333392040104884438208594329793895206056414
    const COEFF_B: Fq = MontFp!("17764315118651679038286329069295091506801468118146712649886336045535808055361274148466772191243305528312843236347777260247138934336850548243151534538734724191505953341403463040067571652261229308333392040104884438208594329793895206056414");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: G1Affine = G1Affine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);
}

/// G1_GENERATOR_X =
/// 5511163824921585887915590525772884263960974614921003940645351443740084257508990841338974915037175497689287870585840954231884082785026301437744745393958283053278991955159266640440849940136976927372133743626748847559939620888818486853646
pub const G1_GENERATOR_X: Fq = MontFp!("5511163824921585887915590525772884263960974614921003940645351443740084257508990841338974915037175497689287870585840954231884082785026301437744745393958283053278991955159266640440849940136976927372133743626748847559939620888818486853646");

/// G1_GENERATOR_Y =
/// 7913123550914612057135582061699117755797758113868200992327595317370485234417808273674357776714522052694559358668442301647906991623400754234679697332299689255516547752391831738454121261248793568285885897998257357202903170202349380518443
pub const G1_GENERATOR_Y: Fq = MontFp!("7913123550914612057135582061699117755797758113868200992327595317370485234417808273674357776714522052694559358668442301647906991623400754234679697332299689255516547752391831738454121261248793568285885897998257357202903170202349380518443");