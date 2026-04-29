#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
use super::commitments::{Commitments, MultiCommitGens};
use super::dense_mlpoly::DensePolynomial;
use super::errors::ProofVerifyError;
use super::group::{CompressedGroup, GroupElement, VartimeMultiscalarMul};
use super::nizk::DotProductProof;
use super::random::RandomTape;
use super::scalar::Scalar;
use super::transcript::{AppendToTranscript, ProofTranscript};
use super::unipoly::{CompressedUniPoly, UniPoly};
use core::iter;
use itertools::izip;
use merlin::Transcript;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct SumcheckInstanceProof {
    compressed_polys: Vec<CompressedUniPoly>,
}
impl SumcheckInstanceProof {
    pub fn new(compressed_polys: Vec<CompressedUniPoly>) -> SumcheckInstanceProof {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        claim: Scalar,
        num_rounds: usize,
        degree_bound: usize,
        transcript: &mut Transcript,
    ) -> Result<(Scalar, Vec<Scalar>), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ZKSumcheckInstanceProof {
    comm_polys: Vec<CompressedGroup>,
    comm_evals: Vec<CompressedGroup>,
    proofs: Vec<DotProductProof>,
}
impl ZKSumcheckInstanceProof {
    pub fn new(
        comm_polys: Vec<CompressedGroup>,
        comm_evals: Vec<CompressedGroup>,
        proofs: Vec<DotProductProof>,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        comm_claim: &CompressedGroup,
        num_rounds: usize,
        degree_bound: usize,
        gens_1: &MultiCommitGens,
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
    ) -> Result<(CompressedGroup, Vec<Scalar>), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
impl SumcheckInstanceProof {
    pub fn prove_cubic<F>(
        claim: &Scalar,
        num_rounds: usize,
        poly_A: &mut DensePolynomial,
        poly_B: &mut DensePolynomial,
        poly_C: &mut DensePolynomial,
        comb_func: F,
        transcript: &mut Transcript,
    ) -> (Self, Vec<Scalar>, Vec<Scalar>)
    where
        F: Fn(&Scalar, &Scalar, &Scalar) -> Scalar,
    {
        panic!("STUB: not implemented");
    }
    pub fn prove_cubic_batched<F>(
        claim: &Scalar,
        num_rounds: usize,
        poly_vec_par: (
            &mut Vec<&mut DensePolynomial>,
            &mut Vec<&mut DensePolynomial>,
            &mut DensePolynomial,
        ),
        poly_vec_seq: (
            &mut Vec<&mut DensePolynomial>,
            &mut Vec<&mut DensePolynomial>,
            &mut Vec<&mut DensePolynomial>,
        ),
        coeffs: &[Scalar],
        comb_func: F,
        transcript: &mut Transcript,
    ) -> (
        Self,
        Vec<Scalar>,
        (Vec<Scalar>, Vec<Scalar>, Scalar),
        (Vec<Scalar>, Vec<Scalar>, Vec<Scalar>),
    )
    where
        F: Fn(&Scalar, &Scalar, &Scalar) -> Scalar,
    {
        panic!("STUB: not implemented");
    }
}
impl ZKSumcheckInstanceProof {
    pub fn prove_quad<F>(
        claim: &Scalar,
        blind_claim: &Scalar,
        num_rounds: usize,
        poly_A: &mut DensePolynomial,
        poly_B: &mut DensePolynomial,
        comb_func: F,
        gens_1: &MultiCommitGens,
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> (Self, Vec<Scalar>, Vec<Scalar>, Scalar)
    where
        F: Fn(&Scalar, &Scalar) -> Scalar,
    {
        panic!("STUB: not implemented");
    }
    pub fn prove_cubic_with_additive_term<F>(
        claim: &Scalar,
        blind_claim: &Scalar,
        num_rounds: usize,
        poly_A: &mut DensePolynomial,
        poly_B: &mut DensePolynomial,
        poly_C: &mut DensePolynomial,
        poly_D: &mut DensePolynomial,
        comb_func: F,
        gens_1: &MultiCommitGens,
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> (Self, Vec<Scalar>, Vec<Scalar>, Scalar)
    where
        F: Fn(&Scalar, &Scalar, &Scalar, &Scalar) -> Scalar,
    {
        panic!("STUB: not implemented");
    }
}
