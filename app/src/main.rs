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
 * Modules
 ****************************************************************************/

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
fn hello_rust_main(_argc: i32, _argv: *const *const u8) -> Result<i32, i32> {
    /* "Hello, Rust!!" using printf() from libc */

    nuttx::safe_puts("Hello, Rust!!");

    /* Blink LED 1 using ioctl() from NuttX */

    nuttx::safe_puts("Opening /dev/userleds");
    let fd = nuttx::safe_open("/dev/userleds", nuttx::O_WRONLY)?;
    nuttx::safe_puts("Set LED 1 to 1");
    
    nuttx::safe_ioctl(fd, nuttx::ULEDIOC_SETALL, 1)?;
    nuttx::safe_puts("Sleeping...");
    unsafe { nuttx::usleep(500_000); }

    nuttx::safe_puts("Set LED 1 to 0");
    nuttx::safe_ioctl(fd, nuttx::ULEDIOC_SETALL, 0)?;
    unsafe { nuttx::close(fd); }

    /* Exit with status 0 */

    Ok(0)
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
