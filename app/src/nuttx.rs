/****************************************************************************
 * Uses
 ****************************************************************************/

use core::{
    cmp::Ord,
    result::Result::{self, Err, Ok},
};

/****************************************************************************
 * Externs
 ****************************************************************************/

extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
    pub fn open(path: *const u8, oflag: i32, ...) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn ioctl(fd: i32, request: i32, ...) -> i32;
    pub fn usleep(usec: u32) -> u32;
    pub fn puts(s: *const u8) -> i32;
}

/****************************************************************************
 * Public Constants
 ****************************************************************************/

pub const O_WRONLY: i32 = 1 << 1;
pub const ULEDIOC_SETALL: i32 = 0x1d03;

/****************************************************************************
 * Private Functions
 ****************************************************************************/

/* Copy the Rust Str to the Byte Buffer */

fn copy_to_buffer(s: &str, buffer: &mut [u8]) -> Result<(), i32> {
    let byte_str = s.as_bytes();
    if byte_str.len() >= buffer.len() {
        return Err(-1);
    }
    let len = byte_str.len().min(buffer.len() - 1);
    buffer[..len].copy_from_slice(&byte_str[..len]);
    buffer[len] = 0;
    Ok(())
}

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/* Safe Version of open() */

pub fn safe_open(path: &str, oflag: i32) -> Result<i32, i32> {
    let mut buffer = [0u8; 256];
    copy_to_buffer(path, &mut buffer)?;

    let fd = unsafe { open(buffer.as_ptr(), oflag) };
    if fd < 0 {
        Err(fd)
    } else {
        Ok(fd)
    }
}

/* Safe Version of ioctl() */

pub fn safe_ioctl(fd: i32, request: i32, arg: i32) -> Result<i32, i32> {
    let ret = unsafe { ioctl(fd, request, arg) };
    if ret < 0 {
        Err(ret)
    } else {
        Ok(ret)
    }
}

/* Safe Version of puts() */

pub fn safe_puts(s: &str) {
    let mut buffer = [0u8; 256];
    copy_to_buffer(s, &mut buffer).unwrap();
    unsafe {
        puts(buffer.as_ptr());
    }
}
