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

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = GAMEPAD) = {
        (collection = APPLICATION, usage = POINTER) = {
            (collection = PHYSICAL, usage_page = GENERIC_DESKTOP) = {
                (usage = X,) = {
                    # [item_settings data, variable, relative] x = input;
                };
                (usage = Y,) = {
                    # [item_settings data, variable, relative] y = input;
                };
                (usage = Z,) = {
                    # [item_settings data, variable, relative] z = input;
                };
            };
            (usage_page = BUTTON, usage_min = 0x01, usage_max = 0x08) = {
                # [packed_bits 8] # [item_settings data, variable, absolute] buttons = input;
            };
        };
    }
)]
pub struct GamepadReport {
    pub buttons: u8,
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
