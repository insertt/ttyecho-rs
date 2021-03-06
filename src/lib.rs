//! ttyecho is a crate that lets you write data into a Linux kernel pseudoterminal buffer.
//! This crate will not work on a non-Linux platform.
//! 
//! # Example
//! 
//! ```rust
//! fn main() {
//!     // You can put whatever you want, not only commands.
//!     let command = "echo ttyecho!";
//!     // Target tty 
//!     let tty = "/dev/pts/27";
//!     // We want to append new line as we want to run echo without user interaction.
//!     let append_new_line = true;
//!     
//!     ttyecho(tty, command, append_new_line);
//! }
//! ```
//! 
#[cfg(target_os = "linux")]
use libc::{ ioctl, open, close, TIOCSTI, O_RDWR };

/// Appends given data into given pseudoterminal buffer by using [ioctl] syscall with [TIOCSTI] parameter.
/// It will append a null terminator to the tty path if there isn't one, 
/// because most libc functions expect strings to be null terminated.
#[allow(unused_variables)]
pub fn ttyecho<S: Into<String>>(tty: S, data: S, new_line: bool) {
    #[cfg(target_os = "linux")]
    {
        let mut tty = tty.into();
        let mut command = data.into();

        if ! tty.ends_with('\0') {
            tty.push('\0');
        }

        if new_line && ! command.ends_with('\0') {
            command.push('\r');
        }

        unsafe {
            let fd = open(tty.as_ptr() as *const i8, O_RDWR);

            for ch in command.as_bytes() {
                ioctl(fd, TIOCSTI, ch);
            }
            
            close(fd);
        }
    }
}