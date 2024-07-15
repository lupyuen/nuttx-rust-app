/****************************************************************************
 * apps/examples/hello_rust/hello_rust_main.rs
 *
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.  The
 * ASF licenses this file to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance with the
 * License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
 * WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  See the
 * License for the specific language governing permissions and limitations
 * under the License.
 *
 ****************************************************************************/

/****************************************************************************
 * Attributes
 ****************************************************************************/

// TODO: Uncomment this
// #![no_main]
// #![no_std]

/****************************************************************************
 * Uses
 ****************************************************************************/

// TODO: Uncomment this
// use core::panic::PanicInfo;

/****************************************************************************
 * Externs
 ****************************************************************************/

extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
    pub fn open(path: *const u8, oflag: i32, ...) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn ioctl(fd: i32, request: i32, ...) -> i32;
    pub fn usleep(usec: u32) -> u32;
}

/****************************************************************************
 * Constants
 ****************************************************************************/

pub const O_WRONLY: i32 = 1 << 1;
pub const ULEDIOC_SETALL: i32 = 0x1d03;

/****************************************************************************
 * Private Functions
 ****************************************************************************/

/****************************************************************************
 * Panic Handler (needed for [no_std] compilation)
 ****************************************************************************/

// TODO: Uncomment this
// #[panic_handler]
// fn panic(_panic: &PanicInfo<'_>) -> ! {
//     loop {}
// }

/****************************************************************************
 * Public Functions
 ****************************************************************************/

/****************************************************************************
 * hello_rust_main
 ****************************************************************************/

#[no_mangle]
pub extern "C" fn hello_rust_main(_argc: i32, _argv: *const *const u8) -> Result<i32, i32> {
    /* "Hello, Rust!!" using printf() from libc */

    new_puts("Hello, Rust!!");

    /* Blink LED 1 using ioctl() from NuttX */

    new_puts("Opening /dev/userleds");
    let fd = new_open(b"/dev/userleds\0" as *const u8, O_WRONLY)?;
    new_puts("Set LED 1 to 1");
    
    new_ioctl(fd, ULEDIOC_SETALL, 1)?;
    new_puts("Sleeping...");
    unsafe { usleep(500_000); }

    new_puts("Set LED 1 to 0");
    new_ioctl(fd, ULEDIOC_SETALL, 0)?;
    unsafe { close(fd); }

    /* Exit with status 0 */

    Ok(0)
}

// TODO: Remove this function
#[no_mangle]
pub extern "C" fn old_hello_rust_main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        /* "Hello, Rust!!" using printf() from libc */

        printf(b"Old Hello, Rust!!\n\0" as *const u8);

        /* Blink LED 1 using ioctl() from NuttX */

        printf(b"Opening /dev/userleds\n\0" as *const u8);
        let fd = open(b"/dev/userleds\0" as *const u8, O_WRONLY);
        if fd < 0 {
            printf(b"Unable to open /dev/userleds, skipping the blinking\n\0" as *const u8);
            return 1;
        }

        printf(b"Set LED 1 to 1\n\0" as *const u8);
        let ret = ioctl(fd, ULEDIOC_SETALL, 1);
        if ret < 0 {
            printf(b"ERROR: ioctl(ULEDIOC_SETALL) failed\n\0" as *const u8);
            close(fd);
            return 1;
        }

        printf(b"Sleeping...\n\0" as *const u8);
        usleep(500_000);

        printf(b"Set LED 1 to 0\n\0" as *const u8);
        let ret = ioctl(fd, ULEDIOC_SETALL, 0);
        if ret < 0 {
            printf(b"ERROR: ioctl(ULEDIOC_SETALL) failed\n\0" as *const u8);
            close(fd);
            return 1;
        }

        close(fd);
    }

    /* Exit with status 0 */

    0
}

// Safer Version of open()
pub fn new_open(_path: *const u8, _oflag: i32) -> Result<i32, i32> {
    // TODO: Call open()
    println!("TODO: Call open()");
    Ok(1)
}

// Safer Version of ioctl()
pub fn new_ioctl(_fd: i32, _request: i32, _arg: i32) -> Result<i32, i32> {
    // TODO: Call ioctl()
    println!("TODO: Call ioctl()");
    Ok(0)
}

// Safer Version of puts()
pub fn new_puts(s: &str) {
    // TODO: Call puts()
    println!("{}", s);
}

// TODO: Remove the Main Function
fn main() {
    // Call the Old Version of Rust Main
    old_hello_rust_main(0, std::ptr::null());

    // Call the New Version of Rust Main
    _ = hello_rust_main(0, std::ptr::null());
}
