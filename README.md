# About

This is an alternative firmware to the
*Waveshare ESP32-S3-PhotoPainter 7.3inch E6 Full Color E-paper Display*.

Product: [Link][1].

The [original firmware][2] is written in C++ and
includes lot of stuff, like Audio speech-to-text,
AI image generation and display weather information.

My goal is to write a very basic firmware instead,
written in Rust, but with the feature to send new images over Bluetooth BLE.

It is also a learning project for me to get to know the ESP32 better.

Note: Waveshare has very similar products, do not confuse this project
with the [two][3] [PhotoPainters][4] they offer with a Raspberry Pi Pico.
They don't have **WiFi or Bluetooth**.

There is also [one][5] version with a Raspberry Pi Zero.
If you use a Raspberry Pi Zero W or 2W, this one does have WiFi and Bluetooth.

However, this project is only for the **ESP32-S3-PhotoPainter**
and does not work on any other model.

# Progress

This is still very early WIP.
Nothing works so far.

- [x] Flash own firmware works
- [x] Access the display via SPI
- [ ] Send images via Bluetooth BLE
- [ ] Use RTC
- [ ] Have a companion app for Android

# Development

See [Development.md](Development.md).

# Hardware details and comparison

See [documentation/Waveshare-ESP32-S3-PhotoPainter.md](documentation/Waveshare-ESP32-S3-PhotoPainter.md).

# Similar project

Tim Boldt has written a very similar firmware like this,
just for the Pi Pico version of the PhotoPainter: [Link][6]

[1]: https://www.waveshare.com/esp32-s3-photopainter.htm "Waveshare ESP32-S3-PhotoPainter"
[2]: https://github.com/waveshareteam/ESP32-S3-PhotoPainter/tree/v2.1.2/01_Example/xiaozhi-esp32 "Original Firmware"
[3]: https://www.waveshare.com/wiki/PhotoPainter
[4]: https://www.waveshare.com/wiki/PhotoPainter_(B)
[5]: https://www.waveshare.com/wiki/RPi_Zero_PhotoPainter
[6]: https://github.com/timboldt/waveshare-photopainter
