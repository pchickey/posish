//! Time-related operations.

#[cfg(not(target_os = "redox"))]
mod clock;

// TODO: Convert WASI'S clock APIs to use handles rather than ambient
// clock identifiers, update `wasi-libc`, and then add support in `posish`.
#[cfg(not(any(target_os = "wasi", target_os = "redox")))]
pub use clock::{clock_getres, clock_gettime, ClockId};

#[cfg(not(target_os = "redox"))]
pub use clock::{timespec, UTIME_NOW, UTIME_OMIT};
