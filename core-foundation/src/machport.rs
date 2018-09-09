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
// use core_foundation_sys::base::CFIndex;
// use core_foundation_sys::base::{kCFAllocatorDefault, CFOptionFlags};
// use core_foundation_sys::string::CFStringRef;

use base::{TCFType};
// use date::{CFAbsoluteTime, CFTimeInterval};
// use filedescriptor::CFFileDescriptor;
// use string::{CFString};

// pub type CFRunLoopMode = CFStringRef;


declare_TCFType!(CFMachPort, CFMachPortRef);
impl_TCFType!(CFMachPort, CFMachPortRef, CFMachPortGetTypeID);
impl_CFTypeDescription!(CFMachPort);

impl CFMachPort {
    // pub fn get_current() -> CFRunLoop {
    //     unsafe {
    //         let run_loop_ref = CFRunLoopGetCurrent();
    //         TCFType::wrap_under_get_rule(run_loop_ref)
    //     }
    // }

    // pub fn get_main() -> CFRunLoop {
    //     unsafe {
    //         let run_loop_ref = CFRunLoopGetMain();
    //         TCFType::wrap_under_get_rule(run_loop_ref)
    //     }
    // }

    // pub fn run_current() {
    //     unsafe {
    //         CFRunLoopRun();
    //     }
    // }

    // pub fn stop(&self) {
    //     unsafe {
    //         CFRunLoopStop(self.0);
    //     }
    // }

    // pub fn current_mode(&self) -> Option<String> {
    //     unsafe {
    //         let string_ref = CFRunLoopCopyCurrentMode(self.0);
    //         if string_ref.is_null() {
    //             return None;
    //         }

    //         let cf_string: CFString = TCFType::wrap_under_create_rule(string_ref);
    //         Some(cf_string.to_string())
    //     }
    // }

    // pub fn contains_timer(&self, timer: &CFRunLoopTimer, mode: CFRunLoopMode) -> bool {
    //     unsafe {
    //         CFRunLoopContainsTimer(self.0, timer.0, mode) != 0
    //     }
    // }

    // pub fn add_timer(&self, timer: &CFRunLoopTimer, mode: CFRunLoopMode) {
    //     unsafe {
    //         CFRunLoopAddTimer(self.0, timer.0, mode);
    //     }
    // }

    // pub fn remove_timer(&self, timer: &CFRunLoopTimer, mode: CFRunLoopMode) {
    //     unsafe {
    //         CFRunLoopRemoveTimer(self.0, timer.0, mode);
    //     }
    // }

    // pub fn contains_source(&self, source: &CFRunLoopSource, mode: CFRunLoopMode) -> bool {
    //     unsafe {
    //         CFRunLoopContainsSource(self.0, source.0, mode) != 0
    //     }
    // }

    // pub fn add_source(&self, source: &CFRunLoopSource, mode: CFRunLoopMode) {
    //     unsafe {
    //         CFRunLoopAddSource(self.0, source.0, mode);
    //     }
    // }

    // pub fn remove_source(&self, source: &CFRunLoopSource, mode: CFRunLoopMode) {
    //     unsafe {
    //         CFRunLoopRemoveSource(self.0, source.0, mode);
    //     }
    // }

    // pub fn contains_observer(&self, observer: &CFRunLoopObserver, mode: CFRunLoopMode) -> bool {
    //     unsafe {
    //         CFRunLoopContainsObserver(self.0, observer.0, mode) != 0
    //     }
    // }

    // pub fn add_observer(&self, observer: &CFRunLoopObserver, mode: CFRunLoopMode) {
    //     unsafe {
    //         CFRunLoopAddObserver(self.0, observer.0, mode);
    //     }
    // }

    // pub fn remove_observer(&self, observer: &CFRunLoopObserver, mode: CFRunLoopMode) {
    //     unsafe {
    //         CFRunLoopRemoveObserver(self.0, observer.0, mode);
    //     }
    // }

}
