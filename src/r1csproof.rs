#![allow(clippy::too_many_arguments)]
use super::commitments::{Commitments, MultiCommitGens};
use super::dense_mlpoly::{
    DensePolynomial, EqPolynomial, PolyCommitment, PolyCommitmentGens, PolyEvalProof,
};
use super::errors::ProofVerifyError;
use super::group::{CompressedGroup, GroupElement, VartimeMultiscalarMul};
use super::math::Math;
use super::nizk::{EqualityProof, KnowledgeProof, ProductProof};
use super::r1cs::R1CSShape;
use super::random::RandomTape;
use super::scalar::Scalar;
use super::sparse_mlpoly::{SparsePolyEntry, SparsePolynomial};
use super::sumcheck::ZKSumcheckInstanceProof;
use super::timer::Timer;
use super::transcript::{AppendToTranscript, ProofTranscript};
use core::iter;
use merlin::Transcript;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct R1CSProof {
    comm_vars: PolyCommitment,
    sc_proof_phase1: ZKSumcheckInstanceProof,
    claims_phase2: (CompressedGroup, CompressedGroup, CompressedGroup, CompressedGroup),
    pok_claims_phase2: (KnowledgeProof, ProductProof),
    proof_eq_sc_phase1: EqualityProof,
    sc_proof_phase2: ZKSumcheckInstanceProof,
    comm_vars_at_ry: CompressedGroup,
    proof_eval_vars_at_ry: PolyEvalProof,
    proof_eq_sc_phase2: EqualityProof,
}
#[derive(Serialize, Deserialize)]
pub struct R1CSSumcheckGens {
    gens_1: MultiCommitGens,
    gens_3: MultiCommitGens,
    gens_4: MultiCommitGens,
}
impl R1CSSumcheckGens {
    pub fn new(label: &'static [u8], gens_1_ref: &MultiCommitGens) -> Self {
        panic!("STUB: not implemented");
    }
}
#[derive(Serialize, Deserialize)]
pub struct R1CSGens {
    gens_sc: R1CSSumcheckGens,
    gens_pc: PolyCommitmentGens,
}
impl R1CSGens {
    pub fn new(label: &'static [u8], _num_cons: usize, num_vars: usize) -> Self {
        panic!("STUB: not implemented");
    }
}
impl R1CSProof {
    fn prove_phase_one(
        num_rounds: usize,
        evals_tau: &mut DensePolynomial,
        evals_Az: &mut DensePolynomial,
        evals_Bz: &mut DensePolynomial,
        evals_Cz: &mut DensePolynomial,
        gens: &R1CSSumcheckGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> (ZKSumcheckInstanceProof, Vec<Scalar>, Vec<Scalar>, Scalar) {
        panic!("STUB: not implemented");
    }
    fn prove_phase_two(
        num_rounds: usize,
        claim: &Scalar,
        blind_claim: &Scalar,
        evals_z: &mut DensePolynomial,
        evals_ABC: &mut DensePolynomial,
        gens: &R1CSSumcheckGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> (ZKSumcheckInstanceProof, Vec<Scalar>, Vec<Scalar>, Scalar) {
        panic!("STUB: not implemented");
    }
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        inst: &R1CSShape,
        vars: Vec<Scalar>,
        input: &[Scalar],
        gens: &R1CSGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> (R1CSProof, Vec<Scalar>, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        num_vars: usize,
        num_cons: usize,
        input: &[Scalar],
        evals: &(Scalar, Scalar, Scalar),
        transcript: &mut Transcript,
        gens: &R1CSGens,
    ) -> Result<(Vec<Scalar>, Vec<Scalar>), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    fn produce_tiny_r1cs() -> (R1CSShape, Vec<Scalar>, Vec<Scalar>) {
        let num_cons = 128;
        let num_vars = 256;
        let num_inputs = 2;
        let mut A: Vec<(usize, usize, Scalar)> = Vec::new();
        let mut B: Vec<(usize, usize, Scalar)> = Vec::new();
        let mut C: Vec<(usize, usize, Scalar)> = Vec::new();
        let one = Scalar::one();
        A.push((0, 0, one));
        A.push((0, 1, one));
        B.push((0, num_vars + 1, one));
        C.push((0, 2, one));
        A.push((1, 0, one));
        A.push((1, num_vars + 2, one));
        B.push((1, 2, one));
        C.push((1, 3, one));
        A.push((2, 4, one));
        B.push((2, num_vars, one));
        let inst = R1CSShape::new(num_cons, num_vars, num_inputs, &A, &B, &C);
        let mut csprng: OsRng = OsRng;
        let i0 = Scalar::random(&mut csprng);
        let i1 = Scalar::random(&mut csprng);
        let z1 = Scalar::random(&mut csprng);
        let z2 = Scalar::random(&mut csprng);
        let z3 = (z1 + z2) * i0;
        let z4 = (z1 + i1) * z3;
        let z5 = Scalar::zero();
        let mut vars = vec![Scalar::zero(); num_vars];
        vars[0] = z1;
        vars[1] = z2;
        vars[2] = z3;
        vars[3] = z4;
        vars[4] = z5;
        let mut input = vec![Scalar::zero(); num_inputs];
        input[0] = i0;
        input[1] = i1;
        (inst, vars, input)
    }
    #[test]
    fn test_tiny_r1cs() {
        let (inst, vars, input) = tests::produce_tiny_r1cs();
        let is_sat = inst.is_sat(&vars, &input);
        assert!(is_sat);
    }
    #[test]
    fn test_synthetic_r1cs() {
        let (inst, vars, input) = R1CSShape::produce_synthetic_r1cs(1024, 1024, 10);
        let is_sat = inst.is_sat(&vars, &input);
        assert!(is_sat);
    }
    #[test]
    pub fn check_r1cs_proof() {
        let num_vars = 1024;
        let num_cons = num_vars;
        let num_inputs = 10;
        let (inst, vars, input) = R1CSShape::produce_synthetic_r1cs(
            num_cons,
            num_vars,
            num_inputs,
        );
        let gens = R1CSGens::new(b"test-m", num_cons, num_vars);
        let mut random_tape = RandomTape::new(b"proof");
        let mut prover_transcript = Transcript::new(b"example");
        let (proof, rx, ry) = R1CSProof::prove(
            &inst,
            vars,
            &input,
            &gens,
            &mut prover_transcript,
            &mut random_tape,
        );
        let inst_evals = inst.evaluate(&rx, &ry);
        let mut verifier_transcript = Transcript::new(b"example");
        assert!(
            proof.verify(inst.get_num_vars(), inst.get_num_cons(), & input, & inst_evals,
            & mut verifier_transcript, & gens,).is_ok()
        );
    }
}
