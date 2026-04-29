mod ristretto255;
pub type Scalar = ristretto255::Scalar;
pub type ScalarBytes = curve25519_dalek::scalar::Scalar;
pub trait ScalarFromPrimitives {
    fn to_scalar(self) -> Scalar;
}
impl ScalarFromPrimitives for usize {
    #[inline]
    fn to_scalar(self) -> Scalar {
        panic!("STUB: not implemented");
    }
}
impl ScalarFromPrimitives for bool {
    #[inline]
    fn to_scalar(self) -> Scalar {
        panic!("STUB: not implemented");
    }
}
pub trait ScalarBytesFromScalar {
    fn decompress_scalar(s: &Scalar) -> ScalarBytes;
}
impl ScalarBytesFromScalar for Scalar {
    fn decompress_scalar(s: &Scalar) -> ScalarBytes {
        panic!("STUB: not implemented");
    }
}
