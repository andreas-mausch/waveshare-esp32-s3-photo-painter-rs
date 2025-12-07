use esp_idf_hal::delay::{FreeRtos, TickType};
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::task::queue::Queue;

const QUEUE_SIZE: usize = 10;
const MESSAGE_BUTTON_PRESSED: u32 = 1;

fn button_pressed<P1: Pin, P2: Pin>(
  button: &mut PinDriver<P1, Input>,
  led_state: &mut bool,
  green_led: &mut PinDriver<P2, Output>
) -> anyhow::Result<()> {
  log::info!("Button pressed");

  if *led_state {
    green_led.set_high()?;
    *led_state = false;
  } else {
    green_led.set_low()?;
    *led_state = true;
  }

  button.enable_interrupt()?;

  Ok(())
}

fn main() -> anyhow::Result<()> {
  // It is necessary to call this function once. Otherwise, some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();

  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  log::info!("Hello, world!");

  let queue = Queue::new(QUEUE_SIZE);

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

  // register interrupt callback, here it's a closure on stack
  unsafe {
    button
      .subscribe_nonstatic(|| {
        queue.send_front(MESSAGE_BUTTON_PRESSED, TickType::new_millis(5000).into()).unwrap();
      })?;
  }
  button.enable_interrupt()?;

  loop {
    if let Some((message, _higher_priority_task_awoken)) =
      queue.recv_front(TickType::new_millis(5000).into())
    {
      log::info!("Message received from queue: {:?}", message);

      match message {
        MESSAGE_BUTTON_PRESSED => button_pressed(&mut button, &mut led_state, &mut green_led)?,
        _ => log::info!("Unknown message received, ignoring")
      }
    }

    // debounce
    FreeRtos::delay_ms(200);
  }
}
