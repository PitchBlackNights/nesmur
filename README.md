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
  - [ ] Emulate individual cycles (PPU, CPU, & APU)
  - [ ] Rewrite will require new master ticking method, unsure how to solve
  - [ ] At least a 100% accurate CPU
  - [ ] Avoid `Rc<RefCell<_>>` crap, global "app" context passed to each component? (just like frontend)
  - [ ] Safely share data between threads, avoiding (almost) any extra processing by "nes" thread: [data_buffer_example.rs](./docs/future_dev/data_buffer_example.rs)
    - [ ] Implement using custom `Buffered<T>` struct/type
    - [ ] Functions used to access/write data: `.set()`, `.get()`, `.add()`, `.wrapping_add()`, `.sub()`, `.wrapping_sub()`
  - [ ] Actually implement the APU
  - [ ] Aim for ~110 tests passed in AccuracyCoin
  - [ ] Streamline mapper chip creation + Implement MMC1 (Mapper001)
  - [ ] Access memory only through `Memory` struct (handles mappers in the background)
  - [ ] Use "open bus" design
  - [ ] Multi-region support in PPU
  - [ ] Other stuff, etc.
- [ ] Make `ThreadCom` take a type for Channel Messages (`ThreadCom<T> -> channel::_::<T>()`)
  - [ ] Requires making `MessagePacket` take a type too (`MessagePacket<T>(&'static str, T)`)
- [ ] Create a system to automatically test graphical roms
- [ ] Add proper error handling (specifically to frontend)
- [ ] GUI Specifics
  - [ ] Implement loading ROMs within the app itself, instead of hardcoding the ROM path
  - [ ] Memory Debugger (inspiration: [ocornut/imgui_club - imgui_memory_editor](https://github.com/ocornut/imgui_club/blob/main/imgui_memory_editor/imgui_memory_editor.h))
  - [ ] CPU Debugger ([example format](./docs/future_dev/cpu_debugger_example.html))
    - [ ] Intructions Debugger
    - [ ] Code Names/Symbols ([example format](./docs/future_dev/symbols_example.toml))
  - [ ] PPU Nametable Viewer
  - [ ] PPU Tile Viewer
  - [ ] PPU Sprite Viewer
  - [ ] APU Status Viewer
  - [ ] APU Sound Visualizer
