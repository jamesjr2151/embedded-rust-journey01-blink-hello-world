# ESP32 Blinky - Embedded Rust Journey01

## About This Project
This is my first Embedded Rust project, demonstrating GPIO control on ESP32.

## Technical Stack
- **MCU**: ESP32
- **Language**: Rust 
- **Framework**: `esp-idf-hal`
- **Target**: `xtensa-esp32-espidf`
- **LED Pin**: GPIO2

## Prerequisites
Rust installed (https://rustup.rs/)

# Install espup globally (as standard RUST only supports x86/ARM architectures)
cargo install espup

# Install ESP toolchain and ESP-IDF
espup install

# Source the environment (do this in every new terminal)
source ~/export-esp.sh

# Optional: Add to your shell profile to run automatically
echo "source ~/export-esp.sh" >> ~/.bashrc  # for bash
# OR
echo "source ~/export-esp.sh" >> ~/.zshrc   # for zsh

# Check ESP targets are available
rustup target list | grep esp

# You should see:
# xtensa-esp32-espidf
# xtensa-esp32s2-espidf
# xtensa-esp32s3-espidf
# riscv32imc-esp-espidf
# etc.

# clone the project
git clone https://github.com/jamesjr2151/embedded-rust-journey01-blink-hello-world.git

# Build for ESP32
cargo build --target xtensa-esp32-espidf

# Install cargo-espflash
cargo install cargo-espflash

# Flash the firmware
cargo espflash flash --target xtensa-esp32-espidf

# Monitor serial output
cargo espflash monitor --target xtensa-esp32-espidf

# common issues
- "Failed to parse edition" error
Ensure edition = "2021" in Cargo.toml
Delete Cargo.lock and run cargo clean

- Dependency version errors
Use compatible versions: esp-idf-hal = "0.45" and esp-idf-sys = "0.36"

- "Toolchain doesn't support target"
Verify target within rustup target list | grep esp
you might skip to Install espup globally
Make sure you ran source ~/export-esp.sh

- Build fails with linking errors
Ensure ESP-IDF is properly set up via espup
Check all environment variables are set