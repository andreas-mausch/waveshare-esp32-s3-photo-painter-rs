mod epaper;

use esp_idf_hal::delay::{FreeRtos, TickType};
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::FromValueType;
use esp_idf_hal::spi::config::{Config, Duplex::Half, MODE_0};
use esp_idf_hal::spi::SpiAnyPins;
use esp_idf_hal::spi::SpiDeviceDriver;
use esp_idf_hal::spi::SpiDriver;
use esp_idf_hal::spi::SpiDriverConfig;
use esp_idf_hal::task::queue::Queue;

use crate::epaper::EPaper7In3F;

const QUEUE_SIZE: usize = 10;
const MESSAGE_BUTTON_PRESSED: u32 = 1;

fn button_pressed<P5: Pin, P6: Pin>(
  button: &mut PinDriver<P5, Input>,
  led_state: &mut bool,
  green_led: &mut PinDriver<P6, Output>
) -> anyhow::Result<()> {
  log::info!("Button pressed");

  if *led_state {
    green_led.set_high()?;
    *led_state = false;
  } else {
    green_led.set_low()?;
    *led_state = true;
  }

  // Note that PinDriver::enable_interrupt should also be called after each received
  // notification from non-ISR context, because the driver will automatically
  // disable ISR interrupts on each received ISR notification
  button.enable_interrupt()?;

  Ok(())
}

pub fn spi_init<'a, SPI: SpiAnyPins, P1: OutputPin, P2: OutputPin>(
  spi: impl Peripheral<P = SPI> + 'a,
  sclk: &'a mut P1,
  sdo: &'a mut P2
) -> anyhow::Result<SpiDeviceDriver<'a, SpiDriver<'a>>> {
  let sdi = AnyInputPin::none();
  let cs = AnyOutputPin::none();

  let config = Config::new()
    .baudrate(2u32.MHz().into())
    .data_mode(MODE_0)
    .duplex(Half);

  Ok(SpiDeviceDriver::new_single(
    spi,
    sclk,
    sdo,
    sdi,
    cs,
    &SpiDriverConfig::new(),
    &config
  )?)
}

fn main() -> anyhow::Result<()> {
  // It is necessary to call this function once. Otherwise, some patches to the runtime
  // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
  esp_idf_svc::sys::link_patches();

  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();

  log::info!("Hello, world!");

  let queue = Queue::new(QUEUE_SIZE);

  let mut peripherals = Peripherals::take()?;
  let spi_device = spi_init(
    &mut peripherals.spi3,
    &mut peripherals.pins.gpio10,
    &mut peripherals.pins.gpio11
  )?;

  // See here:
  // https://github.com/esp-rs/esp-idf-hal/blob/v0.45.2/examples/button_interrupt.rs
  let mut green_led = PinDriver::output(peripherals.pins.gpio42)?;
  let mut red_led = PinDriver::output(peripherals.pins.gpio45)?;
  let mut button = PinDriver::input(peripherals.pins.gpio4)?;
  button.set_pull(Pull::Up)?;
  button.set_interrupt_type(InterruptType::PosEdge)?;

  let mut epaper_dc_pin = PinDriver::output(peripherals.pins.gpio8)?;
  let mut epaper_cs_pin = PinDriver::output(peripherals.pins.gpio9)?;
  let mut epaper_reset_pin = PinDriver::output(peripherals.pins.gpio12)?;
  let epaper_busy_pin = PinDriver::input(peripherals.pins.gpio13)?;

  epaper_dc_pin.set_low()?;
  epaper_cs_pin.set_high()?;
  epaper_reset_pin.set_low()?;

  let mut epaper = EPaper7In3F::new(
    spi_device,
    epaper_dc_pin,
    epaper_cs_pin,
    epaper_reset_pin,
    epaper_busy_pin
  );

  log::info!("Drawing..");
  epaper.init().unwrap();
  // epaper.clear(epaper::Color::Blue).unwrap();
  epaper.show_seven_color_blocks().unwrap();
  log::info!("Done");

  let mut led_state = false;
  // Turn both LEDs off
  red_led.set_high()?;
  green_led.set_high()?;

  // register interrupt callback, here it's a closure on stack
  unsafe {
    button
      .subscribe_nonstatic(|| {
        #[allow(clippy::unwrap_used)]
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
