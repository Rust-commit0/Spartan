#![allow(clippy::too_many_arguments)]
use super::commitments::{Commitments, MultiCommitGens};
use super::errors::ProofVerifyError;
use super::group::{CompressedGroup, CompressedGroupExt};
use super::math::Math;
use super::random::RandomTape;
use super::scalar::Scalar;
use super::transcript::{AppendToTranscript, ProofTranscript};
use merlin::Transcript;
use serde::{Deserialize, Serialize};
mod bullet;
use bullet::BulletReductionProof;
#[derive(Serialize, Deserialize, Debug)]
pub struct KnowledgeProof {
    alpha: CompressedGroup,
    z1: Scalar,
    z2: Scalar,
}
impl KnowledgeProof {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
        x: &Scalar,
        r: &Scalar,
    ) -> (KnowledgeProof, CompressedGroup) {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        C: &CompressedGroup,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct EqualityProof {
    alpha: CompressedGroup,
    z: Scalar,
}
impl EqualityProof {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
        v1: &Scalar,
        s1: &Scalar,
        v2: &Scalar,
        s2: &Scalar,
    ) -> (EqualityProof, CompressedGroup, CompressedGroup) {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        C1: &CompressedGroup,
        C2: &CompressedGroup,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductProof {
    alpha: CompressedGroup,
    beta: CompressedGroup,
    delta: CompressedGroup,
    z: [Scalar; 5],
}
impl ProductProof {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
        x: &Scalar,
        rX: &Scalar,
        y: &Scalar,
        rY: &Scalar,
        z: &Scalar,
        rZ: &Scalar,
    ) -> (ProductProof, CompressedGroup, CompressedGroup, CompressedGroup) {
        panic!("STUB: not implemented");
    }
    fn check_equality(
        P: &CompressedGroup,
        X: &CompressedGroup,
        c: &Scalar,
        gens_n: &MultiCommitGens,
        z1: &Scalar,
        z2: &Scalar,
    ) -> bool {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        X: &CompressedGroup,
        Y: &CompressedGroup,
        Z: &CompressedGroup,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DotProductProof {
    delta: CompressedGroup,
    beta: CompressedGroup,
    z: Vec<Scalar>,
    z_delta: Scalar,
    z_beta: Scalar,
}
impl DotProductProof {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    pub fn compute_dotproduct(a: &[Scalar], b: &[Scalar]) -> Scalar {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        gens_1: &MultiCommitGens,
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
        x_vec: &[Scalar],
        blind_x: &Scalar,
        a_vec: &[Scalar],
        y: &Scalar,
        blind_y: &Scalar,
    ) -> (DotProductProof, CompressedGroup, CompressedGroup) {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        gens_1: &MultiCommitGens,
        gens_n: &MultiCommitGens,
        transcript: &mut Transcript,
        a: &[Scalar],
        Cx: &CompressedGroup,
        Cy: &CompressedGroup,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[derive(Serialize, Deserialize)]
pub struct DotProductProofGens {
    n: usize,
    pub gens_n: MultiCommitGens,
    pub gens_1: MultiCommitGens,
}
impl DotProductProofGens {
    pub fn new(n: usize, label: &[u8]) -> Self {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DotProductProofLog {
    bullet_reduction_proof: BulletReductionProof,
    delta: CompressedGroup,
    beta: CompressedGroup,
    z1: Scalar,
    z2: Scalar,
}
impl DotProductProofLog {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    pub fn compute_dotproduct(a: &[Scalar], b: &[Scalar]) -> Scalar {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        gens: &DotProductProofGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
        x_vec: &[Scalar],
        blind_x: &Scalar,
        a_vec: &[Scalar],
        y: &Scalar,
        blind_y: &Scalar,
    ) -> (DotProductProofLog, CompressedGroup, CompressedGroup) {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        n: usize,
        gens: &DotProductProofGens,
        transcript: &mut Transcript,
        a: &[Scalar],
        Cx: &CompressedGroup,
        Cy: &CompressedGroup,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    #[test]
    fn check_knowledgeproof() {
        let mut csprng: OsRng = OsRng;
        let gens_1 = MultiCommitGens::new(1, b"test-knowledgeproof");
        let x = Scalar::random(&mut csprng);
        let r = Scalar::random(&mut csprng);
        let mut random_tape = RandomTape::new(b"proof");
        let mut prover_transcript = Transcript::new(b"example");
        let (proof, committed_value) = KnowledgeProof::prove(
            &gens_1,
            &mut prover_transcript,
            &mut random_tape,
            &x,
            &r,
        );
        let mut verifier_transcript = Transcript::new(b"example");
        assert!(
            proof.verify(& gens_1, & mut verifier_transcript, & committed_value).is_ok()
        );
    }
    #[test]
    fn check_equalityproof() {
        let mut csprng: OsRng = OsRng;
        let gens_1 = MultiCommitGens::new(1, b"test-equalityproof");
        let v1 = Scalar::random(&mut csprng);
        let v2 = v1;
        let s1 = Scalar::random(&mut csprng);
        let s2 = Scalar::random(&mut csprng);
        let mut random_tape = RandomTape::new(b"proof");
        let mut prover_transcript = Transcript::new(b"example");
        let (proof, C1, C2) = EqualityProof::prove(
            &gens_1,
            &mut prover_transcript,
            &mut random_tape,
            &v1,
            &s1,
            &v2,
            &s2,
        );
        let mut verifier_transcript = Transcript::new(b"example");
        assert!(proof.verify(& gens_1, & mut verifier_transcript, & C1, & C2).is_ok());
    }
    #[test]
    fn check_productproof() {
        let mut csprng: OsRng = OsRng;
        let gens_1 = MultiCommitGens::new(1, b"test-productproof");
        let x = Scalar::random(&mut csprng);
        let rX = Scalar::random(&mut csprng);
        let y = Scalar::random(&mut csprng);
        let rY = Scalar::random(&mut csprng);
        let z = x * y;
        let rZ = Scalar::random(&mut csprng);
        let mut random_tape = RandomTape::new(b"proof");
        let mut prover_transcript = Transcript::new(b"example");
        let (proof, X, Y, Z) = ProductProof::prove(
            &gens_1,
            &mut prover_transcript,
            &mut random_tape,
            &x,
            &rX,
            &y,
            &rY,
            &z,
            &rZ,
        );
        let mut verifier_transcript = Transcript::new(b"example");
        assert!(
            proof.verify(& gens_1, & mut verifier_transcript, & X, & Y, & Z).is_ok()
        );
    }
    #[test]
    fn check_dotproductproof() {
        let mut csprng: OsRng = OsRng;
        let n = 1024;
        let gens_1 = MultiCommitGens::new(1, b"test-two");
        let gens_1024 = MultiCommitGens::new(n, b"test-1024");
        let mut x: Vec<Scalar> = Vec::new();
        let mut a: Vec<Scalar> = Vec::new();
        for _ in 0..n {
            x.push(Scalar::random(&mut csprng));
            a.push(Scalar::random(&mut csprng));
        }
        let y = DotProductProofLog::compute_dotproduct(&x, &a);
        let r_x = Scalar::random(&mut csprng);
        let r_y = Scalar::random(&mut csprng);
        let mut random_tape = RandomTape::new(b"proof");
        let mut prover_transcript = Transcript::new(b"example");
        let (proof, Cx, Cy) = DotProductProof::prove(
            &gens_1,
            &gens_1024,
            &mut prover_transcript,
            &mut random_tape,
            &x,
            &r_x,
            &a,
            &y,
            &r_y,
        );
        let mut verifier_transcript = Transcript::new(b"example");
        assert!(
            proof.verify(& gens_1, & gens_1024, & mut verifier_transcript, & a, & Cx, &
            Cy).is_ok()
        );
    }
    #[test]
    fn check_dotproductproof_log() {
        let mut csprng: OsRng = OsRng;
        let n = 1024;
        let gens = DotProductProofGens::new(n, b"test-1024");
        let x: Vec<Scalar> = (0..n).map(|_i| Scalar::random(&mut csprng)).collect();
        let a: Vec<Scalar> = (0..n).map(|_i| Scalar::random(&mut csprng)).collect();
        let y = DotProductProof::compute_dotproduct(&x, &a);
        let r_x = Scalar::random(&mut csprng);
        let r_y = Scalar::random(&mut csprng);
        let mut random_tape = RandomTape::new(b"proof");
        let mut prover_transcript = Transcript::new(b"example");
        let (proof, Cx, Cy) = DotProductProofLog::prove(
            &gens,
            &mut prover_transcript,
            &mut random_tape,
            &x,
            &r_x,
            &a,
            &y,
            &r_y,
        );
        let mut verifier_transcript = Transcript::new(b"example");
        assert!(
            proof.verify(n, & gens, & mut verifier_transcript, & a, & Cx, & Cy).is_ok()
        );
    }
}
