# Details and Specs

Source: https://www.waveshare.com/esp32-s3-photopainter.htm

![Details](Waveshare-ESP32-S3-PhotoPainter-details.jpg)

1. ESP32-S3-WROOM-1-N16R8
   Xtensa 32-bit LX7 dual-core processor
   Supports 2.4GHz Wi-Fi and Bluetooth 5 (LE), with onboard antenna
   240MHz operating frequency
   Built-in 512KB SRAM, 384KB ROM, with integrated 16MB Flash and 8MB PSRAM
2. SHTC3 temperature and humidity sensor
   provides ambient temperature and humidity measurements, enabling environmental monitoring function
3. ES7210 ADC audio decoder chip
   high-performance and low-power consumption audio ADC, supports multiple-channel microphone inputs
4. ES8311 DAC audio encoder chip
   high-performance and low-power consumption audio DAC
5. AXP2101
   highly integrated power management chip
6. TF card slot
   supports FAT32-formatted TF card for data expansion
7. MX1.25 2PIN speaker header
   audio output signal, for connecting external speaker
8. MX1.25 2PIN Lithium battery header
   for connecting Lithium battery
9. MX1.25 2PIN RTC backup battery header
   supports rechargeable RTC batteries only
10. KEY button
    for custom function
11. BOOT button
    press it when powering on to enter download mode
12. PWR power button
    for system power supply ON/OFF
13. PCF85063
    RTC chip, for time keeping function
14. Dual-microphone array design
    dual microphone array, supporting richer voice interaction functions
15. Type-C port
    for program burning and log printing

# e-Paper Display

The display is the [7.3inch e-Paper HAT (E)][1].
Waveshare has different kind of e-Paper displays, so watch out to not confuse them.

The previous version I owned uses the [7.3inch e-Paper HAT (F)][2].

[1]: https://www.waveshare.com/wiki/7.3inch_e-Paper_HAT_(E)_Manual
[2]: https://www.waveshare.com/wiki/7.3inch_e-Paper_HAT_(F)_Manual

# PhotoPainter Comparison

|                   | **ESP32-S3-PhotoPainter** | **PhotoPainter**         | **RPi Zero PhotoPainter** |
|-------------------|---------------------------|--------------------------|---------------------------|
| Processor         | ESP32-S3-WROOM-1-N16R8    | Raspberry Pi Pico RP2040 | Raspberry Pi Zero 2W[^1]  |
| Flash             | 16 MB                     | 2 MB                     | microSD card              |
| RAM               | 512 KB SRAM + 8 MB PSRAM  | 264 KB SRAM              | 512 MB                    |
| Display           | 7.3inch e-Paper HAT (E)   | 7.3inch e-Paper HAT (F)  | 7.3inch e-Paper HAT (E)   |
| WiFi              | 🟢                        | 🔴                       | 🟢                        |
| Bluetooth LE      | 🟢                        | 🔴                       | 🟢                        |
| Bluetooth Classic | 🔴                        | 🔴                       | 🟢                        |

[^1]: You can also use a different Pi Zero model

# Panel Comparison

|              | **7.3inch e-Paper HAT (E)**                | **7.3inch e-Paper HAT (F)**      |
|--------------|--------------------------------------------|----------------------------------|
| Technology   | Spectra 6 (E6)                             | Advanced Color ePaper 7 (ACeP 7) |
| Colors       | 6 (Black, White, Green, Blue, Red, Yellow) | 7 (+ Orange)                     |
| Refresh time | 12s                                        | 35s                              |
| Release date | 2024                                       | 2018                             |
| Notes        | more vibrant colors                        |                                  |

Video: [ACeP vs Spectra 6](https://www.youtube.com/watch?v=FVqNkZOOpg4)
