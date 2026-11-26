use esp_idf_svc::hal::{
    delay::FreeRtos,
    gpio::{AnyInputPin, PinDriver, Pull},
    peripherals::Peripherals,
    spi::{SpiDeviceDriver, SpiDriver, SpiDriverConfig, config},
    units::Hertz,
};

use mipidsi::{
    Builder,
    interface::SpiInterface,
    models::ILI9342CRgb565,
    options::{ColorInversion, ColorOrder}
};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    text::Text,
    pixelcolor::Rgb565,
    prelude::*
};

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().expect("Failed to take peripherals");
    // Basic's A button has an external pull-up; pressing it drives GPIO39 low.
    let button = PinDriver::input(peripherals.pins.gpio39, Pull::Floating)
        .expect("Failed to configure button A");
    let mut was_pressed = false;

    let spi = SpiDriver::new(
        peripherals.spi2,
        peripherals.pins.gpio18,
        peripherals.pins.gpio23,
        None::<AnyInputPin>,
        &SpiDriverConfig::default(),
    )
    .expect("Failed to initialize SPI");

    let display_config = config::Config::new()
        .baudrate(Hertz(10_000_000));

    let display_spi = SpiDeviceDriver::new(
        spi,
        Some(peripherals.pins.gpio14),
        &display_config,
    )
    .expect("Failed to configure display SPI");

    let dc = PinDriver::output(peripherals.pins.gpio27)
        .expect("Failed to configure display DC");

    let mut display_buffer = [0_u8; 512];
    let interface = SpiInterface::new(display_spi, dc, &mut display_buffer);

    let reset = PinDriver::output(peripherals.pins.gpio33)
        .expect("Failed to configure display reset");

    let mut display = Builder::new(ILI9342CRgb565, interface)
        .reset_pin(reset)
        .invert_colors(ColorInversion::Inverted)
        .color_order(ColorOrder::Bgr)
        .init(&mut FreeRtos)
        .expect("Failed to initialize display");

    display.clear(Rgb565::BLUE)
        .expect("Failed to clear display");

    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);

    Text::new("Hello, moqumoqu!", Point::new(20, 40), text_style)
        .draw(&mut display)
        .expect("Failed to draw text");

    let mut backlight = PinDriver::output(peripherals.pins.gpio32)
        .expect("Failed to configure backlight");
    backlight.set_high().expect("Failed to enable backlight");

    log::info!("Press button A (left)");
    loop {
        let pressed = button.is_low();
        if pressed != was_pressed {
            // Confirm the new level after the mechanical contacts settle.
            FreeRtos::delay_ms(30);
            if button.is_low() == pressed {
                was_pressed = pressed;
                if pressed {
                    log::info!("Button A pressed");
                    display.clear(Rgb565::RED)
                        .expect("Failed to clear display");
                    Text::new("Button A pressed", Point::new(20, 40), text_style)
                        .draw(&mut display)
                        .expect("Failed to draw text");
                }
            }
        }
        FreeRtos::delay_ms(10);
    }
}
