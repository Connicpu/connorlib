extern crate libc;
extern crate byteorder;

#[cfg(windows)]
extern crate named_pipe;

#[cfg(windows)]
pub use windows::IpcClient;
#[cfg(unix)]
pub use unix::IpcClient;

pub mod ffi;

#[cfg(windows)]
mod windows;
#[cfg(unix)]
mod unix;
