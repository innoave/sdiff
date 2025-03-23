#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
mod std {
    pub use core::*;
}

#[cfg(feature = "std")]
mod std {
    pub use std::*;
}
