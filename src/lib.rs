#![allow(non_snake_case)]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![allow(clippy::assertions_on_result_states)]
extern crate byteorder;
extern crate core;
extern crate curve25519_dalek;
extern crate digest;
extern crate merlin;
extern crate sha3;
#[cfg(feature = "multicore")]
extern crate rayon;
mod commitments;
mod dense_mlpoly;
mod errors;
mod group;
mod math;
mod nizk;
mod product_tree;
mod r1cs;
mod r1csproof;
mod random;
mod scalar;
mod sparse_mlpoly;
mod sumcheck;
mod timer;
mod transcript;
mod unipoly;
use core::cmp::max;
use errors::{ProofVerifyError, R1CSError};
use merlin::Transcript;
use r1cs::{
    R1CSCommitment, R1CSCommitmentGens, R1CSDecommitment, R1CSEvalProof, R1CSShape,
};
use r1csproof::{R1CSGens, R1CSProof};
use random::RandomTape;
use scalar::Scalar;
use serde::{Deserialize, Serialize};
use timer::Timer;
use transcript::{AppendToTranscript, ProofTranscript};
/// `ComputationCommitment` holds a public preprocessed NP statement (e.g., R1CS)
#[derive(Serialize, Deserialize)]
pub struct ComputationCommitment {
    comm: R1CSCommitment,
}
/// `ComputationDecommitment` holds information to decommit `ComputationCommitment`
#[derive(Serialize, Deserialize)]
pub struct ComputationDecommitment {
    decomm: R1CSDecommitment,
}
/// `Assignment` holds an assignment of values to either the inputs or variables in an `Instance`
#[derive(Clone, Serialize, Deserialize)]
pub struct Assignment {
    assignment: Vec<Scalar>,
}
impl Assignment {
    /// Constructs a new `Assignment` from a vector
    pub fn new(assignment: &[[u8; 32]]) -> Result<Assignment, R1CSError> {
        panic!("STUB: not implemented");
    }
    /// pads Assignment to the specified length
    fn pad(&self, len: usize) -> VarsAssignment {
        panic!("STUB: not implemented");
    }
}
/// `VarsAssignment` holds an assignment of values to variables in an `Instance`
pub type VarsAssignment = Assignment;
/// `InputsAssignment` holds an assignment of values to variables in an `Instance`
pub type InputsAssignment = Assignment;
/// `Instance` holds the description of R1CS matrices and a hash of the matrices
pub struct Instance {
    inst: R1CSShape,
    digest: Vec<u8>,
}
impl Instance {
    /// Constructs a new `Instance` and an associated satisfying assignment
    pub fn new(
        num_cons: usize,
        num_vars: usize,
        num_inputs: usize,
        A: &[(usize, usize, [u8; 32])],
        B: &[(usize, usize, [u8; 32])],
        C: &[(usize, usize, [u8; 32])],
    ) -> Result<Instance, R1CSError> {
        panic!("STUB: not implemented");
    }
    /// Checks if a given R1CSShape is satisfiable with a given variables and inputs assignments
    pub fn is_sat(
        &self,
        vars: &VarsAssignment,
        inputs: &InputsAssignment,
    ) -> Result<bool, R1CSError> {
        panic!("STUB: not implemented");
    }
    /// Constructs a new synthetic R1CS `Instance` and an associated satisfying assignment
    pub fn produce_synthetic_r1cs(
        num_cons: usize,
        num_vars: usize,
        num_inputs: usize,
    ) -> (Instance, VarsAssignment, InputsAssignment) {
        panic!("STUB: not implemented");
    }
}
/// `SNARKGens` holds public parameters for producing and verifying proofs with the Spartan SNARK
#[derive(Serialize, Deserialize)]
pub struct SNARKGens {
    gens_r1cs_sat: R1CSGens,
    gens_r1cs_eval: R1CSCommitmentGens,
}
impl SNARKGens {
    /// Constructs a new `SNARKGens` given the size of the R1CS statement
    /// `num_nz_entries` specifies the maximum number of non-zero entries in any of the three R1CS matrices
    pub fn new(
        num_cons: usize,
        num_vars: usize,
        num_inputs: usize,
        num_nz_entries: usize,
    ) -> Self {
        panic!("STUB: not implemented");
    }
}
/// `SNARK` holds a proof produced by Spartan SNARK
#[derive(Serialize, Deserialize, Debug)]
pub struct SNARK {
    r1cs_sat_proof: R1CSProof,
    inst_evals: (Scalar, Scalar, Scalar),
    r1cs_eval_proof: R1CSEvalProof,
}
impl SNARK {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    /// A public computation to create a commitment to an R1CS instance
    pub fn encode(
        inst: &Instance,
        gens: &SNARKGens,
    ) -> (ComputationCommitment, ComputationDecommitment) {
        panic!("STUB: not implemented");
    }
    /// A method to produce a SNARK proof of the satisfiability of an R1CS instance
    pub fn prove(
        inst: &Instance,
        comm: &ComputationCommitment,
        decomm: &ComputationDecommitment,
        vars: VarsAssignment,
        inputs: &InputsAssignment,
        gens: &SNARKGens,
        transcript: &mut Transcript,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    /// A method to verify the SNARK proof of the satisfiability of an R1CS instance
    pub fn verify(
        &self,
        comm: &ComputationCommitment,
        input: &InputsAssignment,
        transcript: &mut Transcript,
        gens: &SNARKGens,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
/// `NIZKGens` holds public parameters for producing and verifying proofs with the Spartan NIZK
pub struct NIZKGens {
    gens_r1cs_sat: R1CSGens,
}
impl NIZKGens {
    /// Constructs a new `NIZKGens` given the size of the R1CS statement
    pub fn new(num_cons: usize, num_vars: usize, num_inputs: usize) -> Self {
        panic!("STUB: not implemented");
    }
}
/// `NIZK` holds a proof produced by Spartan NIZK
#[derive(Serialize, Deserialize, Debug)]
pub struct NIZK {
    r1cs_sat_proof: R1CSProof,
    r: (Vec<Scalar>, Vec<Scalar>),
}
impl NIZK {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    /// A method to produce a NIZK proof of the satisfiability of an R1CS instance
    pub fn prove(
        inst: &Instance,
        vars: VarsAssignment,
        input: &InputsAssignment,
        gens: &NIZKGens,
        transcript: &mut Transcript,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    /// A method to verify a NIZK proof of the satisfiability of an R1CS instance
    pub fn verify(
        &self,
        inst: &Instance,
        input: &InputsAssignment,
        transcript: &mut Transcript,
        gens: &NIZKGens,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn check_snark() {
        let num_vars = 256;
        let num_cons = num_vars;
        let num_inputs = 10;
        let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_cons);
        let (inst, vars, inputs) = Instance::produce_synthetic_r1cs(
            num_cons,
            num_vars,
            num_inputs,
        );
        let (comm, decomm) = SNARK::encode(&inst, &gens);
        let mut prover_transcript = Transcript::new(b"example");
        let proof = SNARK::prove(
            &inst,
            &comm,
            &decomm,
            vars,
            &inputs,
            &gens,
            &mut prover_transcript,
        );
        let mut verifier_transcript = Transcript::new(b"example");
        assert!(
            proof.verify(& comm, & inputs, & mut verifier_transcript, & gens).is_ok()
        );
    }
    #[test]
    pub fn check_r1cs_invalid_index() {
        let num_cons = 4;
        let num_vars = 8;
        let num_inputs = 1;
        let zero: [u8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        let A = vec![(0, 0, zero)];
        let B = vec![(100, 1, zero)];
        let C = vec![(1, 1, zero)];
        let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C);
        assert!(inst.is_err());
        assert_eq!(inst.err(), Some(R1CSError::InvalidIndex));
    }
    #[test]
    pub fn check_r1cs_invalid_scalar() {
        let num_cons = 4;
        let num_vars = 8;
        let num_inputs = 1;
        let zero: [u8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        let larger_than_mod = [
            3, 0, 0, 0, 255, 255, 255, 255, 254, 91, 254, 255, 2, 164, 189, 83, 5, 216,
            161, 9, 8, 216, 57, 51, 72, 125, 157, 41, 83, 167, 237, 115,
        ];
        let A = vec![(0, 0, zero)];
        let B = vec![(1, 1, larger_than_mod)];
        let C = vec![(1, 1, zero)];
        let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C);
        assert!(inst.is_err());
        assert_eq!(inst.err(), Some(R1CSError::InvalidScalar));
    }
    #[test]
    fn test_padded_constraints() {
        let num_cons = 1;
        let num_vars = 0;
        let num_inputs = 3;
        let num_non_zero_entries = 3;
        let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
        let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
        let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();
        A.push((0, num_vars + 2, Scalar::one().to_bytes()));
        B.push((0, num_vars + 2, Scalar::one().to_bytes()));
        C.push((0, num_vars + 1, Scalar::one().to_bytes()));
        C.push((0, num_vars, (-Scalar::from(13u64)).to_bytes()));
        C.push((0, num_vars + 3, (-Scalar::one()).to_bytes()));
        let vars = vec![Scalar::zero().to_bytes(); num_vars];
        let mut inputs = vec![Scalar::zero().to_bytes(); num_inputs];
        inputs[0] = Scalar::from(16u64).to_bytes();
        inputs[1] = Scalar::from(1u64).to_bytes();
        inputs[2] = Scalar::from(2u64).to_bytes();
        let assignment_inputs = InputsAssignment::new(&inputs).unwrap();
        let assignment_vars = VarsAssignment::new(&vars).unwrap();
        let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();
        let res = inst.is_sat(&assignment_vars, &assignment_inputs);
        assert!(res.unwrap(), "should be satisfied");
        let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_non_zero_entries);
        let (comm, decomm) = SNARK::encode(&inst, &gens);
        let mut prover_transcript = Transcript::new(b"snark_example");
        let proof = SNARK::prove(
            &inst,
            &comm,
            &decomm,
            assignment_vars.clone(),
            &assignment_inputs,
            &gens,
            &mut prover_transcript,
        );
        let mut verifier_transcript = Transcript::new(b"snark_example");
        assert!(
            proof.verify(& comm, & assignment_inputs, & mut verifier_transcript, & gens)
            .is_ok()
        );
        let gens = NIZKGens::new(num_cons, num_vars, num_inputs);
        let mut prover_transcript = Transcript::new(b"nizk_example");
        let proof = NIZK::prove(
            &inst,
            assignment_vars,
            &assignment_inputs,
            &gens,
            &mut prover_transcript,
        );
        let mut verifier_transcript = Transcript::new(b"nizk_example");
        assert!(
            proof.verify(& inst, & assignment_inputs, & mut verifier_transcript, & gens)
            .is_ok()
        );
    }
}
