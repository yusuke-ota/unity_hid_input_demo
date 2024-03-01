#![no_std]
#![no_main]

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
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usbd_hid::descriptor::{MouseReport, SerializedDescriptor};
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

    let _terminal = terminal::Terminal::new(display);

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

    let mut usb_hid = Some(HIDClass::new(&bus_allocator, MouseReport::desc(), 60));
    let mut usb_bus = Some(
        UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Fake company")
            .product("GamePad via Wio Terminal")
            .serial_number("TEST")
            .build(),
    );

    loop {
        update_hid_report_via_joystick();
        update_hid_report_via_accelerometer(&mut accelerometer);
        if let (Some(usb_dev), Some(hid)) = (usb_bus.as_mut(), usb_hid.as_mut()) {
            usb_dev.poll(&mut [hid]);
        }
    }
}

fn update_hid_report_via_joystick() {
    unsafe {
        V.iter()
            .for_each(|event| todo!("impl hid report update function"));
    }
    unsafe {
        V.clear();
    }
}

fn update_hid_report_via_accelerometer<T: RawAccelerometer<I16x3>>(accelerometer: &mut T) {
    if let Ok(_vec3) = accelerometer.accel_raw() {
        // todo!("impl hid report update function");
    }
}

button_interrupt!(
    BUTTON_CONTROLLER,
    unsafe fn on_button_event(_cs: &CriticalSection, event: ButtonEvent) {
        V.push(event).ok();
    }
);
