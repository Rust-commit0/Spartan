#![allow(clippy::too_many_arguments)]
use super::commitments::{Commitments, MultiCommitGens};
use super::errors::ProofVerifyError;
use super::group::{CompressedGroup, GroupElement, VartimeMultiscalarMul};
use super::math::Math;
use super::nizk::{DotProductProofGens, DotProductProofLog};
use super::random::RandomTape;
use super::scalar::Scalar;
use super::transcript::{AppendToTranscript, ProofTranscript};
use core::ops::Index;
use merlin::Transcript;
use serde::{Deserialize, Serialize};
#[cfg(feature = "multicore")]
use rayon::prelude::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct DensePolynomial {
    num_vars: usize,
    len: usize,
    Z: Vec<Scalar>,
}
#[derive(Serialize, Deserialize)]
pub struct PolyCommitmentGens {
    pub gens: DotProductProofGens,
}
impl PolyCommitmentGens {
    pub fn new(num_vars: usize, label: &'static [u8]) -> PolyCommitmentGens {
        panic!("STUB: not implemented");
    }
}
pub struct PolyCommitmentBlinds {
    blinds: Vec<Scalar>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PolyCommitment {
    C: Vec<CompressedGroup>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstPolyCommitment {
    C: CompressedGroup,
}
pub struct EqPolynomial {
    r: Vec<Scalar>,
}
impl EqPolynomial {
    pub fn new(r: Vec<Scalar>) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn evaluate(&self, rx: &[Scalar]) -> Scalar {
        panic!("STUB: not implemented");
    }
    pub fn evals(&self) -> Vec<Scalar> {
        panic!("STUB: not implemented");
    }
    pub fn compute_factored_lens(ell: usize) -> (usize, usize) {
        panic!("STUB: not implemented");
    }
    pub fn compute_factored_evals(&self) -> (Vec<Scalar>, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
}
pub struct IdentityPolynomial {
    size_point: usize,
}
impl IdentityPolynomial {
    pub fn new(size_point: usize) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn evaluate(&self, r: &[Scalar]) -> Scalar {
        panic!("STUB: not implemented");
    }
}
impl DensePolynomial {
    pub fn new(Z: Vec<Scalar>) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn get_num_vars(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub fn clone(&self) -> DensePolynomial {
        panic!("STUB: not implemented");
    }
    pub fn split(&self, idx: usize) -> (DensePolynomial, DensePolynomial) {
        panic!("STUB: not implemented");
    }
    #[cfg(feature = "multicore")]
    fn commit_inner(&self, blinds: &[Scalar], gens: &MultiCommitGens) -> PolyCommitment {
        panic!("STUB: not implemented");
    }
    #[cfg(not(feature = "multicore"))]
    fn commit_inner(&self, blinds: &[Scalar], gens: &MultiCommitGens) -> PolyCommitment {
        panic!("STUB: not implemented");
    }
    pub fn commit(
        &self,
        gens: &PolyCommitmentGens,
        random_tape: Option<&mut RandomTape>,
    ) -> (PolyCommitment, PolyCommitmentBlinds) {
        panic!("STUB: not implemented");
    }
    pub fn bound(&self, L: &[Scalar]) -> Vec<Scalar> {
        panic!("STUB: not implemented");
    }
    pub fn bound_poly_var_top(&mut self, r: &Scalar) {
        panic!("STUB: not implemented");
    }
    pub fn bound_poly_var_bot(&mut self, r: &Scalar) {
        panic!("STUB: not implemented");
    }
    pub fn evaluate(&self, r: &[Scalar]) -> Scalar {
        panic!("STUB: not implemented");
    }
    fn vec(&self) -> &Vec<Scalar> {
        panic!("STUB: not implemented");
    }
    pub fn extend(&mut self, other: &DensePolynomial) {
        panic!("STUB: not implemented");
    }
    pub fn merge<'a, I>(polys: I) -> DensePolynomial
    where
        I: IntoIterator<Item = &'a DensePolynomial>,
    {
        panic!("STUB: not implemented");
    }
    pub fn from_usize(Z: &[usize]) -> Self {
        panic!("STUB: not implemented");
    }
}
impl Index<usize> for DensePolynomial {
    type Output = Scalar;
    #[inline(always)]
    fn index(&self, _index: usize) -> &Scalar {
        panic!("STUB: not implemented");
    }
}
impl AppendToTranscript for PolyCommitment {
    fn append_to_transcript(&self, label: &'static [u8], transcript: &mut Transcript) {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PolyEvalProof {
    proof: DotProductProofLog,
}
impl PolyEvalProof {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        poly: &DensePolynomial,
        blinds_opt: Option<&PolyCommitmentBlinds>,
        r: &[Scalar],
        Zr: &Scalar,
        blind_Zr_opt: Option<&Scalar>,
        gens: &PolyCommitmentGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> (PolyEvalProof, CompressedGroup) {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        gens: &PolyCommitmentGens,
        transcript: &mut Transcript,
        r: &[Scalar],
        C_Zr: &CompressedGroup,
        comm: &PolyCommitment,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
    pub fn verify_plain(
        &self,
        gens: &PolyCommitmentGens,
        transcript: &mut Transcript,
        r: &[Scalar],
        Zr: &Scalar,
        comm: &PolyCommitment,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::super::scalar::ScalarFromPrimitives;
    use super::*;
    use rand::rngs::OsRng;
    fn evaluate_with_LR(Z: &[Scalar], r: &[Scalar]) -> Scalar {
        let eq = EqPolynomial::new(r.to_vec());
        let (L, R) = eq.compute_factored_evals();
        let ell = r.len();
        assert!(ell % 2 == 0);
        let n = ell.pow2();
        let m = (n as f64).sqrt() as usize;
        let LZ = (0..m)
            .map(|i| (0..m).map(|j| L[j] * Z[j * m + i]).sum())
            .collect::<Vec<Scalar>>();
        DotProductProofLog::compute_dotproduct(&LZ, &R)
    }
    #[test]
    fn check_polynomial_evaluation() {
        let Z = vec![
            Scalar::one(), (2_usize).to_scalar(), (1_usize).to_scalar(), (4_usize)
            .to_scalar(),
        ];
        let r = vec![(4_usize).to_scalar(), (3_usize).to_scalar()];
        let eval_with_LR = evaluate_with_LR(&Z, &r);
        let poly = DensePolynomial::new(Z);
        let eval = poly.evaluate(&r);
        assert_eq!(eval, (28_usize).to_scalar());
        assert_eq!(eval_with_LR, eval);
    }
    pub fn compute_factored_chis_at_r(r: &[Scalar]) -> (Vec<Scalar>, Vec<Scalar>) {
        let mut L: Vec<Scalar> = Vec::new();
        let mut R: Vec<Scalar> = Vec::new();
        let ell = r.len();
        assert!(ell % 2 == 0);
        let n = ell.pow2();
        let m = (n as f64).sqrt() as usize;
        for i in 0..m {
            let mut chi_i = Scalar::one();
            for j in 0..ell / 2 {
                let bit_j = ((m * i) & (1 << (r.len() - j - 1))) > 0;
                if bit_j {
                    chi_i *= r[j];
                } else {
                    chi_i *= Scalar::one() - r[j];
                }
            }
            L.push(chi_i);
        }
        for i in 0..m {
            let mut chi_i = Scalar::one();
            for j in ell / 2..ell {
                let bit_j = (i & (1 << (r.len() - j - 1))) > 0;
                if bit_j {
                    chi_i *= r[j];
                } else {
                    chi_i *= Scalar::one() - r[j];
                }
            }
            R.push(chi_i);
        }
        (L, R)
    }
    pub fn compute_chis_at_r(r: &[Scalar]) -> Vec<Scalar> {
        let ell = r.len();
        let n = ell.pow2();
        let mut chis: Vec<Scalar> = Vec::new();
        for i in 0..n {
            let mut chi_i = Scalar::one();
            for j in 0..r.len() {
                let bit_j = (i & (1 << (r.len() - j - 1))) > 0;
                if bit_j {
                    chi_i *= r[j];
                } else {
                    chi_i *= Scalar::one() - r[j];
                }
            }
            chis.push(chi_i);
        }
        chis
    }
    pub fn compute_outerproduct(L: Vec<Scalar>, R: Vec<Scalar>) -> Vec<Scalar> {
        assert_eq!(L.len(), R.len());
        (0..L.len())
            .map(|i| (0..R.len()).map(|j| L[i] * R[j]).collect::<Vec<Scalar>>())
            .collect::<Vec<Vec<Scalar>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<Scalar>>()
    }
    #[test]
    fn check_memoized_chis() {
        let mut csprng: OsRng = OsRng;
        let s = 10;
        let mut r: Vec<Scalar> = Vec::new();
        for _i in 0..s {
            r.push(Scalar::random(&mut csprng));
        }
        let chis = tests::compute_chis_at_r(&r);
        let chis_m = EqPolynomial::new(r).evals();
        assert_eq!(chis, chis_m);
    }
    #[test]
    fn check_factored_chis() {
        let mut csprng: OsRng = OsRng;
        let s = 10;
        let mut r: Vec<Scalar> = Vec::new();
        for _i in 0..s {
            r.push(Scalar::random(&mut csprng));
        }
        let chis = EqPolynomial::new(r.clone()).evals();
        let (L, R) = EqPolynomial::new(r).compute_factored_evals();
        let O = compute_outerproduct(L, R);
        assert_eq!(chis, O);
    }
    #[test]
    fn check_memoized_factored_chis() {
        let mut csprng: OsRng = OsRng;
        let s = 10;
        let mut r: Vec<Scalar> = Vec::new();
        for _i in 0..s {
            r.push(Scalar::random(&mut csprng));
        }
        let (L, R) = tests::compute_factored_chis_at_r(&r);
        let eq = EqPolynomial::new(r);
        let (L2, R2) = eq.compute_factored_evals();
        assert_eq!(L, L2);
        assert_eq!(R, R2);
    }
    #[test]
    fn check_polynomial_commit() {
        let Z = vec![
            (1_usize).to_scalar(), (2_usize).to_scalar(), (1_usize).to_scalar(),
            (4_usize).to_scalar(),
        ];
        let poly = DensePolynomial::new(Z);
        let r = vec![(4_usize).to_scalar(), (3_usize).to_scalar()];
        let eval = poly.evaluate(&r);
        assert_eq!(eval, (28_usize).to_scalar());
        let gens = PolyCommitmentGens::new(poly.get_num_vars(), b"test-two");
        let (poly_commitment, blinds) = poly.commit(&gens, None);
        let mut random_tape = RandomTape::new(b"proof");
        let mut prover_transcript = Transcript::new(b"example");
        let (proof, C_Zr) = PolyEvalProof::prove(
            &poly,
            Some(&blinds),
            &r,
            &eval,
            None,
            &gens,
            &mut prover_transcript,
            &mut random_tape,
        );
        let mut verifier_transcript = Transcript::new(b"example");
        assert!(
            proof.verify(& gens, & mut verifier_transcript, & r, & C_Zr, &
            poly_commitment).is_ok()
        );
    }
}
