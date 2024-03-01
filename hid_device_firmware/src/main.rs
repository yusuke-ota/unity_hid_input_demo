#![no_std]
#![no_main]

mod gamepad_descriptor;
mod terminal;

use cortex_m::interrupt::{free as disable_interrupts, CriticalSection};

// define panic handler.
use panic_halt as _;

use bsp::hal;
use bsp::pac;
use wio_terminal as bsp;

use bsp::entry;
use bsp::prelude::RawAccelerometer;
use bsp::{button_interrupt, ButtonEvent};
use hal::fugit::RateExtU32;
use pac::interrupt;

use heapless::Vec;
use lis3dh::accelerometer::vector::I16x3;
use lis3dh::Lis3dh;

// usb
use gamepad_descriptor::GamePadReport;
use terminal::Terminal;
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usbd_hid::descriptor::SerializedDescriptor;
use usbd_hid::hid_class::HIDClass;

static mut BUTTON_CONTROLLER: Option<bsp::ButtonController> = None;
static mut V: Vec<ButtonEvent, 8> = Vec::new();

// This implementation is inspired by
// - [the `button` example in the `wio_terminal` crate](https://github.com/atsamd-rs/atsamd/blob/master/boards/wio_terminal/examples/button.rs)
// - [the `orientation` example in the `wio_terminal` crate](https://github.com/atsamd-rs/atsamd/blob/master/boards/wio_terminal/examples/orientation.rs)
// - [the `twitching_usb_mouse` example in the `wio_terminal` crate](https://github.com/atsamd-rs/atsamd/blob/master/boards/itsybitsy_m0/examples/twitching_usb_mouse.rs)

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let mut core = pac::CorePeripherals::take().unwrap();
    let main_clock = &mut peripherals.MCLK;

    let mut clocks = hal::clock::GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        main_clock,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let sets = bsp::Pins::new(peripherals.PORT).split();

    let (display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            main_clock,
            58.MHz(),
            &mut delay,
        )
        .unwrap();

    let mut terminal = terminal::Terminal::new(display);

    let button_controller = sets.buttons.init(peripherals.EIC, &mut clocks, main_clock);
    let nvic = &mut core.NVIC;
    disable_interrupts(|_| unsafe {
        button_controller.enable(nvic);
        BUTTON_CONTROLLER = Some(button_controller);
    });

    let mut accelerometer: Lis3dh<_> =
        sets.accelerometer
            .init(&mut clocks, peripherals.SERCOM4, main_clock);

    let bus_allocator = sets
        .usb
        .usb_allocator(peripherals.USB, &mut clocks, main_clock);

    let mut usb_hid = HIDClass::new(&bus_allocator, GamePadReport::desc(), 60);
    let mut usb_bus = UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("GamePad via Wio Terminal")
        .serial_number("TEST")
        .build();

    let mut display_str = heapless::String::<64>::new();
    let mut report = GamePadReport::default();

    loop {
        update_hid_report_via_joystick(&mut report);
        update_hid_report_via_accelerometer(&mut accelerometer, &mut report);
        display_report(&mut terminal, &mut display_str, &report);
        usb_hid.push_input(&report).ok();
        usb_bus.poll(&mut [&mut usb_hid]);
    }
}

fn update_hid_report_via_joystick(report: &mut GamePadReport) {
    unsafe {
        V.iter().for_each(|event| {
            let (button, down) = (&event.button, &event.down);
            match button {
                bsp::Button::Up => {
                    report.y = if *down { 127 } else { 0 };
                }
                bsp::Button::Down => {
                    report.y = if *down { -127 } else { 0 };
                }
                bsp::Button::Left => {
                    report.x = if *down { -127 } else { 0 };
                }
                bsp::Button::Right => {
                    report.x = if *down { 127 } else { 0 };
                }
                bsp::Button::Click => {
                    report.z = if *down { 127 } else { 0 };
                }
                _ => {}
            }
        });
    }
    unsafe {
        V.clear();
    }
}

fn update_hid_report_via_accelerometer<T: RawAccelerometer<I16x3>>(
    accelerometer: &mut T,
    report: &mut GamePadReport,
) {
    if let Ok(vec3) = accelerometer.accel_raw() {
        report.rx = vec3.x;
        report.ry = vec3.y;
        report.rz = vec3.z;
    }
}

fn display_report(
    terminal: &mut Terminal,
    display_str: &mut heapless::String<64>,
    report: &GamePadReport,
) {
    display_str.clear();
    let x: heapless::String<4> = heapless::String::try_from(report.x).unwrap();
    let y: heapless::String<4> = heapless::String::try_from(report.y).unwrap();
    let z: heapless::String<4> = heapless::String::try_from(report.z).unwrap();
    let rx: heapless::String<6> = heapless::String::try_from(report.rx).unwrap();
    let ry: heapless::String<6> = heapless::String::try_from(report.ry).unwrap();
    let rz: heapless::String<6> = heapless::String::try_from(report.rz).unwrap();
    display_str.push_str("x: ").unwrap(); // cap(64) > len( 0 + 3 =  3)
    display_str.push_str(x.as_str()).unwrap(); // cap(64) > len( 3 + 4 =  7)
    display_str.push_str("\ny: ").unwrap(); // cap(64) > len( 7 + 5 = 12)
    display_str.push_str(y.as_str()).unwrap(); // cap(64) > len(12 + 4 = 16)
    display_str.push_str("\nz: ").unwrap(); // cap(64) > len(16 + 5 = 21)
    display_str.push_str(z.as_str()).unwrap(); // cap(64) > len(21 + 4 = 25)
    display_str.push_str("\nrx: ").unwrap(); // cap(64) > len(25 + 6 = 31)
    display_str.push_str(rx.as_str()).unwrap(); // cap(64) > len(31 + 6 = 37)
    display_str.push_str("\nry: ").unwrap(); // cap(64) > len(37 + 6 = 43)
    display_str.push_str(ry.as_str()).unwrap(); // cap(64) > len(43 + 6 = 49)
    display_str.push_str("\nrz: ").unwrap(); // cap(64) > len(49 + 6 = 55)
    display_str.push_str(rz.as_str()).unwrap(); // cap(64) > len(55 + 6 = 61)
    terminal.write_str(display_str.as_str());
}

button_interrupt!(
    BUTTON_CONTROLLER,
    unsafe fn on_button_event(_cs: &CriticalSection, event: ButtonEvent) {
        V.push(event).ok();
    }
);
