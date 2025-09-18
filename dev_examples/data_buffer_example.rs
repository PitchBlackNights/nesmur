use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

const NUM_BUFFERS: usize = 2;

struct Shared {
    buffers: [Vec<(u8, u8, u8)>; NUM_BUFFERS],
    frametimes: [f32; NUM_BUFFERS],
    current: AtomicUsize, // index of the buffer the ui should read
}

fn main() {
    let shared = Arc::new(Shared {
        buffers: [vec![(0,0,0); 800*600], vec![(0,0,0); 800*600]],
        frametimes: [0.0; NUM_BUFFERS],
        current: AtomicUsize::new(0),
    });

    let render = {
        let shared = Arc::clone(&shared);
        thread::spawn(move || {
            let mut idx = 0;
            loop {
                let buf = &mut shared.buffers[idx];
                let ft  = &mut shared.frametimes[idx];

                // write pixels & frametime
                for px in buf.iter_mut() {
                    *px = (255, 0, 0); // example render
                }
                *ft = 16.67;

                // publish this buffer
                shared.current.store(idx, Ordering::Release);

                // switch buffer
                idx = (idx + 1) % NUM_BUFFERS;
            }
        })
    };

    let ui = {
        let shared = Arc::clone(&shared);
        thread::spawn(move || {
            loop {
                let idx = shared.current.load(Ordering::Acquire);
                let buf = &shared.buffers[idx];
                let ft  = shared.frametimes[idx];

                // safely read without locking
                let color = buf[0];
                println!("UI sees color {:?}, frametime {}", color, ft);
            }
        })
    };

    render.join().unwrap();
    ui.join().unwrap();
}
