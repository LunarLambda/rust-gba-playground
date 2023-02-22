#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
#[repr(transparent)]
pub struct Bits<T>(pub T);

macro_rules! bits_impl {
    ($($T:ty)*) => {$(
        impl Bits<$T> {
            /// Checks that the given offset and length fit inside of the type.
            const fn check_field(offset: usize, length: usize) {
                assert!(offset < <$T>::BITS as usize);
                assert!(offset + length <= <$T>::BITS as usize);
            }

            /// Checks that the given value fits inside of the bitfield.
            const fn check_value(length: usize, value: $T)
            {
                assert!(value < (1 << length) - 1);
            }

            /// Return the bitmask for the given bitfield.
            pub const fn mask(offset: usize, length: usize) -> $T {
                Self::check_field(offset, length);

                ((1 << length) - 1) << offset
            }

            /// Sets the given bit to the given value.
            pub const fn set_bit(mut self, bit: usize, value: bool) -> Self {
                Self::check_field(bit, 1);

                self.0 &= !(1 << bit);
                self.0 |= (value as $T) << bit;
                self
            }

            /// Gets the given bit.
            pub const fn get_bit(self, bit: usize) -> bool {
                Self::check_field(bit, 1);

                self.0 & (1 << bit) != 0
            }

            /// Sets the given bits to the given value.
            pub const fn set_bits(mut self, offset: usize, length: usize, value: $T) -> Self
            {
                Self::check_field(offset, length);
                Self::check_value(length, value);

                let mask = Self::mask(offset, length);

                self.0 &= !mask;
                self.0 |= value << offset;
                self
            }

            /// Gets the given bits.
            pub const fn get_bits(self, offset: usize, length: usize) -> $T {
                Self::check_field(offset, length);

                (self.0 & Self::mask(offset, length)) >> offset
            }

            /// Splices the given bits from the given value.
            pub const fn splice_bits(mut self, offset: usize, length: usize, value: $T) -> Self {
                Self::check_field(offset, length);

                let mask = Self::mask(offset, length);

                self.0 &= !mask;
                self.0 |= value & mask;
                self
            }

            /// Isolates the given bits.
            pub const fn isolate_bits(mut self, offset: usize, length: usize) -> Self {
                Self::check_field(offset, length);

                self.0 &= Self::mask(offset, length);
                self
            }

            /// Returns a new value with the given bits.
            pub const fn with_bits(offset: usize, length: usize, value: $T) -> Self {
                Self(0).set_bits(offset, length, value)
            }

            /// Returns a new value with the given bit.
            pub const fn with_bit(bit: usize, value: bool) -> Self {
                Self(0).set_bit(bit, value)
            }
        }
    )*}
}

bits_impl! { u8 u16 u32 u64 u128 }
