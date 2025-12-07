use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
  // It is necessary to call this function once. Otherwise, some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();

  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  log::info!("Hello, world!");

  let peripherals = Peripherals::take()?;

  // See here:
  // https://github.com/esp-rs/esp-idf-hal/blob/v0.45.2/examples/button.rs
  let mut green_led = PinDriver::output(peripherals.pins.gpio45)?;
  let mut red_led = PinDriver::output(peripherals.pins.gpio42)?;
  let mut button = PinDriver::input(peripherals.pins.gpio4)?;
  button.set_pull(Pull::Up)?;

  loop {
    FreeRtos::delay_ms(10);

    if button.is_high() {
      red_led.set_high()?;
      green_led.set_low()?;
    } else {
      red_led.set_low()?;
      green_led.set_high()?;
    }
  }
}
