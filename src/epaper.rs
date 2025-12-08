use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::Input;
use esp_idf_hal::gpio::InputPin;
use esp_idf_hal::gpio::Output;
use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::spi::SpiDeviceDriver;
use esp_idf_hal::spi::SpiDriver;
use esp_idf_hal::sys::EspError;

pub const EPD_7IN3F_WIDTH: usize = 800;
pub const EPD_7IN3F_HEIGHT: usize = 480;

#[derive(Clone, Copy, Debug)]
pub enum Error {
  Timeout,
  #[allow(dead_code)]
  SpiError(EspError)
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Color {
  Black = 0b000,
  White = 0b001,
  Yellow = 0b010,
  Red = 0b011,
  Blue = 0b101,
  Green = 0b110,
  #[allow(dead_code)]
  Clean = 0b111 // Not a real color, used to clear the display.
}

pub struct EPaper7In3F<
  'a,
  P1: OutputPin + 'a,
  P2: OutputPin + 'a,
  P3: OutputPin + 'a,
  P4: InputPin + 'a
> {
  spi: SpiDeviceDriver<'a, SpiDriver<'a>>,
  dc_pin: PinDriver<'a, P1, Output>,
  cs_pin: PinDriver<'a, P2, Output>,
  reset_pin: PinDriver<'a, P3, Output>,
  busy_pin: PinDriver<'a, P4, Input>
}

impl<'a, P1, P2, P3, P4> EPaper7In3F<'a, P1, P2, P3, P4>
where
  P1: OutputPin + 'a,
  P2: OutputPin + 'a,
  P3: OutputPin + 'a,
  P4: InputPin + 'a
{
  pub fn new(
    spi: SpiDeviceDriver<'a, SpiDriver<'a>>,
    dc_pin: PinDriver<'a, P1, Output>,
    cs_pin: PinDriver<'a, P2, Output>,
    reset_pin: PinDriver<'a, P3, Output>,
    busy_pin: PinDriver<'a, P4, Input>
  ) -> Self {
    EPaper7In3F {
      spi,
      dc_pin,
      cs_pin,
      reset_pin,
      busy_pin
    }
  }

  /// Initializes the display.
  pub fn init(&mut self) -> Result<(), Error> {
    self.reset()?;
    self.wait_for_idle()?;
    FreeRtos::delay_ms(30);

    // Magic initialization sequence: replicated from Waveshare C code.
    // https://github.com/waveshareteam/ESP32-S3-PhotoPainter/blob/v2.1.2/01_Example/xiaozhi-esp32/components/port_bsp/display_bsp.cpp#L168
    self.send_cmd_with_data(0xAA, &[0x49, 0x55, 0x20, 0x08, 0x09, 0x18])?;
    self.send_cmd_with_data(0x01, &[0x3F])?;
    self.send_cmd_with_data(0x00, &[0x5F, 0x69])?;
    self.send_cmd_with_data(0x03, &[0x00, 0x54, 0x00, 0x44])?;
    self.send_cmd_with_data(0x05, &[0x40, 0x1F, 0x1F, 0x2C])?;
    self.send_cmd_with_data(0x06, &[0x6F, 0x1F, 0x17, 0x49])?;
    self.send_cmd_with_data(0x08, &[0x6F, 0x1F, 0x1F, 0x22])?;
    self.send_cmd_with_data(0x30, &[0x03])?;
    self.send_cmd_with_data(0x50, &[0x3F])?;
    self.send_cmd_with_data(0x60, &[0x02, 0x00])?;
    self.send_cmd_with_data(0x61, &[0x03, 0x20, 0x01, 0xE0])?;
    self.send_cmd_with_data(0x84, &[0x01])?;
    self.send_cmd_with_data(0xE3, &[0x2F])?;
    self.send_cmd(0x04)?;
    self.wait_for_idle()?;
    Ok(())
  }

  /// Clears the display with the given color.
  pub fn _clear(&mut self, color: Color) -> Result<(), Error> {
    self.send_cmd(0x10)?;
    let color = color as u8;
    let data = [color << 4 | color; EPD_7IN3F_WIDTH / 2];
    for _ in 0..EPD_7IN3F_HEIGHT {
      self.send_data(&data)?;
    }

    self.display_frame()?;
    Ok(())
  }

  /// Draw the seven color blocks on the screen.
  pub fn show_seven_color_blocks(&mut self) -> Result<(), Error> {
    log::info!("show_seven_color_blocks()");
    self.send_cmd(0x10)?;

    let color_list = [
      Color::White,
      Color::Black,
      Color::Blue,
      Color::Green,
      Color::Red,
      Color::Yellow,
      Color::White
    ];
    for color in color_list.iter() {
      log::info!("show_seven_color_blocks(): color");
      let color = *color as u8;
      // This consumes 400 bytes of stack memory, which is probably okay?
      // The alternative is to call send_data() 400 times, which is also toggles the GPIOs 400
      // times.
      let data = [color << 4 | color; EPD_7IN3F_WIDTH / 2];
      for _ in 0..EPD_7IN3F_HEIGHT / color_list.len() {
        self.send_data(&data)?;
      }
    }
    log::info!("show_seven_color_blocks(): ok");
    self.display_frame()?;
    Ok(())
  }

  /* /// Sends the given image to the display.
  pub fn show_image(
    &mut self,
    image: &DisplayBuffer
  ) -> Result<(), Error> {
    self.send_cmd(0x10)?;

    for y in 0..EPD_7IN3F_HEIGHT {
      let offset = y * EPD_7IN3F_WIDTH / 2;
      self.send_data(&image.frame_buffer[offset..(offset + EPD_7IN3F_WIDTH / 2)])?;
      watchdog.feed();
    }

    self.display_frame()?;
    Ok(())
  } */

  /// Puts the display in deep sleep mode.
  pub fn _deep_sleep(&mut self) -> Result<(), Error> {
    self.send_cmd_with_data(0x07, &[0xA5])?;
    Ok(())
  }

  /// Resets the display.
  fn reset(&mut self) -> Result<(), Error> {
    self.reset_pin.set_high().map_err(Error::SpiError)?;
    FreeRtos::delay_ms(50);
    self.reset_pin.set_low().map_err(Error::SpiError)?;
    FreeRtos::delay_ms(20);
    self.reset_pin.set_high().map_err(Error::SpiError)?;
    FreeRtos::delay_ms(50);
    Ok(())
  }

  /// Sends a command to the display.
  fn send_cmd(&mut self, command: u8) -> Result<(), Error> {
    // DC low: next byte is command.
    self.dc_pin.set_low().map_err(Error::SpiError)?;
    // CS low: start command transmission.
    self.cs_pin.set_low().map_err(Error::SpiError)?;
    // Send the command.
    self.spi.write(&[command]).map_err(Error::SpiError)?;
    // CS high: end command transmission.
    self.cs_pin.set_high().map_err(Error::SpiError)?;
    Ok(())
  }

  // Sends data to the display.
  fn send_data(&mut self, data: &[u8]) -> Result<(), Error> {
    // DC high: next byte is data.
    self.dc_pin.set_high().map_err(Error::SpiError)?;
    // CS low: start data transmission.
    self.cs_pin.set_low().map_err(Error::SpiError)?;
    self.spi.write(data).map_err(Error::SpiError)?;
    // CS high: end data transmission.
    self.cs_pin.set_high().map_err(Error::SpiError)?;
    Ok(())
  }

  /// Sends a command with data, to the display.
  fn send_cmd_with_data(&mut self, command: u8, data: &[u8]) -> Result<(), Error> {
    self.send_cmd(command)?;
    self.send_data(data)?;
    Ok(())
  }

  /// Waits for the display to become idle.
  fn wait_for_idle(&mut self) -> Result<(), Error> {
    let max_delay_ms = 500_000;
    let polling_ms = 10;

    let mut accum_ms = 0;
    while self.busy_pin.is_low() {
      FreeRtos::delay_ms(polling_ms);
      accum_ms += polling_ms;
      if accum_ms >= max_delay_ms {
        return Err(Error::Timeout);
      }
    }
    Ok(())
  }

  /// Powers on the display, refreshes (transfers the frame buffer to) the display, and then
  /// powers off the display.
  fn display_frame(&mut self) -> Result<(), Error> {
    log::info!("display_frame()");
    // Power on the display.
    self.send_cmd(0x04)?;
    self.wait_for_idle()?;
    log::info!("display_frame() 1");

    // Second setting
    self.send_cmd_with_data(0x06, &[0x6F, 0x1F, 0x17, 0x49])?;
    self.wait_for_idle()?;
    log::info!("display_frame() 2");

    // Refresh the display.
    self.send_cmd_with_data(0x12, &[0x00])?;
    self.wait_for_idle()?;
    log::info!("display_frame() 3");

    // Power off the display.
    self.send_cmd_with_data(0x02, &[0x00])?;
    self.wait_for_idle()?;
    log::info!("display_frame() done");

    Ok(())
  }
}
