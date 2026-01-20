<h1 align="center">NESMUR v2<br>NES eMUlator in Rust</h1>

### An **ACCURATE** NES emulator written in Rust

### NOTES:

`cargo docs` usage:

- `gen` - Generate docs
- `open` - Open docs server in browser
- `run` - Run the docs server
- `full` - Equivalent to `cargo docs gen open run`

### TODO:

- [ ] Rewrite the whole NES
  - [ ] Emulate individual cycles (including Master Clock?)
  - [ ] Rewrite will require new master ticking method, unsure how to solve
  - [ ] A 100% cycle accurate CPU
  - [ ] System, and partial implementation, of all sub-cycle behaviors (Dummy Reads/Writes, IRQ timing, etc.)
  - [ ] Completely avoid `Rc<RefCell<_>>` crap
  - [ ] Safely share data between threads, avoiding (almost) any extra processing by "nes" thread
  - [ ] Actually implement the APU
  - [ ] Aim for ~110 tests passed in AccuracyCoin
  - [ ] Streamline mapper chip creation + Implement MMC1 (Mapper001)
  - [ ] Access memory only through `Memory` struct (handles mappers in the background)
  - [ ] Use "open bus" design
  - [ ] Multi-region support in PPU
- [ ] Completely rework ThreadCom (maybe own crate), or remove it entirely
- [ ] Create a system to automatically test graphical roms
- [ ] Add proper error handling (specifically to frontend)
- [ ] Switch to egui/eframe
  - [X] Start switch
  - [ ] Reach parity
    - [X] Start + Pause
      - [X] Show UI FPS + FT
      - [ ] Hook it up to the (legacy) NES Manager
    - [ ] Show emulator screen
    - [ ] Show NES State + FPS + FT
    - [ ] Stepping
- [ ] Implement these GUI features
  - [ ] Loading ROMs within the app itself, instead of hardcoding the ROM path
    - [X] Add "Load ROM" button + file dialog
    - [ ] Hook it up to the (legacy) NES Manager
  - [ ] In-app Controller Configuration
    - [X] Keyboard rebinding
    - [ ] Support for physical controllers
    - [ ] Support for up to 2 NES controllers (digital)
    - [ ] Hook it up to the (legacy) NES Manager
  - [ ] Memory Debugger (inspiration: [ocornut/imgui_club - imgui_memory_editor](https://github.com/ocornut/imgui_club/blob/main/imgui_memory_editor/imgui_memory_editor.h))
  - [ ] CPU Debugger ([example format](https://html-preview.github.io/?url=https://github.com/PitchBlackNights/nesmur/blob/main/docs/future_dev/cpu_debugger_example.html))
    - [ ] Intructions Debugger
    - [ ] Code Names/Symbols ([example format](./docs/future_dev/symbols_example.toml))
  - [ ] PPU Nametable Viewer
  - [ ] PPU Tile Viewer
  - [ ] PPU Sprite Viewer
  - [ ] APU Status Viewer
  - [ ] APU Sound Visualizer
