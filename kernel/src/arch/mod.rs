// Export our platform-specific modules.

#[cfg(target_arch="x86_64")]
pub use self::x86_64::{io, mem};

#[cfg(target_arch="x86")]
pub use self::x86_64::{io, mem};

#[cfg(target_arch="x86_64")]
pub mod x86_64;

// This needs to be visible but not public on x86_64 (for things common to x86 and x86_64)
#[cfg(target_arch="x86_64")]
mod x86;

#[cfg(target_arch="x86")]
pub mod x86_64;
