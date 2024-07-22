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
mod nuttx;

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

 // TODO: Rename this function
#[no_mangle]
fn hello_rust_main(_argc: i32, _argv: *const *const u8) -> Result<i32, i32> {
    /* "Hello, Rust!!" using printf() from libc */

    safe_puts("Hello, Rust!!");  // TODO: nuttx::safe_puts

    /* Blink LED 1 using ioctl() from NuttX */

    safe_puts("Opening /dev/userleds");
    let fd = safe_open(b"/dev/userleds\0" as *const u8, nuttx::O_WRONLY)?;  // TODO: nuttx::safe_open
    safe_puts("Set LED 1 to 1");
    
    safe_ioctl(fd, nuttx::ULEDIOC_SETALL, 1)?;  // TODO: nuttx::safe_ioctl
    safe_puts("Sleeping...");
    unsafe { nuttx::usleep(500_000); }

    safe_puts("Set LED 1 to 0");
    safe_ioctl(fd, nuttx::ULEDIOC_SETALL, 0)?;
    unsafe { nuttx::close(fd); }

    /* Exit with status 0 */

    Ok(0)
}

// TODO: Remove this function
#[no_mangle]
pub extern "C" fn old_hello_rust_main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        /* "Hello, Rust!!" using printf() from libc */

        nuttx::printf(b"Old Hello, Rust!!\n\0" as *const u8);

        /* Blink LED 1 using ioctl() from NuttX */

        nuttx::printf(b"Opening /dev/userleds\n\0" as *const u8);
        let fd = nuttx::open(b"/dev/userleds\0" as *const u8, nuttx::O_WRONLY);
        if fd < 0 {
            nuttx::printf(b"Unable to open /dev/userleds, skipping the blinking\n\0" as *const u8);
            return 1;
        }

        nuttx::printf(b"Set LED 1 to 1\n\0" as *const u8);
        let ret = nuttx::ioctl(fd, nuttx::ULEDIOC_SETALL, 1);
        if ret < 0 {
            nuttx::printf(b"ERROR: ioctl(ULEDIOC_SETALL) failed\n\0" as *const u8);
            nuttx::close(fd);
            return 1;
        }

        nuttx::printf(b"Sleeping...\n\0" as *const u8);
        nuttx::usleep(500_000);

        nuttx::printf(b"Set LED 1 to 0\n\0" as *const u8);
        let ret = nuttx::ioctl(fd, nuttx::ULEDIOC_SETALL, 0);
        if ret < 0 {
            nuttx::printf(b"ERROR: ioctl(ULEDIOC_SETALL) failed\n\0" as *const u8);
            nuttx::close(fd);
            return 1;
        }

        nuttx::close(fd);
    }

    /* Exit with status 0 */

    0
}

// TODO: Move to module nuttx: Safer Version of open()
pub fn safe_open(_path: *const u8, _oflag: i32) -> Result<i32, i32> {
    // TODO: Just return the fd as Err or OK
    // TODO: Pass _path and _oflag to open()
    // TODO: Handle _path safely. Allocate a byte array, copy the bytes over, terminate with null
    let fd;
    unsafe {
        fd = nuttx::open(b"/dev/userleds\0" as *const u8, nuttx::O_WRONLY);
    }
    if fd<0 {
        println!("Unable to open /dev/userleds, skipping the blinking");
        // Err(fd)
    } else {
        println!("Opened /dev/userleds successfully");
        // Ok(fd)
    }

    // Return successfully with a File Descriptor
    Ok(1)

    // Or return an error code
    // Err(-1)  
}

// TODO: Move to module nuttx: Safer Version of ioctl()
pub fn safe_ioctl(_fd: i32, _request: i32, _arg: i32) -> Result<i32, i32> {
    // TODO: Just return the ret as Err or OK
    // TODO: Pas _request and _arg to ioctl()
    let ret;
    unsafe {
        ret = nuttx::ioctl(_fd, nuttx::ULEDIOC_SETALL, 1);
    }
    if ret<0 {
        println!("ERROR: ioctl(ULEDIOC_SETALL) failed!");
        // Err(ret)
    } else {
        println!("SUCCESS: ioctl(ULEDIOC_SETALL) completed!");
        // Ok(ret)
    }

    // Return successfully with the ioctl() result
    Ok(0)

    // Or return an error code
    // Err(-1)
}

// TODO: Move to module nuttx: Safer Version of puts()
pub fn safe_puts(s: &str) {
    // TODO: to_owned() requires String from Rust Standard Library, won't work with Rust Core Library
    // Need to allocate a byte array, copy the bytes over, terminate with null
    let mut str_c = s.to_owned();
    str_c.push('\n');
    let c_str = str_c.as_bytes();
    unsafe {
        nuttx::puts(c_str.as_ptr());
    }
    // println!("{}", s);
}

// TODO: Rename the Main Function to hello_rust_main
fn main() {
    // Call the Old Version of Rust Main
    // old_hello_rust_main(0, std::ptr::null());

    // Call the New Version of Rust Main
    let res = hello_rust_main(0, std::ptr::null());

    // If Rust Main returns an error, print it
    if let Err(e) = res {
        // TODO: Call printf()
        println!("ERROR: Failed with error {}", e);
    }
}
