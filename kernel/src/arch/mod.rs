// Export our platform-specific modules.

#[macro_use]
mod x86_64;

#[cfg(target_arch="x86_64")]
pub use self::x86_64::*;
