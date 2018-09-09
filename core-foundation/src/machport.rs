// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

pub use core_foundation_sys::machport::*;

use core_foundation_sys::base::{Boolean, CFIndex};
use core_foundation_sys::base::{kCFAllocatorDefault, CFOptionFlags};

use base::TCFType;
use runloop::CFRunLoopSource;

declare_TCFType!(CFMachPort, CFMachPortRef);
impl_TCFType!(CFMachPort, CFMachPortRef, CFMachPortGetTypeID);
impl_CFTypeDescription!(CFMachPort);

impl CFMachPort {
    pub fn to_run_loop_source(&self, order: CFIndex) -> Option<CFRunLoopSource> {
        unsafe {
            let source_ref = CFMachPortCreateRunLoopSource(
                kCFAllocatorDefault,
                self.0,
                order
            );
            if source_ref.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(source_ref))
            }
        }
    }

}
