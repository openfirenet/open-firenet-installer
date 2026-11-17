# Changelog

## v1.14.0 (2026-11-16)

### New
- macOS: the installer is also published as a disk image (`.dmg`) for Apple Silicon and Intel Macs. Open it and drag the app into Applications: no need for the Terminal anymore. The plain programs are still published. By @openfirenet.

## v1.13.0 (2026-11-15)

### New
- Automatic detection of boards with octal PSRAM (8 or 16 MB) when probing over USB or Wi-Fi, displaying PSRAM status and selecting the matching firmware binary (standard or PSRAM) for installations and updates. If an older release does not contain a PSRAM build, the installer informs the user and falls back to the standard binary. Boards with quad PSRAM (2 MB, such as the ESP32-S3FH4R2 chips) are not supported by the PSRAM build: the installer says so and installs the standard build. By @openfirenet.
- Local firmware inspection: when selecting a local .bin file, the installer inspects its binary content to detect if it was compiled for ESP32-S3 and with PSRAM support, alerting the user if an incompatible binary is selected. By @openfirenet.
- Custom styled dropdown menus matching the application theme for serial ports and release selections. By @openfirenet.
- The installer is available in Spanish, in the window and on the command line. By @guillaumeparis2000. (#40)
