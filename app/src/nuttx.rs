/****************************************************************************
 * Uses
 ****************************************************************************/

use core::cmp::Ord;
use core::result::Result::{self, Err, Ok};

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
 * Constants
 ****************************************************************************/

pub const O_WRONLY: i32 = 1 << 1;
pub const ULEDIOC_SETALL: i32 = 0x1d03;

/****************************************************************************
 * Public Functions
 ****************************************************************************/

pub fn safe_open(path: &str, oflag: i32) -> Result<i32, i32> {

    let byte_str = path.as_bytes();
    let mut buffer = [0u8; 256];
    let len = byte_str.len().min(buffer.len() - 1); // Memory rquired for and null terminator
    buffer[..len].copy_from_slice(&byte_str[..len]);
    buffer[len] = 0;

    let fd = unsafe { open(buffer.as_ptr(), oflag) };
    if fd < 0 {
        Err(fd)
    } else {
        Ok(fd)
    }
}

pub fn safe_ioctl(fd: i32, request: i32, arg: i32) -> Result<i32, i32> {
    let ret = unsafe { ioctl(fd, request, arg) };
    if ret < 0 {
        Err(ret)
    } else {
        Ok(ret)
    }
}

pub fn safe_puts(s: &str) {
    let byte_str = s.as_bytes();
    let mut buffer = [0u8; 256];
    let len = byte_str.len().min(buffer.len() - 1); // Memory rquired for and null terminator
    buffer[..len].copy_from_slice(&byte_str[..len]);
    // below code is commented, because it would be better to provide
    // function like safe_putsln() following the convention:
    // print and println from Java
    // buffer[len] = b'\n';
    buffer[len] = 0;

    unsafe {
        puts(buffer.as_ptr());
    }
}
