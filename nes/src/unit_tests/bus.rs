use super::*;

#[test]
fn test_mem_read_write_to_ram() {
    let nes: NES = setup_nes();
    nes.bus_mut().write(0x0001, 0x55);

    assert_eq!(nes.bus().read(0x0001), 0x55);
}
