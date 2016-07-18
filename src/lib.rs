extern crate libc;
extern crate libpulse_sys;

mod pa_simple;
mod low_level;

pub use pa_simple::{Builder, Reader, Writer};

#[test]
fn it_works() {
}
