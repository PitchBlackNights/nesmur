use crate::prelude::*;
// use std::thread::sleep;
use std::time::{Duration, Instant};

fn emulate_timings(
    cpf: u32,
    ppu_div: u32,
    cpu_div: u32,
    _target_fps: u32,
    frames: u32,
    test_cycles: u32,
) -> (f64, f64) {
    let end: u32 = cpf * frames;
    let mut loops: u32 = 0;
    let mut frametime_avg: f64 = 0.0;
    let mut cycletime_avg: f64 = 0.0;

    let mut cycle: u32 = 0;
    let mut ppu: u32 = 0;
    let mut cpu: u32 = 0;

    let mut time: Instant = Instant::now();
    loop {
        cycle += 1;

        if cycle % ppu_div == 0 {
            ppu += 1;
        }
        if cycle % cpu_div == 0 {
            cpu += 1;
        }

        if cycle == end {
            let elapsed: f64 = {
                let elapsed_uint: u128 = time.elapsed().as_nanos();
                if elapsed_uint > 2u128.pow(53) {
                    panic!("`elapsed` is greater than the integers that can be exactly represented by `f64`!");
                }
                elapsed_uint as f64
            };
            let frametime: f64 = elapsed / frames as f64;
            let cycletime: f64 = elapsed / cycle as f64;

            /*
            trace!("ELAPSED: {} ns", elapsed);
            trace!("FRAMETIME: ~{:.2} ns", frametime);
            info!(
                "Completed {} frames in {} ms",
                frames,
                elapsed / 1_000_000.0
            );
            info!(
                "~{:.2} FPS == TARGET {} FPS",
                1_000_000_000.0 / frametime,
                target_fps
            );
            info!(
                "~{:.4} ms/frame == ~{:.4} ns/cycle",
                frametime / 1_000_000.0,
                cycletime
            );
            */

            loops += 1;
            trace!(
                "Completed {}/{}... ({:.2} ms)",
                loops,
                test_cycles,
                elapsed / 1_000_000.0
            );
            frametime_avg = frametime_avg + ((frametime - frametime_avg) / (loops as f64));
            cycletime_avg = cycletime_avg + ((cycletime - cycletime_avg) / (loops as f64));
            if loops == test_cycles {
                trace!(
                    "Tested a total of {} groups of {} frames ({} total frames)",
                    test_cycles,
                    frames,
                    frames * test_cycles
                );
                return (frametime_avg, cycletime_avg);
            }

            cycle = 0;
            ppu = 0;
            cpu = 0;
            time = Instant::now();
        }
    }
}

fn u32_insert_commas(num: u32) -> String {
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

fn format_duration(duration: Duration) -> String {
    let total_millis = duration.as_millis();
    let minutes = total_millis / 60_000;
    let seconds = (total_millis % 60_000) / 1_000;
    let millis = total_millis % 1_000;
    format!("{:02}:{:02}.{:03}", minutes, seconds, millis)
}

pub fn test_region_timings() {
    let frames: u32 = 50;
    let test_cycles: u32 = 100;
    info!("===== NTSC =====");
    let ntsc_time_start: Instant = Instant::now();
    let (ntsc_favg, ntsc_cavg) = emulate_timings(357_366, 4, 12, 60, frames, test_cycles);
    let ntsc_time: Duration = ntsc_time_start.elapsed();
    let (ntsc_mcyls, ntsc_pcyls, ntsc_ccyls) = (
        u32_insert_commas((357_366 * frames) * test_cycles),
        u32_insert_commas((89_341 * frames) * test_cycles),
        u32_insert_commas((29_780 * frames) * test_cycles),
    );
    info!("");
    info!("");
    info!("===== PAL  =====");
    let pal_time_start: Instant = Instant::now();
    let (pal_favg, pal_cavg) = emulate_timings(531_960, 5, 16, 50, frames, test_cycles);
    let pal_time: Duration = pal_time_start.elapsed();
    let (pal_mcyls, pal_pcyls, pal_ccyls) = (
        u32_insert_commas((531_960 * frames) * test_cycles),
        u32_insert_commas((106_392 * frames) * test_cycles),
        u32_insert_commas((33_247 * frames) * test_cycles),
    );
    info!("Finished!\n\n");

    info!("===== NTSC =====");
    info!("TIME: {}", format_duration(ntsc_time));
    info!("MSTR CYCLES: {}", ntsc_mcyls);
    info!("PPU CYCLES:  {}", ntsc_pcyls);
    info!("CPU CYCLES:  {}", ntsc_ccyls);
    info!(
        "AVERAGE FRAMETIME: {:.4} ms/frame ({:.2} FPS)",
        ntsc_favg / 1_000_000.0,
        1_000_000_000.0 / ntsc_favg
    );
    info!(
        "AVERAGE CYCLETIME: {:.4} ns/cycle ({:.2} MHz)",
        ntsc_cavg,
        1_000.0 / ntsc_cavg
    );
    info!("");
    info!("");
    info!("===== PAL  =====");
    info!("TIME: {}", format_duration(pal_time));
    info!("MSTR CYCLES: {}", pal_mcyls);
    info!("PPU CYCLES:  {}", pal_pcyls);
    info!("CPU CYCLES:  {}", pal_ccyls);
    info!(
        "AVERAGE FRAMETIME: {:.4} ms/frame ({:.2} FPS)",
        pal_favg / 1_000_000.0,
        1_000_000_000.0 / pal_favg
    );
    info!(
        "AVERAGE CYCLETIME: {:.4} ns/cycle ({:.2} MHz)",
        pal_cavg,
        1_000.0 / pal_cavg
    );
}
