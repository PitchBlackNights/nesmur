<h1 align="center">NESMUR v2<br>NES eMUlator in Rust</h1>

### An **ACCURATE** NES emulator written in Rust

### TODO:

- [ ] Rewrite the whole NES
  - [ ] Emulate individual cycles (PPU, CPU, & APU)
  - [ ] Rewrite will require new master ticking method, unsure how to solve
  - [ ] At least a 100% accurate CPU
  - [ ] Avoid `Rc<RefCell<_>>` crap, global "app" context passed to each component? (just like frontend)
  - [ ] Safely share data between threads, avoiding (almost) any extra processing by "nes" thread: [data_buffer_example.rs](./dev_examples/data_buffer_example.rs)
    - [ ] Implement using custom `Buffered<T>` struct/type
    - [ ] Functions used to access/write data: `.set()`, `.get()`, `.add()`, `.wrapping_add()`, `.sub()`, `.wrapping_sub()`
  - [ ] Actually implement the APU
  - [ ] Aim for ~110 tests passed in AccuracyCoin
  - [ ] Streamline mapper chip creation + Implement MMC1 (Mapper001)
  - [ ] Use "open bus" design
  - [ ] Multi-region support in PPU
  - [ ] Other stuff, etc.
- [ ] Create a system to automatically test graphical roms
- [ ] Add proper error handling (specifically to frontend)
- [ ] GUI Specifics
  - [ ] Implement loading ROMs within the app itself, instead of hardcoding the ROM path
  - [ ] Memory Debugger (inspiration: [ocornut/imgui_club - imgui_memory_editor](https://github.com/ocornut/imgui_club/blob/main/imgui_memory_editor/imgui_memory_editor.h))
  - [ ] CPU Debugger ([example format](./dev_examples/cpu_debugger_example.html))
    - [ ] Intructions Debugger
    - [ ] Code Names/Symbols ([example format](./dev_examples/symbols_example.toml))
  - [ ] PPU Nametable Viewer
  - [ ] PPU Tile Viewer
  - [ ] PPU Sprite Viewer
  - [ ] APU Status Viewer
  - [ ] APU Sound Visualizer
