#[cfg(target_arch = "x86_64")]
#[macro_use]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;


#[cfg(target_arch = "riscv")]
#[macro_use]
pub mod riscv;
#[cfg(target_arch = "riscv")]
pub use self::riscv::*;

#[cfg(target_arch = "aarch64")]
#[macro_use]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use self::aarch64::*;
