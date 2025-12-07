use core::num::NonZero;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::task::notification::Notification;

fn main() -> anyhow::Result<()> {
  // It is necessary to call this function once. Otherwise, some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();

  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  log::info!("Hello, world!");

  let peripherals = Peripherals::take()?;

  // See here:
  // https://github.com/esp-rs/esp-idf-hal/blob/v0.45.2/examples/button_interrupt.rs
  let mut green_led = PinDriver::output(peripherals.pins.gpio42)?;
  let mut red_led = PinDriver::output(peripherals.pins.gpio45)?;
  let mut button = PinDriver::input(peripherals.pins.gpio4)?;
  button.set_pull(Pull::Up)?;
  button.set_interrupt_type(InterruptType::PosEdge)?;

  let mut led_state = false;
  // Turn both LEDs off
  red_led.set_high()?;
  green_led.set_high()?;

  loop {
    // prepare communication channel
    let notification = Notification::new();
    let waker = notification.notifier();

    // register interrupt callback, here it's a closure on stack
    unsafe {
      button
        .subscribe_nonstatic(move || {
          waker.notify(NonZero::new(1).unwrap());
        })
        .unwrap();
    }

    // enable interrupt, will be automatically disabled after being triggered
    button.enable_interrupt()?;
    // block until notified
    notification.wait_any();

    // toggle the LED
    if led_state {
      green_led.set_high()?;
      led_state = false;
    } else {
      green_led.set_low()?;
      led_state = true;
    }

    // debounce
    FreeRtos::delay_ms(200);
  }
}
