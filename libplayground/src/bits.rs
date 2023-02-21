#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
#[repr(transparent)]
pub struct Bits<T>(pub T);

macro_rules! bits_impl {
    ($($T:ty)*) => {$(
        impl Bits<$T> {
            pub const fn mask(length: usize) -> $T {
                assert!(length <= <$T>::BITS as usize);
                if length == 0 { 0 } else { <$T>::MAX >> (<$T>::BITS as usize - length) }
            }

            pub const fn shifted_mask(offset: usize, length: usize) -> $T {
                assert!(offset < <$T>::BITS as usize && offset + length <= <$T>::BITS as usize);
                Self::mask(length) << offset
            }

            pub const fn set_bit(mut self, bit: usize, value: bool) -> Self {
                assert!(bit < <$T>::BITS as usize);
                self.0 &= !(1 << bit);
                self.0 |= (value as $T) << bit;
                self
            }

            pub const fn get_bit(self, bit: usize) -> bool {
                assert!(bit < <$T>::BITS as usize);
                ((self.0 >> bit) & 1) == 1
            }

            pub const fn set_bits(mut self, offset: usize, length: usize, value: $T) -> Self
            {
                assert!(offset < <$T>::BITS as usize && offset + length <= <$T>::BITS as usize);
                let mask = (1 << length) - 1;
                self.0 &= !(mask << offset);
                self.0 |= (value & mask) << offset;
                self
            }

            pub const fn get_bits(self, offset: usize, length: usize) -> $T {
                assert!(offset < <$T>::BITS as usize && offset + length <= <$T>::BITS as usize);
                let mask = (1 << length) - 1;
                (self.0 >> offset) & mask
            }
        }
    )*}
}

bits_impl! { u8 u16 u32 u64 u128 }
