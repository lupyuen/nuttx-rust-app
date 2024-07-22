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

/*
// TODO: Move here: Safer Version of open()
pub fn safe_open(_path: *const u8, _oflag: i32) -> Result<i32, i32> {
    // Return successfully with a File Descriptor
    Ok(1)

    // Or return an error code
    // Err(-1)  
}

// TODO: Move here: Safer Version of ioctl()
pub fn safe_ioctl(_fd: i32, _request: i32, _arg: i32) -> Result<i32, i32> {
    // Return successfully with the ioctl() result
    Ok(0)

    // Or return an error code
    // Err(-1)
}

// TODO: Move here: Safer Version of puts()
pub fn safe_puts(s: &str) {
} 
*/
