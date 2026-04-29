use super::group::CompressedGroup;
use super::scalar::Scalar;
use merlin::Transcript;
pub trait ProofTranscript {
    fn append_protocol_name(&mut self, protocol_name: &'static [u8]);
    fn append_scalar(&mut self, label: &'static [u8], scalar: &Scalar);
    fn append_point(&mut self, label: &'static [u8], point: &CompressedGroup);
    fn challenge_scalar(&mut self, label: &'static [u8]) -> Scalar;
    fn challenge_vector(&mut self, label: &'static [u8], len: usize) -> Vec<Scalar>;
}
impl ProofTranscript for Transcript {
    fn append_protocol_name(&mut self, protocol_name: &'static [u8]) {
        panic!("STUB: not implemented");
    }
    fn append_scalar(&mut self, label: &'static [u8], scalar: &Scalar) {
        panic!("STUB: not implemented");
    }
    fn append_point(&mut self, label: &'static [u8], point: &CompressedGroup) {
        panic!("STUB: not implemented");
    }
    fn challenge_scalar(&mut self, label: &'static [u8]) -> Scalar {
        panic!("STUB: not implemented");
    }
    fn challenge_vector(&mut self, label: &'static [u8], len: usize) -> Vec<Scalar> {
        panic!("STUB: not implemented");
    }
}
pub trait AppendToTranscript {
    fn append_to_transcript(&self, label: &'static [u8], transcript: &mut Transcript);
}
impl AppendToTranscript for Scalar {
    fn append_to_transcript(&self, label: &'static [u8], transcript: &mut Transcript) {
        panic!("STUB: not implemented");
    }
}
impl AppendToTranscript for [Scalar] {
    fn append_to_transcript(&self, label: &'static [u8], transcript: &mut Transcript) {
        panic!("STUB: not implemented");
    }
}
impl AppendToTranscript for CompressedGroup {
    fn append_to_transcript(&self, label: &'static [u8], transcript: &mut Transcript) {
        panic!("STUB: not implemented");
    }
}
