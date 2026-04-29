use super::group::{GroupElement, VartimeMultiscalarMul, GROUP_BASEPOINT_COMPRESSED};
use super::scalar::Scalar;
use digest::{ExtendableOutput, Input, XofReader};
use serde::{Deserialize, Serialize};
use sha3::Shake256;
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiCommitGens {
    pub n: usize,
    pub G: Vec<GroupElement>,
    pub h: GroupElement,
}
impl MultiCommitGens {
    pub fn new(n: usize, label: &[u8]) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn clone(&self) -> MultiCommitGens {
        panic!("STUB: not implemented");
    }
    pub fn scale(&self, s: &Scalar) -> MultiCommitGens {
        panic!("STUB: not implemented");
    }
    pub fn split_at(&self, mid: usize) -> (MultiCommitGens, MultiCommitGens) {
        panic!("STUB: not implemented");
    }
}
pub trait Commitments {
    fn commit(&self, blind: &Scalar, gens_n: &MultiCommitGens) -> GroupElement;
}
impl Commitments for Scalar {
    fn commit(&self, blind: &Scalar, gens_n: &MultiCommitGens) -> GroupElement {
        panic!("STUB: not implemented");
    }
}
impl Commitments for Vec<Scalar> {
    fn commit(&self, blind: &Scalar, gens_n: &MultiCommitGens) -> GroupElement {
        panic!("STUB: not implemented");
    }
}
impl Commitments for [Scalar] {
    fn commit(&self, blind: &Scalar, gens_n: &MultiCommitGens) -> GroupElement {
        panic!("STUB: not implemented");
    }
}
