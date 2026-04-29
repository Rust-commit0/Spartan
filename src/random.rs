use super::scalar::Scalar;
use super::transcript::ProofTranscript;
use merlin::Transcript;
use rand::rngs::OsRng;
pub struct RandomTape {
    tape: Transcript,
}
impl RandomTape {
    pub fn new(name: &'static [u8]) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn random_scalar(&mut self, label: &'static [u8]) -> Scalar {
        panic!("STUB: not implemented");
    }
    pub fn random_vector(&mut self, label: &'static [u8], len: usize) -> Vec<Scalar> {
        panic!("STUB: not implemented");
    }
}
