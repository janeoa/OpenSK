// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]
#![no_std]

extern crate lang_items;

use libtock_console::Console;
use libtock_drivers::result::FlexUnwrap;
use libtock_platform::Syscalls;
use libtock_runtime::{set_main, stack_size, TockSyscalls};
use core::fmt::Write;

stack_size! {0x800}
set_main! {main}

type S = TockSyscalls;

fn main() {
    const CAP_TOUCH_PIN: u32 = 0;  // P0_05 pin
    // const THRESHOLD: u32 = 500;    // Threshold for touch detection
    const CHARGE_TIME_US: u32 = 100; // Charge time in microseconds
    // const POLL_INTERVAL_MS: u64 = 50; // Check every 50ms
    const GPIO_DRIVER_NUM: u32 = 0x4;  // GPIO driver number

    // Write messages of length up to the console driver's buffer size.
    // let mut buf = [0; 1024];
    loop {
        let _ = S::command(GPIO_DRIVER_NUM, 1 /*ENABLE_OUTPUT*/, CAP_TOUCH_PIN, 0);
        let _ = S::command(GPIO_DRIVER_NUM, 2 /*SET*/, CAP_TOUCH_PIN, 0);
                
        // let pin_config = 0x00100000; // Shift to position 0x00XX0000
        let _ = S::command(GPIO_DRIVER_NUM, 5 /*ENABLE_INPUT*/, CAP_TOUCH_PIN, 0);        
        let result = S::command(GPIO_DRIVER_NUM, 6 /*READ*/, CAP_TOUCH_PIN, 0);
        
        if result.is_success_u32() {
            writeln!(Console::<S>::writer(), "Value before discharge: {}", result.get_success_u32().unwrap()).unwrap();
        } else {
            writeln!(Console::<S>::writer(), "Failed to read pin value before discharge").unwrap();        
        }
    }
}
