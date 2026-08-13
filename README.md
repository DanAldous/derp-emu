# DERP-EMU
## Dans Engineered, Redundant, Pathetic Emulator for CHIP8 Systems

New attempt at Rust based CHIP8 Emulator, see if we can't do better than the C# code

* derp_sys      - system level definitions
* * derp_cpu    - CPU/ALU
* * derp_ram    - RAM/Memory handling
* * derp_cart   - load rom into ram
* * derp_gfx    - graphics handling
* * derp_keypad - keypad handler
* * derp_audio  - play sound

Attempts at graphic libraries are in alternate branches, still deciding on stack for presentation, currently testing against ggez crate
