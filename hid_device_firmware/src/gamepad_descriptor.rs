use usbd_hid::descriptor::generator_prelude::*;

// This code is inspired by <https://github.com/twitchyliquid64/usbd-hid/issues/61>.
/// GAMEPAD describes a report and its companion descriptor than can be used
/// to send GAMEPAD button presses to a host.
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = GAMEPAD) = {
        (collection = APPLICATION, usage = POINTER) = {
            (usage = X,) = {
                #[item_settings data,variable,absolute] x=input;
            };
            (usage = Y,) = {
                #[item_settings data,variable,absolute] y=input;
            };
            (usage = Z,) = {
                #[item_settings data,variable,absolute] z=input;
            };
            (usage = 0x33,) = {
                #[item_settings data,variable,absolute] rx=input;
            };
            (usage = 0x34,) = {
                #[item_settings data,variable,absolute] ry=input;
            };
            (usage = 0x35,) = {
                #[item_settings data,variable,absolute] rz=input;
            };
        };
        (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] buttons=input;
        }
    }
)]
#[derive(Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct GamePadReport {
    pub buttons: u8,
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub rx: i16,
    pub ry: i16,
    pub rz: i16,
}
