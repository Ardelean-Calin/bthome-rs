/// Asserts that constant expressions evaluate to `true`.
///
/// Constant expressions can be ensured to have certain properties via this
/// macro If the expression evaluates to `false`, the file will fail to compile.
/// This is synonymous to [`static_assert` in C++][static_assert].
#[macro_export]
macro_rules! const_assert {
    ($x:expr $(,)?) => {
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = $x;
            ASSERT
        } as usize] = [];
    };
}

/// # Normal usecase:
///
/// ```rust
/// use bthome;
/// use bthome::build_bthome_ad;
/// use bthome::const_assert;
/// build_bthome_ad!(
///     struct CompileFail {
///         field1: bthome::fields::Count_4bytes,
///         field2: bthome::fields::Count_4bytes,
///         field3: bthome::fields::Count_4bytes,
///     }
/// );
/// ```
/// # Automatic overflow detection if payload goes over 23 bytes (TODO BLE5 with more bytes):
///
/// ```compile_fail
/// use bthome;
/// use bthome::build_bthome_ad;
/// use bthome::const_assert;
/// build_bthome_ad!(
///     struct CompileFail {
///         field1: bthome::fields::Count_4bytes,
///         field2: bthome::fields::Count_4bytes,
///         field3: bthome::fields::Count_4bytes,
///         field4: bthome::fields::Count_4bytes,
///         field5: bthome::fields::Count_4bytes,
///         field6: bthome::fields::Count_4bytes,
///         field7: bthome::fields::Count_4bytes,
///     }
/// );
/// ```
#[macro_export]
macro_rules! build_bthome_ad {
    (struct $name:ident {
        $($field:ident: $ty:path,)*
    }) => {
        #[derive(Default, Clone)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name {
            $(
                $field: Option<$ty>,
            )*
        }

        impl $name {
            const _TOTAL_SIZE: usize = 0 $(+ core::mem::size_of::<$ty>())*;

            $(
                pub fn $field(self, $field: $ty) -> Self {
                    Self {
                        $field: Some($field),
                        ..self
                    }
                }
            )*

            pub fn as_vec(&self) -> Result<heapless::Vec<u8, 31>, ()>{
                let mut buf: heapless::Vec<u8, 31> = heapless::Vec::new();
                buf.extend_from_slice(&[
                    0x02,
                    0x01,
                    0x06, // BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE
                ])?;

                // The BTHome Header
                let header = [
                    0x04, // Size of the BTHome AD. Minimum 4 for the header.
                    0x16,
                    0xD2,
                    0xFC,
                    0x40,
                ];
                buf.extend_from_slice(&header)?;

                $(
                    if let Some($field) = self.$field.clone() {
                        buf.extend_from_slice($field.as_slice())?;
                        buf[3] += core::mem::size_of::<$ty>() as u8;
                    }
                )*

                Ok(buf)
            }
        }

        const_assert!($name::_TOTAL_SIZE <= 23);
    };
}

macro_rules! impl_field {
    ($name:ident, $id:literal, $internal_repr:ty, $external_repr:ty) => {
        #[derive(Clone)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name($internal_repr);

        impl $name {
            const ID: u8 = $id;
            const SIZE: usize = core::mem::size_of::<$internal_repr>() - 1;

            pub fn get(&self) -> $external_repr {
                let mut bytes = [0u8; core::mem::size_of::<$external_repr>()];
                bytes[0..Self::SIZE].clone_from_slice(&self.0[1..]);
                <$external_repr>::from_le_bytes(bytes)
            }

            pub fn as_slice(&self) -> &[u8] {
                &self.0
            }
        }

        impl From<$external_repr> for $name {
            fn from(value: $external_repr) -> Self {
                let mut bytes = [0u8; core::mem::size_of::<$internal_repr>()];
                bytes[0] = Self::ID;
                bytes[1..].clone_from_slice(&value.to_le_bytes()[0..Self::SIZE]);
                $name(bytes)
            }
        }
    };
}

macro_rules! impl_flag {
    ($name:ident, $id:literal) => {
        #[derive(Clone)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $name([u8; 2]);
        impl $name {
            pub fn get(&self) -> bool {
                self.0[1] != 0
            }

            pub fn as_slice(&self) -> &[u8] {
                &self.0
            }
        }

        impl From<bool> for $name {
            fn from(value: bool) -> Self {
                let bytes: [u8; 2] = [$id, value as u8];
                $name(bytes)
            }
        }
    };
}
