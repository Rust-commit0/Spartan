use crate::transcript::AppendToTranscript;
use super::dense_mlpoly::DensePolynomial;
use super::errors::ProofVerifyError;
use super::math::Math;
use super::random::RandomTape;
use super::scalar::Scalar;
use super::sparse_mlpoly::{
    MultiSparseMatPolynomialAsDense, SparseMatEntry, SparseMatPolyCommitment,
    SparseMatPolyCommitmentGens, SparseMatPolyEvalProof, SparseMatPolynomial,
};
use super::timer::Timer;
use flate2::{write::ZlibEncoder, Compression};
use merlin::Transcript;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct R1CSShape {
    num_cons: usize,
    num_vars: usize,
    num_inputs: usize,
    A: SparseMatPolynomial,
    B: SparseMatPolynomial,
    C: SparseMatPolynomial,
}
#[derive(Serialize, Deserialize)]
pub struct R1CSCommitmentGens {
    gens: SparseMatPolyCommitmentGens,
}
impl R1CSCommitmentGens {
    pub fn new(
        label: &'static [u8],
        num_cons: usize,
        num_vars: usize,
        num_inputs: usize,
        num_nz_entries: usize,
    ) -> R1CSCommitmentGens {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct R1CSCommitment {
    num_cons: usize,
    num_vars: usize,
    num_inputs: usize,
    comm: SparseMatPolyCommitment,
}
impl AppendToTranscript for R1CSCommitment {
    fn append_to_transcript(&self, _label: &'static [u8], transcript: &mut Transcript) {
        panic!("STUB: not implemented");
    }
}
#[derive(Serialize, Deserialize)]
pub struct R1CSDecommitment {
    dense: MultiSparseMatPolynomialAsDense,
}
impl R1CSCommitment {
    pub fn get_num_cons(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub fn get_num_vars(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub fn get_num_inputs(&self) -> usize {
        panic!("STUB: not implemented");
    }
}
impl R1CSShape {
    pub fn new(
        num_cons: usize,
        num_vars: usize,
        num_inputs: usize,
        A: &[(usize, usize, Scalar)],
        B: &[(usize, usize, Scalar)],
        C: &[(usize, usize, Scalar)],
    ) -> R1CSShape {
        panic!("STUB: not implemented");
    }
    pub fn get_num_vars(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub fn get_num_cons(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub fn get_num_inputs(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub fn get_digest(&self) -> Vec<u8> {
        panic!("STUB: not implemented");
    }
    pub fn produce_synthetic_r1cs(
        num_cons: usize,
        num_vars: usize,
        num_inputs: usize,
    ) -> (R1CSShape, Vec<Scalar>, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
    pub fn is_sat(&self, vars: &[Scalar], input: &[Scalar]) -> bool {
        panic!("STUB: not implemented");
    }
    pub fn multiply_vec(
        &self,
        num_rows: usize,
        num_cols: usize,
        z: &[Scalar],
    ) -> (DensePolynomial, DensePolynomial, DensePolynomial) {
        panic!("STUB: not implemented");
    }
    pub fn compute_eval_table_sparse(
        &self,
        num_rows: usize,
        num_cols: usize,
        evals: &[Scalar],
    ) -> (Vec<Scalar>, Vec<Scalar>, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
    pub fn evaluate(&self, rx: &[Scalar], ry: &[Scalar]) -> (Scalar, Scalar, Scalar) {
        panic!("STUB: not implemented");
    }
    pub fn commit(
        &self,
        gens: &R1CSCommitmentGens,
    ) -> (R1CSCommitment, R1CSDecommitment) {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct R1CSEvalProof {
    proof: SparseMatPolyEvalProof,
}
impl R1CSEvalProof {
    pub fn prove(
        decomm: &R1CSDecommitment,
        rx: &[Scalar],
        ry: &[Scalar],
        evals: &(Scalar, Scalar, Scalar),
        gens: &R1CSCommitmentGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> R1CSEvalProof {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        comm: &R1CSCommitment,
        rx: &[Scalar],
        ry: &[Scalar],
        evals: &(Scalar, Scalar, Scalar),
        gens: &R1CSCommitmentGens,
        transcript: &mut Transcript,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
