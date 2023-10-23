#![no_std]

pub mod bits;
pub mod video;

use video::*;

pub use gba_macros::entry;

pub struct Gba {
    pub display: Display,
    pub effects: Effects,
}

impl Gba {
    pub unsafe fn new_unchecked() -> Self {
        Self {
            display: Display::new(),
            effects: Effects::new(),
        }
    }
}
