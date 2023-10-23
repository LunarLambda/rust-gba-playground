use super::*;

/// Access to the display control.
pub struct Dispcnt<M>(pub(crate) PhantomData<M>);

impl<M: VideoMode> Dispcnt<M> {
    pub fn write(&mut self, cnt: DisplayControl) {
        let bits = cnt.0 .0 | M::N;

        unsafe {
            (0x0400_0000 as *mut u16).write_volatile(bits);
        }
    }

    pub fn read(&self) -> DisplayControl {
        let bits = unsafe { (0x0400_0000 as *mut u16).read_volatile() };

        DisplayControl(Bits(bits))
    }
}

/// Access to the display status.
#[non_exhaustive]
pub struct Dispstat;
