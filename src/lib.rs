#![no_std]

use defmt::Format;
use heapless::Vec;

#[derive(Format, Debug)]
pub enum BTHomeError {
    UnknownObjectID,
    BufferFull,
}

pub trait BTHome<const N: usize> {
    const OBJECT_ID: u8;
    fn as_vec(&mut self) -> Result<Vec<u8, N>, BTHomeError>;
    fn length(&self) -> usize;
}

pub mod fields {
    use crate::{BTHome, BTHomeError};
    use defmt::Format;
    use heapless::Vec;

    const FIELD_SIZE: usize = 5;

    macro_rules! impl_bthome_field {
        ($name:ident, $obj_id:literal, $factor:literal, $type:ty, $opt_actual_size:expr) => {
            #[allow(non_camel_case_types)]
            #[derive(Format, Copy, Clone, Debug, PartialEq)]
            pub struct $name {
                value: f32,
            }

            impl From<f32> for $name {
                fn from(value: f32) -> Self {
                    $name { value }
                }
            }

            impl From<$name> for f32 {
                fn from(field: $name) -> f32 {
                    field.value
                }
            }

            impl BTHome<FIELD_SIZE> for $name {
                const OBJECT_ID: u8 = $obj_id;

                fn as_vec(&mut self) -> Result<Vec<u8, FIELD_SIZE>, BTHomeError> {
                    let mut byte_array = Vec::<u8, FIELD_SIZE>::new();
                    byte_array
                        .push(Self::OBJECT_ID)
                        .map_err(|_| BTHomeError::BufferFull)?;

                    let bytes = ((self.value * $factor) as $type).to_le_bytes();
                    byte_array
                        .extend_from_slice(&bytes)
                        .map_err(|_| BTHomeError::BufferFull)?;

                    if let Some(i) = $opt_actual_size {
                        Ok(
                            Vec::<u8, FIELD_SIZE>::from_slice(byte_array.split_at(i + 1).0)
                                .unwrap(),
                        )
                    } else {
                        Ok(byte_array)
                    }
                }

                #[inline]
                fn length(&self) -> usize {
                    if let Some(i) = $opt_actual_size {
                        (i as usize) + 1
                    } else {
                        core::mem::size_of::<$type>() + 1
                    }
                }
            }
        };
        ($name:ident, $obj_id:literal, $factor:literal, $type:ty) => {
            impl_bthome_field!($name, $obj_id, $factor, $type, None::<usize>);
        };
    }

    impl_bthome_field!(Battery_1Per, 0x01, 1.0, u8);
    impl_bthome_field!(CO2_ppm, 0x12, 1.0, u16);
    impl_bthome_field!(Count_1byte, 0x09, 1.0, u8);
    impl_bthome_field!(Count_2bytes, 0x3D, 1.0, u16);
    impl_bthome_field!(Count_4bytes, 0x3E, 1.0, u32);
    impl_bthome_field!(Current_1mA, 0x43, 1000.0, u16);
    impl_bthome_field!(Dewpoint_10mK, 0x08, 100.0, i16);
    impl_bthome_field!(Distance_100mm, 0x41, 10.0, u16);
    impl_bthome_field!(Distance_1mm, 0x40, 1.0, u16);
    impl_bthome_field!(Duration_1ms, 0x42, 1000.0, u32, Some(3));
    impl_bthome_field!(Energy_1Wh, 0x0A, 1000.0, u32, Some(3));
    impl_bthome_field!(Humidity_10mPer, 0x03, 100.0, u16);
    impl_bthome_field!(Humidity_1Per, 0x2E, 1.0, u8);
    impl_bthome_field!(Illuminance_10mLux, 0x05, 100.0, u32, Some(3));
    impl_bthome_field!(Mass_10g, 0x06, 100.0, u16);
    impl_bthome_field!(Mass_10mLb, 0x07, 100.0, u16);
    impl_bthome_field!(Moisture_10mPer, 0x14, 100.0, u16);
    impl_bthome_field!(Moisture_1Per, 0x2F, 1.0, u8);
    impl_bthome_field!(PM10_1ugmc, 0x0E, 1.0, u16);
    impl_bthome_field!(PM25_1ugmc, 0x0D, 1.0, u16);
    impl_bthome_field!(Power_10mW, 0x0B, 100.0, u32, Some(3));
    impl_bthome_field!(Pressure_1Pa, 0x04, 100.0, u32, Some(3));
    impl_bthome_field!(Rotation_100mDeg, 0x3F, 10.0, i16);
    impl_bthome_field!(Speed_10mms, 0x44, 100.0, u16);
    impl_bthome_field!(Temperature_100mK, 0x45, 10.0, i16);
    impl_bthome_field!(Temperature_10mK, 0x02, 100.0, i16);
    impl_bthome_field!(TVOC_1ugmc, 0x13, 1.0, u16);
    impl_bthome_field!(UVIndex_100milli, 0x46, 10.0, u8);
    impl_bthome_field!(Voltage_100mV, 0x4A, 10.0, u16);
    impl_bthome_field!(Voltage_1mV, 0x0C, 1000.0, u16);
    impl_bthome_field!(Volume_100mL, 0x47, 10.0, u16);
    impl_bthome_field!(Volume_1mL, 0x48, 1.0, u16);
    impl_bthome_field!(VolumeFlowRate_1LpH, 0x49, 1000.0, u16);

    #[cfg(test)]
    mod tests {
        extern crate std;
        use super::*;

        #[test]
        fn test_cast_from_f32() {
            let expected = Battery_1Per { value: 10.2 };
            let field: Battery_1Per = 10.2.into();
            assert_eq!(field, expected);

            let expected: f32 = 10.2;
            let casted: f32 = field.into();
            assert_eq!(casted, expected);
        }

        #[test]
        fn test_as_vec() {
            let mut field: Moisture_10mPer = 33.5f32.into();
            let expected = std::vec![0x14u8, 0x16u8, 0x0Du8];
            let arr = field.as_vec().unwrap();
            assert_eq!(expected, arr.as_slice());

            // Positive temperature
            let mut field: Temperature_100mK = 27.3f32.into();
            let expected = std::vec![0x45u8, 0x11u8, 0x01u8];
            let arr = field.as_vec().unwrap();
            assert_eq!(expected, arr.as_slice());

            // Negative temperature [0.1°C]
            let mut field: Temperature_100mK = (-25.2f32).into();
            let expected = std::vec![0x45u8, 0x04u8, 0xFFu8];
            let arr = field.as_vec().unwrap();
            assert_eq!(expected, arr.as_slice());

            // Negative temperature [0.01°C]
            let mut field: Temperature_10mK = (-25.06f32).into();
            let expected = std::vec![0x02u8, 0x36u8, 0xF6u8];
            let arr = field.as_vec().unwrap();
            assert_eq!(expected, arr.as_slice());

            // 3-byte long fields
            let mut field: Illuminance_10mLux = 13460.67.into();
            let expected: std::vec::Vec<u8> = std::vec![0x05, 0x13, 0x8A, 0x14];
            let arr = field.as_vec().unwrap();
            assert_eq!(expected, arr.as_slice());
        }

        #[test]
        fn test_length() {
            // 1 byte long + 1 byte ID
            let field: Battery_1Per = 69.0.into();
            let expected: usize = 2;
            assert_eq!(field.length(), expected);

            // 2 bytes long + 1 byte ID
            let field: Distance_100mm = 10.0.into();
            let expected: usize = 3;
            assert_eq!(field.length(), expected);

            // 3 bytes long + 1 byte ID
            let field: Illuminance_10mLux = 1234.0.into();
            let expected: usize = 4;
            assert_eq!(field.length(), expected);

            // 4 bytes long + 1 byte ID
            let field: Count_4bytes = 12345678.0.into();
            let expected: usize = 5;
            assert_eq!(field.length(), expected);
        }
    }
}
pub mod flags {
    use crate::{BTHome, BTHomeError};
    use defmt::Format;
    use heapless::Vec;

    const FLAG_SIZE: usize = 2;

    macro_rules! impl_bthome_flag {
        ($name:ident, $obj_id:literal) => {
            #[allow(non_camel_case_types)]
            #[derive(Format, Debug, Copy, Clone, PartialEq)]
            pub struct $name {
                value: bool,
            }

            impl BTHome<FLAG_SIZE> for $name {
                const OBJECT_ID: u8 = $obj_id;

                fn as_vec(&mut self) -> Result<heapless::Vec<u8, FLAG_SIZE>, crate::BTHomeError> {
                    let mut byte_array = Vec::<u8, FLAG_SIZE>::new();
                    byte_array
                        .push(Self::OBJECT_ID)
                        .map_err(|_| BTHomeError::BufferFull)?;
                    byte_array
                        .push(self.value.into())
                        .map_err(|_| BTHomeError::BufferFull)?;

                    Ok(byte_array)
                }

                fn length(&self) -> usize {
                    2
                }
            }

            impl From<bool> for $name {
                fn from(value: bool) -> Self {
                    $name { value }
                }
            }

            impl From<$name> for bool {
                fn from(field: $name) -> bool {
                    field.value
                }
            }
        };
    }

    impl_bthome_flag!(Battery, 0x15);
    impl_bthome_flag!(Battery_Charging, 0x16);
    impl_bthome_flag!(Carbon_Monoxide, 0x17);
    impl_bthome_flag!(Cold, 0x18);
    impl_bthome_flag!(Connectivity, 0x19);
    impl_bthome_flag!(Door, 0x1A);
    impl_bthome_flag!(Garage_Door, 0x1B);
    impl_bthome_flag!(Gas, 0x1C);
    impl_bthome_flag!(Generic_Boolean, 0x0F);
    impl_bthome_flag!(Heat, 0x1D);
    impl_bthome_flag!(Light, 0x1E);
    impl_bthome_flag!(Lock, 0x1F);
    impl_bthome_flag!(Moisture, 0x20);
    impl_bthome_flag!(Motion, 0x21);
    impl_bthome_flag!(Moving, 0x22);
    impl_bthome_flag!(Occupancy, 0x23);
    impl_bthome_flag!(Opening, 0x11);
    impl_bthome_flag!(Plugged_In, 0x24);
    impl_bthome_flag!(Power, 0x10);
    impl_bthome_flag!(Presence, 0x25);
    impl_bthome_flag!(Problem, 0x26);
    impl_bthome_flag!(Running, 0x27);
    impl_bthome_flag!(Safety, 0x28);
    impl_bthome_flag!(Smoke, 0x29);
    impl_bthome_flag!(Sound, 0x2A);
    impl_bthome_flag!(Tamper, 0x2B);
    impl_bthome_flag!(Vibration, 0x2C);
    impl_bthome_flag!(Window, 0x2D);

    #[cfg(test)]
    mod tests {
        extern crate std;
        use super::*;

        #[test]
        fn test_cast_from_bool() {
            let expected = Battery_Charging { value: true };
            let flag: Battery_Charging = true.into();
            assert_eq!(flag, expected);

            let expected: bool = true;
            let casted: bool = flag.into();
            assert_eq!(casted, expected);
        }

        #[test]
        fn test_as_vec() {
            // Battery charging flag.
            let mut flag: Battery_Charging = true.into();
            let expected = std::vec![0x16u8, 0x01];
            let arr = flag.as_vec().unwrap();
            assert_eq!(expected, arr.as_slice());

            // Plugged in flag.
            let mut flag: Plugged_In = false.into();
            let expected = std::vec![0x24u8, 0x00u8];
            let arr = flag.as_vec().unwrap();
            assert_eq!(expected, arr.as_slice());
        }

        #[test]
        fn test_length() {
            // 1 byte long + 1 byte ID
            let flag: Generic_Boolean = false.into();
            let expected: usize = 2;
            assert_eq!(flag.length(), expected);
        }
    }
}
