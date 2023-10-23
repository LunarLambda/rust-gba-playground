use core::marker::PhantomData;

use bitflags::bitflags;

/// Memory-mapped I/O registers.
pub mod mmio;
use mmio::*;
pub struct DisplayControl(Bits<u16>);

bitflags! {
    /// Display layers.
    pub struct Layers: u16 {
        const BG0 = 1;
        const BG1 = 2;
        const BG2 = 4;
        const BG3 = 8;
        const OBJ = 16;
        /// Only for blending
        const BACKDROP = 32;
        /// Window 0. Only for dispcnt.
        const WIN0 = 32;
        /// Window 1. Only for dispcnt.
        const WIN1 = 64;
        /// Object window. Only for dispcnt. Requires OBJ layer.
        const OBJ_WIN = 128 | 16;
    }
}

impl DisplayControl {
    pub const fn new() -> Self {
        Self(Bits(0))
    }

    pub const fn enable_layers(mut self, layers: Layers) -> Self {
        self.0 = self.0.or_field(8, 5, layers.bits());
        self
    }
}

mod sealed {
    pub trait Sealed {
        const N: u16;
        fn layers() -> Self;
    }
}

/// Size of a tiled background.
pub enum TiledBgSize {
    _256x256 = 0,
    _256x512 = 1,
    _512x256 = 2,
    _512x512 = 3,
}

/// Size of an affine background.
pub enum AffineBgSize {
    _128x128 = 0,
    _256x256 = 1,
    _512x512 = 2,
    _1024x1024 = 3,
}

/// Palettes used by a tiled background or object.
pub enum PaletteMode {
    /// 16 palettes of 16 colors
    Multi = 0,
    /// 1 palette of 256 colors
    Single = 1,
}

impl<T> BackgroundControl<T> {
    pub const fn new() -> Self {
        Self(Bits(0), PhantomData)
    }
}

impl BackgroundControl<Tiled> {
    /// Sets the background's size.
    pub const fn size(mut self, size: TiledBgSize) -> Self {
        self.0 = self.0.set_field(14, 2, size as u16);
        self
    }

    /// Sets the starting address for the background's tile data.
    pub const fn graphics_block(mut self, block: usize) -> Self {
        self.0 = self.0.set_field(2, 2, block as _);
        self
    }

    /// Sets the starting address for the background's map data.
    pub const fn map_block(mut self, block: usize) -> Self {
        self.0 = self.0.set_field(8, 5, block as _);
        self
    }

    /// Sets the background's priority
    pub const fn priority(mut self, prio: usize) -> Self {
        self.0 = self.0.set_field(0, 2, prio as _);
        self
    }

    /// Enables the mosaic for this background.
    pub const fn enable_mosaic(mut self, enable: bool) -> Self {
        self.0 = self.0.set_bit(6, enable);
        self
    }

    /// Sets the background's palette mode.
    pub const fn palette_mode(mut self, pal: PaletteMode) -> Self {
        self.0 = self.0.set_field(7, 1, pal as _);
        self
    }
}

impl BackgroundControl<Affine> {
    /// Sets the background's size.
    pub const fn size(self, _size: AffineBgSize) -> Self {
        self
    }

    /// Sets the starting address for the background's tile data.
    pub const fn graphics_block(self, _block: usize) -> Self {
        self
    }

    /// Sets the starting address for the background's map data.
    pub const fn map_block(self, _block: usize) -> Self {
        self
    }

    /// Sets the background's priority
    pub const fn priority(self, _prio: usize) -> Self {
        self
    }

    /// Enables the mosaic for this background.
    pub const fn enable_mosaic(self, _enable: bool) -> Self {
        self
    }
}

///
pub struct Bg<const N: usize, Type> {
    _p: PhantomData<Type>,
}

impl<const N: usize, Type> Bg<N, Type> {
    const INIT: Self = Self { _p: PhantomData };
}

/// Marker types.
pub mod marker {
    /// Marker for tiled backgrounds.
    pub struct Tiled {}
    /// Marker for affine backgrounds.
    pub struct Affine {}
    /// Marker for bitmap backgrounds.
    pub struct Bitmap {}
}

use crate::bits::Bits;
use marker::*;

#[repr(transparent)]
pub struct BackgroundControl<T>(Bits<u16>, PhantomData<T>);

impl<const N: usize> Bg<N, Tiled> {
    pub fn write_control(_cnt: BackgroundControl<Tiled>) {}
}

/// Access to various graphical effects.
pub struct Effects {
    pub window0: (),
    pub window1: (),
    pub obj_window: (),
    pub mosaic: (),
    pub blending: (),
}

impl Effects {
    pub(crate) const fn new() -> Self {
        Self {
            window0: (),
            window1: (),
            obj_window: (),
            mosaic: (),
            blending: (),
        }
    }
}

/// Different blending functions that can be applied.
pub enum BlendOp {
    None,
    Alpha(u8, u8),
    White(u8),
    Black(u8),
}

/// Background layers for mode 0.
pub struct Mode0 {
    pub bg0: Bg<0, Tiled>,
    pub bg1: Bg<1, Tiled>,
    pub bg2: Bg<2, Tiled>,
    pub bg3: Bg<3, Tiled>,
    pub obj: (),
}

/// Background layers for mode 1.
pub struct Mode1 {
    pub bg0: Bg<0, Tiled>,
    pub bg1: Bg<1, Affine>,
    pub bg2: Bg<2, Affine>,
    pub obj: (),
}

/// Background layers for mode 2.
pub struct Mode2 {
    pub bg2: Bg<0, Affine>,
    pub bg3: Bg<1, Affine>,
    pub obj: (),
}

/// Background layers for mode 3.
pub struct Mode3 {
    pub bg2: Bg<2, Bitmap>,
    pub obj: (),
}

/// Background layers for mode 4.
pub struct Mode4 {
    pub bg2: (),
    pub obj: (),
}

/// Background layers for mode 5.
pub struct Mode5 {
    pub bg2: (),
    pub obj: (),
}

/// Marker trait for video modes.
pub trait VideoMode: sealed::Sealed {}

impl VideoMode for () {}
impl VideoMode for Mode0 {}
impl VideoMode for Mode3 {}

/// Marker trait for video modes.
impl sealed::Sealed for () {
    const N: u16 = 0;
    fn layers() -> Self {
        ()
    }
}

impl sealed::Sealed for Mode0 {
    const N: u16 = 0;
    fn layers() -> Self {
        Self {
            bg0: Bg::INIT,
            bg1: Bg::INIT,
            bg2: Bg::INIT,
            bg3: Bg::INIT,
            obj: (),
        }
    }
}

impl sealed::Sealed for Mode3 {
    const N: u16 = 3;
    fn layers() -> Self {
        Self {
            bg2: Bg::INIT,
            obj: (),
        }
    }
}

/// Access to the video hardware.
pub struct Display<M: VideoMode = ()> {
    pub control: Dispcnt<M>,
    pub status: Dispstat,
    pub layers: M,
}

impl<T: VideoMode> Display<T> {
    pub fn mode<M: VideoMode>(self) -> Display<M> {
        Display::new()
    }

    pub(crate) fn new() -> Self {
        Self {
            control: Dispcnt(PhantomData),
            status: Dispstat,
            layers: T::layers(),
        }
    }
}

/// Reads the value of the vertical (scanline) counter.
pub fn vcount() -> u16 {
    unsafe { (0x0400_0006 as *const u16).read_volatile() }
}
