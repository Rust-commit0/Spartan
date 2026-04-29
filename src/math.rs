pub trait Math {
    fn pow2(self) -> usize;
    fn get_bits(self, num_bits: usize) -> Vec<bool>;
    fn log_2(self) -> usize;
}
impl Math for usize {
    #[inline]
    fn pow2(self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns the num_bits from n in a canonical order
    fn get_bits(self, num_bits: usize) -> Vec<bool> {
        panic!("STUB: not implemented");
    }
    fn log_2(self) -> usize {
        panic!("STUB: not implemented");
    }
}
