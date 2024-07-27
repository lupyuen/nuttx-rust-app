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

pub fn safe_open(path: *const u8, oflag: i32) -> Result<i32, i32> {
    let fd;
    unsafe {
        fd = open(path, oflag);
    }
    // TODO: Commet below Ok, and Uncomment if-else block
    Ok(fd)
    // if fd<0 {
    //     // println!("Unable to open /dev/userleds, skipping the blinking");
    //     Err(fd)
    // } else {
    //     // println!("Opened /dev/userleds successfully");
    //     Ok(fd)
    // }
}

pub fn safe_ioctl(fd: i32, request: i32, arg: i32) -> Result<i32, i32> {
    let ret;
    unsafe {
        ret = ioctl(fd, request, arg);
    }
    // TODO: Commet below Ok, and Uncomment if-else block
    Ok(ret)
    // if ret<0 {
    //     // println!("ERROR: ioctl(ULEDIOC_SETALL) failed!");
    //     Err(ret)
    // } else {
    //     // println!("SUCCESS: ioctl(ULEDIOC_SETALL) completed!");
    //     Ok(ret)
    // }
}

// TODO: Move here: Safer Version of puts()
// pub fn safe_puts(s: &str) {
// }
