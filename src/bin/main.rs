#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embedded_hal::delay::DelayNs;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Pull},
    main,
    uart::{Config, Uart},
};
use esp_println as _;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let mut delay = Delay::new();

    let mut uart = Uart::new(peripherals.UART0, Config::default()).unwrap();

    let button1 = Input::new(
        peripherals.GPIO5,
        InputConfig::default().with_pull(Pull::Up),
    );
    let button2 = Input::new(
        peripherals.GPIO14,
        InputConfig::default().with_pull(Pull::Up),
    );

    loop {
        let low1 = button1.is_low();
        let low2 = button2.is_low();
        let data = if low1 && low2 {
            "13\r\n"
        } else if low1 && !low2 {
            "12\r\n"
        } else if !low1 && low2 {
            "03\r\n"
        } else {
            "02\r\n"
        };
        uart.write(data.as_bytes()).unwrap();
        uart.flush().unwrap();
        delay.delay_ms(2_u32);
    }
}
