use super::errors::ProofVerifyError;
use super::scalar::{Scalar, ScalarBytes, ScalarBytesFromScalar};
use core::borrow::Borrow;
use core::ops::{Mul, MulAssign};
pub type GroupElement = curve25519_dalek::ristretto::RistrettoPoint;
pub type CompressedGroup = curve25519_dalek::ristretto::CompressedRistretto;
pub trait CompressedGroupExt {
    type Group;
    fn unpack(&self) -> Result<Self::Group, ProofVerifyError>;
}
impl CompressedGroupExt for CompressedGroup {
    type Group = curve25519_dalek::ristretto::RistrettoPoint;
    fn unpack(&self) -> Result<Self::Group, ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
pub const GROUP_BASEPOINT_COMPRESSED: CompressedGroup = curve25519_dalek::constants::RISTRETTO_BASEPOINT_COMPRESSED;
impl<'b> MulAssign<&'b Scalar> for GroupElement {
    fn mul_assign(&mut self, scalar: &'b Scalar) {
        panic!("STUB: not implemented");
    }
}
impl<'b> Mul<&'b Scalar> for &GroupElement {
    type Output = GroupElement;
    fn mul(self, scalar: &'b Scalar) -> GroupElement {
        panic!("STUB: not implemented");
    }
}
impl<'b> Mul<&'b GroupElement> for &Scalar {
    type Output = GroupElement;
    fn mul(self, point: &'b GroupElement) -> GroupElement {
        panic!("STUB: not implemented");
    }
}
macro_rules! define_mul_variants {
    (LHS = $lhs:ty, RHS = $rhs:ty, Output = $out:ty) => {
        impl <'b > Mul <&'b $rhs > for $lhs { type Output = $out; fn mul(self, rhs : &'b
        $rhs) -> $out { & self * rhs } } impl <'a > Mul <$rhs > for &'a $lhs { type
        Output = $out; fn mul(self, rhs : $rhs) -> $out { self * & rhs } } impl Mul <$rhs
        > for $lhs { type Output = $out; fn mul(self, rhs : $rhs) -> $out { & self * &
        rhs } }
    };
}
macro_rules! define_mul_assign_variants {
    (LHS = $lhs:ty, RHS = $rhs:ty) => {
        impl MulAssign <$rhs > for $lhs { fn mul_assign(& mut self, rhs : $rhs) { * self
        *= & rhs; } }
    };
}
define_mul_assign_variants!(LHS = GroupElement, RHS = Scalar);
define_mul_variants!(LHS = GroupElement, RHS = Scalar, Output = GroupElement);
define_mul_variants!(LHS = Scalar, RHS = GroupElement, Output = GroupElement);
pub trait VartimeMultiscalarMul {
    type Scalar;
    fn vartime_multiscalar_mul<I, J>(scalars: I, points: J) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Self::Scalar>,
        J: IntoIterator,
        J::Item: Borrow<Self>,
        Self: Clone;
}
impl VartimeMultiscalarMul for GroupElement {
    type Scalar = super::scalar::Scalar;
    fn vartime_multiscalar_mul<I, J>(scalars: I, points: J) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Self::Scalar>,
        J: IntoIterator,
        J::Item: Borrow<Self>,
        Self: Clone,
    {
        panic!("STUB: not implemented");
    }
}
