#![no_std]

pub mod bits;

pub use gba_macros::entry;

pub struct Gba {}

impl Gba {
    pub unsafe fn new_unchecked() -> Self {
        Self {}
    }
}
