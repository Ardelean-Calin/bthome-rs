#![no_std]

#[macro_use]
pub mod macros;

#[rustfmt::skip]
#[allow(non_camel_case_types)]
pub mod fields {
    //          Field Name              ID    Internal Representation   External Representation
    impl_field!(PacketID,               0x00, [u8; 2],                  u8);
    impl_field!(Battery_1Per,           0x01, [u8; 2],                  u8);
    impl_field!(CO2_ppm,                0x12, [u8; 3],                  u16);
    impl_field!(Count_1byte,            0x09, [u8; 2],                  u8);
    impl_field!(Count_2bytes,           0x3D, [u8; 3],                  u16);
    impl_field!(Count_4bytes,           0x3E, [u8; 5],                  u32);
    impl_field!(Current_1mA,            0x43, [u8; 3],                  u16);
    impl_field!(Dewpoint_10mK,          0x08, [u8; 3],                  i16);
    impl_field!(Distance_100mm,         0x41, [u8; 3],                  u16);
    impl_field!(Distance_1mm,           0x40, [u8; 3],                  u16);
    impl_field!(Duration_1ms,           0x42, [u8; 4],                  u32);
    impl_field!(Energy_1Wh,             0x0A, [u8; 4],                  u32);
    impl_field!(Humidity_10mPer,        0x03, [u8; 3],                  u16);
    impl_field!(Humidity_1Per,          0x2E, [u8; 2],                  u8);
    impl_field!(Illuminance_10mLux,     0x05, [u8; 4],                  u32);
    impl_field!(Mass_10g,               0x06, [u8; 3],                  u16);
    impl_field!(Mass_10mLb,             0x07, [u8; 3],                  u16);
    impl_field!(Moisture_10mPer,        0x14, [u8; 3],                  u16);
    impl_field!(Moisture_1Per,          0x2F, [u8; 2],                  u8);
    impl_field!(PM10_1ugmc,             0x0E, [u8; 3],                  u16);
    impl_field!(PM25_1ugmc,             0x0D, [u8; 3],                  u16);
    impl_field!(Power_10mW,             0x0B, [u8; 4],                  u32);
    impl_field!(Pressure_1Pa,           0x04, [u8; 4],                  u32);
    impl_field!(Rotation_100mDeg,       0x3F, [u8; 3],                  i16);
    impl_field!(Speed_10mms,            0x44, [u8; 3],                  u16);
    impl_field!(Temperature_100mK,      0x45, [u8; 3],                  i16);
    impl_field!(Temperature_10mK,       0x02, [u8; 3],                  i16);
    impl_field!(TVOC_1ugmc,             0x13, [u8; 3],                  u16);
    impl_field!(UVIndex_100milli,       0x46, [u8; 2],                  u8);
    impl_field!(Voltage_100mV,          0x4A, [u8; 3],                  u16);
    impl_field!(Voltage_1mV,            0x0C, [u8; 3],                  u16);
    impl_field!(Volume_100mL,           0x47, [u8; 3],                  u16);
    impl_field!(Volume_1mL,             0x48, [u8; 3],                  u16);
    impl_field!(VolumeFlowRate_1LpH,    0x49, [u8; 3],                  u16);


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_full() {
            // A simple 1-byte field
            let field = PacketID::from(0x23);
            assert_eq!(field.0, [0x00u8, 0x23u8]);
            assert_eq!(field.get(), 0x23u8);

            let field =  Battery_1Per::from(55);
            assert_eq!(field.0, [0x01, 55u8]);
            assert_eq!(field.get(), 55u8);

            // More complex, 2-byte signed int field
            // Convert positive temperature to bytes
            let temperature = 27.3f32;
            let temp_100mk = temperature * 10f32;
            let temp_field = Temperature_100mK::from(temp_100mk as i16);
            assert_eq!(temp_field.0, [0x45u8, 0x11u8, 0x01u8]);
            // And back
            let temp_100mk: f32 = temp_field.get() as f32;
            assert_eq!(temp_100mk, 273.0f32);

            // Convert negative temperature to bytes
            let temperature = -25.2f32;
            let temp_100mk = temperature * 10f32;
            let temp_field = Temperature_100mK::from(temp_100mk as i16);
            assert_eq!(temp_field.0, [0x45u8, 0x04u8, 0xFFu8]);
            // And back
            let temp_100mk: f32 = temp_field.get() as f32;
            assert_eq!(temp_100mk, -252.0f32);

            // A 3-byte field
            let lux = 13460.67f32;
            let lux_10mlx = (lux * 100f32) as u32;
            let field = Illuminance_10mLux::from(lux_10mlx);
            assert_eq!(field.0, [0x05, 0x13, 0x8A, 0x14]);
            let lux_10mlx = field.get() as f32;
            assert_eq!(lux_10mlx, 1346067f32);

        }

        #[test]
        fn test_length() {
            assert_eq!(core::mem::size_of::<Battery_1Per>(), 2);
            assert_eq!(core::mem::size_of::<Distance_100mm>(), 3);
            assert_eq!(core::mem::size_of::<Illuminance_10mLux>(), 4);
            assert_eq!(core::mem::size_of::<Count_4bytes>(), 5);
        }
    }
}

#[rustfmt::skip]
#[allow(non_camel_case_types)]
pub mod flags {
    //         Flag Name            ID 
    impl_flag!(Battery,             0x15);
    impl_flag!(Battery_Charging,    0x16);
    impl_flag!(Carbon_Monoxide,     0x17);
    impl_flag!(Cold,                0x18);
    impl_flag!(Connectivity,        0x19);
    impl_flag!(Door,                0x1A);
    impl_flag!(Garage_Door,         0x1B);
    impl_flag!(Gas,                 0x1C);
    impl_flag!(Generic_Boolean,     0x0F);
    impl_flag!(Heat,                0x1D);
    impl_flag!(Light,               0x1E);
    impl_flag!(Lock,                0x1F);
    impl_flag!(Moisture,            0x20);
    impl_flag!(Motion,              0x21);
    impl_flag!(Moving,              0x22);
    impl_flag!(Occupancy,           0x23);
    impl_flag!(Opening,             0x11);
    impl_flag!(Plugged_In,          0x24);
    impl_flag!(Power,               0x10);
    impl_flag!(Presence,            0x25);
    impl_flag!(Problem,             0x26);
    impl_flag!(Running,             0x27);
    impl_flag!(Safety,              0x28);
    impl_flag!(Smoke,               0x29);
    impl_flag!(Sound,               0x2A);
    impl_flag!(Tamper,              0x2B);
    impl_flag!(Vibration,           0x2C);
    impl_flag!(Window,              0x2D);

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_full() {
            let flag = Battery_Charging::from(true);
            assert_eq!(flag.0, [0x16u8, 0x01]);
            assert_eq!(flag.get(), true);

            let flag = Plugged_In::from(false);
            assert_eq!(flag.0, [0x24u8, 0x00u8]);
            assert_eq!(flag.get(), false);
        }

        #[test]
        fn test_length() {
            assert_eq!(core::mem::size_of::<Battery_Charging>(), 2);
            assert_eq!(core::mem::size_of::<Plugged_In>(), 2);
            assert_eq!(core::mem::size_of::<Problem>(), 2);
            assert_eq!(core::mem::size_of::<Presence>(), 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn test_bthome_ad() {
        build_bthome_ad!(
            struct MyAD {
                air_temperature: fields::Temperature_100mK,
                air_humidity: fields::Humidity_1Per,
                battery_charging: flags::Battery_Charging,
            }
        );

        let my_ad = MyAD::default()
            .air_humidity(50u8.into()) // 50%
            .air_temperature(255i16.into()) // 25.5 degrees C
            .battery_charging(true.into()); // A flag indicating that the battery is charging

        let expected = std::vec![2, 1, 6, 11, 22, 210, 252, 64, 69, 255, 0, 46, 50, 22, 1];
        assert_eq!(expected.as_slice(), my_ad.as_vec().as_slice());
    }
}
