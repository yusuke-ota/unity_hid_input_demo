#![no_std]
#![no_main]

mod gamepad_descriptor;
mod terminal;
// use cortex_m::interrupt::{free as disable_interrupts, CriticalSection};

// define panic handler.
use panic_halt as _;

use bsp::hal;
use bsp::pac;
use wio_terminal as bsp;

use bsp::entry;
use bsp::prelude::{RawAccelerometer, _embedded_hal_timer_CountDown};
// use bsp::{button_interrupt, ButtonEvent};
use hal::fugit::{ExtU32, RateExtU32};
use hal::timer::TimerCounter;
// use pac::interrupt;

use lis3dh::accelerometer::vector::I16x3;
use lis3dh::{Lis3dh, Mode, Range};

// usb
use gamepad_descriptor::GamepadReport;
use usb_device::device::UsbDeviceState;
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usbd_hid::descriptor::SerializedDescriptor;
use usbd_hid::hid_class::HIDClass;

// static mut BUTTON_CONTROLLER: Option<bsp::ButtonController> = None;
static mut REPORT: GamepadReport = GamepadReport {
    buttons: 0,
    x: 0,
    y: 0,
    z: 0,
};

// This implementation is inspired by
// - [the `button` example in the `wio_terminal` crate](https://github.com/atsamd-rs/atsamd/blob/master/boards/wio_terminal/examples/button.rs)
// - [the `orientation` example in the `wio_terminal` crate](https://github.com/atsamd-rs/atsamd/blob/master/boards/wio_terminal/examples/orientation.rs)
// - [the `twitching_usb_mouse` example in the `wio_terminal` crate](https://github.com/atsamd-rs/atsamd/blob/master/boards/itsybitsy_m0/examples/twitching_usb_mouse.rs)

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let main_clock = &mut peripherals.MCLK;

    let mut clocks = hal::clock::GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        main_clock,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let sets = bsp::Pins::new(peripherals.PORT).split();

    let bus_allocator = sets
        .usb
        .usb_allocator(peripherals.USB, &mut clocks, main_clock);
    let mut usb_hid = HIDClass::new(&bus_allocator, GamepadReport::desc(), 10);
    let mut usb_dev = UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x1209, 0x0001))
        .manufacturer("Fake company")
        .product("Wio Terminal GamePad")
        .serial_number("TEST")
        .build();

    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let (display, _) = sets
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

    // let button_controller = sets.buttons.init(peripherals.EIC, &mut clocks, main_clock);
    // let nvic = &mut core.NVIC;
    // disable_interrupts(|_| unsafe {
    //     button_controller.enable(nvic);
    //     BUTTON_CONTROLLER = Some(button_controller);
    // });

    let mut accelerometer: Lis3dh<_> =
        sets.accelerometer
            .init(&mut clocks, peripherals.SERCOM4, main_clock);

    let gclk0 = clocks.gclk0();
    let mut timer = TimerCounter::tc3_(
        &clocks.tc2_tc3(&gclk0).unwrap(),
        peripherals.TC3,
        main_clock,
    );
    timer.start(10.millis());
    let mut state = usb_dev.state();
    terminal.write_str("Wio Terminal GamePad\n");
    display_usb_state(&mut terminal, state);

    let mode = accelerometer.get_mode();
    match mode {
        Ok(Mode::LowPower) => terminal.write_str("Mode: LowPower\n"),
        Ok(Mode::Normal) => terminal.write_str("Mode: Normal\n"),
        Ok(Mode::HighResolution) => terminal.write_str("Mode: HighResolution\n"),
        _ => {}
    }

    let range = accelerometer.get_range();
    match range {
        Ok(Range::G2) => terminal.write_str("Range: 2G\n"),
        Ok(Range::G4) => terminal.write_str("Range: 4G\n"),
        Ok(Range::G8) => terminal.write_str("Range: 8G\n"),
        Ok(Range::G16) => terminal.write_str("Range: 16G\n"),
        _ => {}
    }

    loop {
        if timer.wait().is_ok() {
            let new_state = usb_dev.state();
            if state != new_state {
                display_usb_state(&mut terminal, new_state);
                state = new_state;
            }
            if new_state == UsbDeviceState::Configured || new_state == UsbDeviceState::Suspend {
                unsafe {
                    update_hid_report_via_accelerometer(&mut accelerometer, &mut REPORT);
                    usb_hid.push_input(&REPORT).ok();
                    display_report(&mut terminal, &REPORT)
                }
            }
        }

        if usb_dev.poll(&mut [&mut usb_hid]) {}
    }
}

fn update_hid_report_via_accelerometer<T: RawAccelerometer<I16x3>>(
    accelerometer: &mut T,
    report: &mut GamepadReport,
) {
    if let Ok(vec3) = accelerometer.accel_raw() {
        report.x = vec3.x;
        report.y = vec3.y;
        report.z = vec3.z;
    }
}

fn display_usb_state(terminal: &mut terminal::Terminal, state: UsbDeviceState) {
    terminal.write_str("USB state: ");
    terminal.write_str(match state {
        UsbDeviceState::Default => "Default",
        UsbDeviceState::Addressed => "Addressed",
        UsbDeviceState::Configured => "Configured",
        UsbDeviceState::Suspend => "Suspended",
    });
    terminal.write_str("\n");
}

fn display_report(terminal: &mut terminal::Terminal, report: &GamepadReport) {
    terminal.write_str("x: ");
    let x: heapless::String<6> = heapless::String::try_from(report.x).unwrap();
    terminal.write_str(x.as_str());
    terminal.write_str(", y: ");
    let y: heapless::String<6> = heapless::String::try_from(report.y).unwrap();
    terminal.write_str(y.as_str());
    terminal.write_str(", y: ");
    let y: heapless::String<6> = heapless::String::try_from(report.z).unwrap();
    terminal.write_str(y.as_str());
    terminal.write_str(", button: ");
    let buttons: heapless::String<4> = heapless::String::try_from(report.buttons).unwrap();
    terminal.write_str(buttons.as_str());
    terminal.write_str("\n");
}

// button_interrupt!(
//     BUTTON_CONTROLLER,
//     unsafe fn on_button_event(_cs: &CriticalSection, event: ButtonEvent) {
//         let (button, down) = (&event.button, &event.down);
//         match button {
//             bsp::Button::Up => {
//                 REPORT.y = if *down { 127 } else { 0 };
//             }
//             bsp::Button::Down => {
//                 REPORT.y = if *down { -127 } else { 0 };
//             }
//             bsp::Button::Left => {
//                 REPORT.x = if *down { -127 } else { 0 };
//             }
//             bsp::Button::Right => {
//                 REPORT.x = if *down { 127 } else { 0 };
//             }
//             bsp::Button::Click => {
//                 REPORT.buttons = if *down { 1 } else { 0 };
//             }
//             _ => {}
//         }
//     }
// );
