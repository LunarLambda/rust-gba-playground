//! Low-level wrapper type for manipulating bitfields

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
                assert!(length < <$T>::BITS as usize);
                assert!(value < (1 << length) - 1);
            }

            /// Return the bitmask for the given bitfield.
            pub const fn mask(offset: usize, length: usize) -> $T {
                Self::check_field(offset, length);

                ((1 << length) - 1) << offset
            }

            /// Set the given bit.
            pub const fn set_bit(self, offset: usize, value: bool) -> Self {
                self.set_field(offset, 1, value as $T)
            }

            /// Get the given bit.
            pub const fn get_bit(self, offset: usize) -> bool {
                self.get_field(offset, 1) != 0
            }

            /// Sets the given bits to the given value.
            pub const fn set_field(mut self, offset: usize, length: usize, value: $T) -> Self
            {
                Self::check_field(offset, length);
                Self::check_value(length, value);

                let mask = Self::mask(offset, length);

                self.0 &= !mask;
                self.0 |= value << offset;
                self
            }

            /// Gets the given bits.
            pub const fn get_field(self, offset: usize, length: usize) -> $T {
                Self::check_field(offset, length);

                (self.0 & Self::mask(offset, length)) >> offset
            }

            /// Logical ORs the given bits.
            pub const fn or_field(mut self, offset: usize, length: usize, value: $T) -> Self {
                Self::check_field(offset, length);
                Self::check_value(length, value);

                self.0 |= value << offset;
                self
            }

            /// Logical ANDs the given bits.
            pub const fn and_field(mut self, offset: usize, length: usize, value: $T) -> Self {
                Self::check_field(offset, length);
                Self::check_value(length, value);

                self.0 &= value << offset;
                self
            }

            /// Logical XORs the given bits.
            pub const fn xor_field(mut self, offset: usize, length: usize, value: $T) -> Self {
                Self::check_field(offset, length);
                Self::check_value(length, value);

                self.0 ^= value << offset;
                self
            }

            /// Performs a bit clear on the given bits.
            pub const fn bic_field(mut self, offset: usize, length: usize, value: $T) -> Self {
                Self::check_field(offset, length);
                Self::check_value(length, value);

                self.0 &= !(value << offset);
                self
            }

            /// Logical NOTs the given bits.
            pub const fn not_field(mut self, offset: usize, length: usize) -> Self {
                Self::check_field(offset, length);

                self.0 ^= Self::mask(offset, length);
                self
            }

            /// Isolate the given bits, clearing everything else.
            pub const fn isolate_field(self, offset: usize, length: usize) -> Self {
                Self::check_field(offset, length);

                Self(self.0 & Self::mask(offset, length))
            }

            /// Clears the given bits.
            pub const fn clear_field(mut self, offset: usize, length: usize) -> Self {
                Self::check_field(offset, length);

                self.0 &= !Self::mask(offset, length);
                self
            }
        }
    )*}
}

bits_impl! { u8 u16 u32 u64 u128 }
