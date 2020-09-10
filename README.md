# Simple NES emulator

__Implemented:__
    - CPU
    - PPU (not all functionality)
    - Controll
    - One mapper for cartridges (000)

__Not implemented:__
    - APU
    - clock rate
    - some functional of PPU

Build and run emulator (Nes file should be in project directory, and has name 'smb.nes'):
```
cargo run --release
```