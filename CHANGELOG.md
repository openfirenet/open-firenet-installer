# Changelog

## Non publié

### New
- Automatic detection of boards with octal PSRAM when probing over USB or Wi-Fi, displaying PSRAM status and selecting the matching firmware binary (standard or PSRAM) for installations and updates. If an older release does not contain a PSRAM build, the installer informs the user and falls back to the standard binary.
- Local firmware inspection: when selecting a local .bin file, the installer inspects its binary content to detect if it was compiled for ESP32-S3 and with PSRAM support, alerting the user if an incompatible binary is selected.
- The installer is available in Spanish, in the window and on the command line.
