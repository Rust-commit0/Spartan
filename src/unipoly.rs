use super::commitments::{Commitments, MultiCommitGens};
use super::group::GroupElement;
use super::scalar::{Scalar, ScalarFromPrimitives};
use super::transcript::{AppendToTranscript, ProofTranscript};
use merlin::Transcript;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct UniPoly {
    coeffs: Vec<Scalar>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CompressedUniPoly {
    coeffs_except_linear_term: Vec<Scalar>,
}
impl UniPoly {
    pub fn from_evals(evals: &[Scalar]) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn degree(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub fn as_vec(&self) -> Vec<Scalar> {
        panic!("STUB: not implemented");
    }
    pub fn eval_at_zero(&self) -> Scalar {
        panic!("STUB: not implemented");
    }
    pub fn eval_at_one(&self) -> Scalar {
        panic!("STUB: not implemented");
    }
    pub fn evaluate(&self, r: &Scalar) -> Scalar {
        panic!("STUB: not implemented");
    }
    pub fn compress(&self) -> CompressedUniPoly {
        panic!("STUB: not implemented");
    }
    pub fn commit(&self, gens: &MultiCommitGens, blind: &Scalar) -> GroupElement {
        panic!("STUB: not implemented");
    }
}
impl CompressedUniPoly {
    pub fn decompress(&self, hint: &Scalar) -> UniPoly {
        panic!("STUB: not implemented");
    }
}
impl AppendToTranscript for UniPoly {
    fn append_to_transcript(&self, label: &'static [u8], transcript: &mut Transcript) {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_evals_quad() {
        let e0 = Scalar::one();
        let e1 = (6_usize).to_scalar();
        let e2 = (15_usize).to_scalar();
        let evals = vec![e0, e1, e2];
        let poly = UniPoly::from_evals(&evals);
        assert_eq!(poly.eval_at_zero(), e0);
        assert_eq!(poly.eval_at_one(), e1);
        assert_eq!(poly.coeffs.len(), 3);
        assert_eq!(poly.coeffs[0], Scalar::one());
        assert_eq!(poly.coeffs[1], (3_usize).to_scalar());
        assert_eq!(poly.coeffs[2], (2_usize).to_scalar());
        let hint = e0 + e1;
        let compressed_poly = poly.compress();
        let decompressed_poly = compressed_poly.decompress(&hint);
        for i in 0..decompressed_poly.coeffs.len() {
            assert_eq!(decompressed_poly.coeffs[i], poly.coeffs[i]);
        }
        let e3 = (28_usize).to_scalar();
        assert_eq!(poly.evaluate(& (3_usize).to_scalar()), e3);
    }
    #[test]
    fn test_from_evals_cubic() {
        let e0 = Scalar::one();
        let e1 = (7_usize).to_scalar();
        let e2 = (23_usize).to_scalar();
        let e3 = (55_usize).to_scalar();
        let evals = vec![e0, e1, e2, e3];
        let poly = UniPoly::from_evals(&evals);
        assert_eq!(poly.eval_at_zero(), e0);
        assert_eq!(poly.eval_at_one(), e1);
        assert_eq!(poly.coeffs.len(), 4);
        assert_eq!(poly.coeffs[0], Scalar::one());
        assert_eq!(poly.coeffs[1], (3_usize).to_scalar());
        assert_eq!(poly.coeffs[2], (2_usize).to_scalar());
        assert_eq!(poly.coeffs[3], (1_usize).to_scalar());
        let hint = e0 + e1;
        let compressed_poly = poly.compress();
        let decompressed_poly = compressed_poly.decompress(&hint);
        for i in 0..decompressed_poly.coeffs.len() {
            assert_eq!(decompressed_poly.coeffs[i], poly.coeffs[i]);
        }
        let e4 = (109_usize).to_scalar();
        assert_eq!(poly.evaluate(& (4_usize).to_scalar()), e4);
    }
}
