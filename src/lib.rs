#[macro_use] mod hooker;

use libc::{c_int, c_void, size_t, ssize_t, c_char};
use once_cell::sync::Lazy;
use std::{env, slice};
use rand::prelude::*;
use rand::SeedableRng;
use parking_lot::Mutex;

static CHANCE: Lazy<f64> = Lazy::new(|| {
    env::var("CHANCE")
        .unwrap_or_else(|_| "0.1".into())
        .parse::<f64>()
        .unwrap_or(0.1)
        .clamp(0.0, 100.0) * 0.01
});
static RANDOM: Lazy<Mutex<SmallRng>> = Lazy::new(|| Mutex::new(SmallRng::from_entropy()));

hook! {
    unsafe fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t => read_hook {
        let mut rand = RANDOM.lock();
        let bytes_read = real!(read)(fd, buf, count);
        let buf = buf as *mut c_char;
        let buf = slice::from_raw_parts_mut(buf, count);

        for byte in buf {
            if rand.gen::<f64>() < *CHANCE {
                *byte = rand.gen()
            }
        }

        bytes_read
    }
}
