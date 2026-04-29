#[cfg(feature = "profile")]
use colored::Colorize;
#[cfg(feature = "profile")]
use core::sync::atomic::AtomicUsize;
#[cfg(feature = "profile")]
use core::sync::atomic::Ordering;
#[cfg(feature = "profile")]
use std::time::Instant;
#[cfg(feature = "profile")]
pub static CALL_DEPTH: AtomicUsize = AtomicUsize::new(0);
#[cfg(feature = "profile")]
pub struct Timer {
    label: String,
    timer: Instant,
}
#[cfg(feature = "profile")]
impl Timer {
    #[inline(always)]
    pub fn new(label: &str) -> Self {
        panic!("STUB: not implemented");
    }
    #[inline(always)]
    pub fn stop(&self) {
        panic!("STUB: not implemented");
    }
    #[inline(always)]
    pub fn print(msg: &str) {
        panic!("STUB: not implemented");
    }
}
#[cfg(not(feature = "profile"))]
pub struct Timer {
    _label: String,
}
#[cfg(not(feature = "profile"))]
impl Timer {
    #[inline(always)]
    pub fn new(label: &str) -> Self {
        panic!("STUB: not implemented");
    }
    #[inline(always)]
    pub fn stop(&self) {
        panic!("STUB: not implemented");
    }
    #[inline(always)]
    pub fn print(_msg: &str) {
        panic!("STUB: not implemented");
    }
}
