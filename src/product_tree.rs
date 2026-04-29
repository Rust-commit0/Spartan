#![allow(dead_code)]
use super::dense_mlpoly::DensePolynomial;
use super::dense_mlpoly::EqPolynomial;
use super::math::Math;
use super::scalar::Scalar;
use super::sumcheck::SumcheckInstanceProof;
use super::transcript::ProofTranscript;
use merlin::Transcript;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct ProductCircuit {
    left_vec: Vec<DensePolynomial>,
    right_vec: Vec<DensePolynomial>,
}
impl ProductCircuit {
    fn compute_layer(
        inp_left: &DensePolynomial,
        inp_right: &DensePolynomial,
    ) -> (DensePolynomial, DensePolynomial) {
        panic!("STUB: not implemented");
    }
    pub fn new(poly: &DensePolynomial) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn evaluate(&self) -> Scalar {
        panic!("STUB: not implemented");
    }
}
pub struct DotProductCircuit {
    left: DensePolynomial,
    right: DensePolynomial,
    weight: DensePolynomial,
}
impl DotProductCircuit {
    pub fn new(
        left: DensePolynomial,
        right: DensePolynomial,
        weight: DensePolynomial,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn evaluate(&self) -> Scalar {
        panic!("STUB: not implemented");
    }
    pub fn split(&mut self) -> (DotProductCircuit, DotProductCircuit) {
        panic!("STUB: not implemented");
    }
}
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct LayerProof {
    pub proof: SumcheckInstanceProof,
    pub claims: Vec<Scalar>,
}
#[allow(dead_code)]
impl LayerProof {
    pub fn verify(
        &self,
        claim: Scalar,
        num_rounds: usize,
        degree_bound: usize,
        transcript: &mut Transcript,
    ) -> (Scalar, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
}
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct LayerProofBatched {
    pub proof: SumcheckInstanceProof,
    pub claims_prod_left: Vec<Scalar>,
    pub claims_prod_right: Vec<Scalar>,
}
#[allow(dead_code)]
impl LayerProofBatched {
    pub fn verify(
        &self,
        claim: Scalar,
        num_rounds: usize,
        degree_bound: usize,
        transcript: &mut Transcript,
    ) -> (Scalar, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCircuitEvalProof {
    proof: Vec<LayerProof>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCircuitEvalProofBatched {
    proof: Vec<LayerProofBatched>,
    claims_dotp: (Vec<Scalar>, Vec<Scalar>, Vec<Scalar>),
}
impl ProductCircuitEvalProof {
    #![allow(dead_code)]
    pub fn prove(
        circuit: &mut ProductCircuit,
        transcript: &mut Transcript,
    ) -> (Self, Scalar, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        eval: Scalar,
        len: usize,
        transcript: &mut Transcript,
    ) -> (Scalar, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
}
impl ProductCircuitEvalProofBatched {
    pub fn prove(
        prod_circuit_vec: &mut [&mut ProductCircuit],
        dotp_circuit_vec: &mut [&mut DotProductCircuit],
        transcript: &mut Transcript,
    ) -> (Self, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        claims_prod_vec: &[Scalar],
        claims_dotp_vec: &[Scalar],
        len: usize,
        transcript: &mut Transcript,
    ) -> (Vec<Scalar>, Vec<Scalar>, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
}
