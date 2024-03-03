use usbd_hid::descriptor::generator_prelude::*;

// TODO: D-Pad対応
// (collection = PHYSICAL, usage_page = GENERIC_DESKTOP) = {
//     (usage = X,) = {
//         # [item_settings data, variable, relative] x = input;
//     };
//     (usage = Y,) = {
//         # [item_settings data, variable, relative] y = input;
//     };
// };

// #[gen_hid_descriptor(
//     (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = GAMEPAD) = {
//         (collection = APPLICATION, usage = POINTER) = {
//             (collection = PHYSICAL, usage_page = GENERIC_DESKTOP) = {
//                 (usage = X,) = {
//                     # [packed_bits 32] # [item_settings data, variable, absolute] x = input;
//                 };
//                 (usage = Y,) = {
//                     # [packed_bits 32] # [item_settings data, variable, absolute] y = input;
//                 };
//                 (usage = Z,) = {
//                     # [packed_bits 32] # [item_settings data, variable, absolute] z = input;
//                 };
//             };
//             (usage_page = BUTTON, usage_min = 0x01, usage_max = 0x08) = {
//                 # [packed_bits 8] # [item_settings data, variable, absolute] buttons = input;
//             };
//         };
//     }
// )]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct GamepadReport {
    pub buttons: u8,
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl SerializedDescriptor for GamepadReport {
    #[rustfmt::skip]
    fn desc() -> &'static [u8] {
        &[
            0x05, 0x01,         // Usage Page (Generic Desktop Controls)
            0x09, 0x05,         // Usage (Game Pad)
            0xa1, 0x01,         // Collection (Application)
            0x09, 0x01,         //   Usage (Pointer)
            0xa1, 0x00,         //   Collection (Physical)
            0x05, 0x01,         //     Usage Page (Generic Desktop Controls)
            0x16, 0xff, 0xff,   //     Logical Minimum (-1)
            0x25, 0x01,         //     Logical Maximum (1)
            0x09, 0x30,         //     Usage (X)
            0x09, 0x31,         //     Usage (Y)
            0x09, 0x32,         //     Usage (Z)
            0x75, 0x20,         //     Report Size (32)
            0x95, 0x03,         //     Report Count (3)
            0x81, 0x02,         //     Input (Data, Variable, Absolute)
            0xc0,               //   End Collection
            0x05, 0x09,         //   Usage Page (Button)
            0x19, 0x00,         //   Logical Minimum (0)
            0x29, 0x08,         //   Logical Maximum (8)
            0x95, 0x08,         //   Report Count (8)
            0x81, 0x02,         //   Input (Data, Variable, Absolute)
            0xc0                // End Collection
        ]
    }
}
impl Serialize for GamepadReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        {
            let mut s = serializer.serialize_tuple(4)?;
            let high_byte = (self.x >> 16) as u16;
            let low_byte = self.x as u16;
            s.serialize_element(&high_byte)?;
            s.serialize_element(&low_byte)?;
            let high_byte = (self.y >> 16) as u16;
            let low_byte = self.y as u16;
            s.serialize_element(&high_byte)?;
            s.serialize_element(&low_byte)?;
            let high_byte = (self.z >> 16) as u16;
            let low_byte = self.z as u16;
            s.serialize_element(&high_byte)?;
            s.serialize_element(&low_byte)?;
            s.serialize_element(&self.buttons)?;
            s.end()
        }
    }
}

impl AsInputReport for GamepadReport {}
