#![no_std]
#![no_main]

mod terminal;

use cortex_m::interrupt::{free as disable_interrupts, CriticalSection};

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

// define panic handler.
use panic_halt as _;

static mut BUTTON_CONTROLLER: Option<bsp::ButtonController> = None;
static mut V: Vec<ButtonEvent, 8> = Vec::new();

// This implementation is inspired by
// - [the `button` example in the `wio_terminal` crate](https://github.com/atsamd-rs/atsamd/blob/master/boards/wio_terminal/examples/button.rs)
// - [the `orientation` example in the `wio_terminal` crate](https://github.com/atsamd-rs/atsamd/blob/master/boards/wio_terminal/examples/orientation.rs)

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let mut core = pac::CorePeripherals::take().unwrap();

    let mut clocks = hal::clock::GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
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
            &mut peripherals.MCLK,
            58.MHz(),
            &mut delay,
        )
        .unwrap();

    let _terminal = terminal::Terminal::new(display);

    let button_controller = sets
        .buttons
        .init(peripherals.EIC, &mut clocks, &mut peripherals.MCLK);
    let nvic = &mut core.NVIC;
    disable_interrupts(|_| unsafe {
        button_controller.enable(nvic);
        BUTTON_CONTROLLER = Some(button_controller);
    });

    let mut accelerometer: Lis3dh<_> =
        sets.accelerometer
            .init(&mut clocks, peripherals.SERCOM4, &mut peripherals.MCLK);

    loop {
        update_hid_report_via_joystick();
        update_hid_report_via_accelerometer(&mut accelerometer);
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
