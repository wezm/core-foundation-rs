// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use base::{CFAllocatorRef, CFIndex, CFTypeID, Boolean};
use data::CFDataRef;
use date::CFTimeInterval;
use runloop::CFRunLoopSourceRef;
use string::CFStringRef;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct CFMachPortContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<unsafe extern fn(info: *const c_void) -> *const c_void>,
    pub release: Option<unsafe extern fn(info: *const c_void)>,
    pub copyDescription: Option<unsafe extern fn(info: *const c_void)
        -> CFStringRef>,
}

// typedef void (*CFMachPortCallBack)(CFMachPortRef port, void *msg, CFIndex size, void *info);
pub type CFMachPortCallBack = Option<
    unsafe extern fn(local: CFMachPortRef,
                     msg: *mut c_void,
                     size: CFIndex,
                     info: *mut c_void)>;

// typedef void (*CFMachPortInvalidationCallBack)(CFMachPortRef port, void *info);
pub type CFMachPortInvalidationCallBack = Option<
    unsafe extern "C" fn(ms: CFMachPortRef, info: *mut c_void)>;

#[repr(C)]
pub struct __CFMachPort(c_void);
pub type CFMachPortRef = *mut __CFMachPort;

extern {
    /*
     * CFMessagePort.h
     */
    pub fn CFMachPortGetTypeID() -> CFTypeID;

// CF_EXPORT CFMachPortRef	CFMachPortCreate(CFAllocatorRef allocator, CFMachPortCallBack callout, CFMachPortContext *context, Boolean *shouldFreeInfo);
// CF_EXPORT CFMachPortRef	CFMachPortCreateWithPort(CFAllocatorRef allocator, mach_port_t portNum, CFMachPortCallBack callout, CFMachPortContext *context, Boolean *shouldFreeInfo);

// CF_EXPORT mach_port_t	CFMachPortGetPort(CFMachPortRef port);
// CF_EXPORT void		CFMachPortGetContext(CFMachPortRef port, CFMachPortContext *context);
// CF_EXPORT void		CFMachPortInvalidate(CFMachPortRef port);
// CF_EXPORT Boolean	CFMachPortIsValid(CFMachPortRef port);
// CF_EXPORT CFMachPortInvalidationCallBack CFMachPortGetInvalidationCallBack(CFMachPortRef port);
// CF_EXPORT void		CFMachPortSetInvalidationCallBack(CFMachPortRef port, CFMachPortInvalidationCallBack callout);

// CF_EXPORT CFRunLoopSourceRef	CFMachPortCreateRunLoopSource(CFAllocatorRef allocator, CFMachPortRef port, CFIndex order);




//     pub fn CFMessagePortCreateLocal(allocator: CFAllocatorRef,
//                                     name: CFStringRef,
//                                     callout: CFMessagePortCallBack,
//                                     context: *const CFMessagePortContext,
//                                     shouldFreeInfo: *mut Boolean)
//         -> CFMessagePortRef;
//     pub fn CFMessagePortCreateRemote(allocator: CFAllocatorRef,
//                                      name: CFStringRef) -> CFMessagePortRef;
//     pub fn CFMessagePortIsRemote(ms: CFMessagePortRef) -> Boolean;
//     pub fn CFMessagePortGetName(ms: CFMessagePortRef) -> CFStringRef;
//     pub fn CFMessagePortSetName(ms: CFMessagePortRef, newName: CFStringRef)
//         -> Boolean;
//     pub fn CFMessagePortGetContext(ms: CFMessagePortRef,
//                                    context: *mut CFMessagePortContext);
//     pub fn CFMessagePortInvalidate(ms: CFMessagePortRef);
//     pub fn CFMessagePortIsValid(ms: CFMessagePortRef) -> Boolean;
//     pub fn CFMessagePortGetInvalidationCallBack(ms: CFMessagePortRef)
//         -> CFMessagePortInvalidationCallBack;
//     pub fn CFMessagePortSetInvalidationCallBack(ms: CFMessagePortRef,
//                                                 callout: CFMessagePortInvalidationCallBack);
//     pub fn CFMessagePortSendRequest(remote: CFMessagePortRef, msgid: i32,
//                                     data: CFDataRef,
//                                     sendTimeout: CFTimeInterval,
//                                     rcvTimeout: CFTimeInterval,
//                                     replyMode: CFStringRef,
//                                     returnData: *mut CFDataRef) -> i32;
//     pub fn CFMessagePortCreateRunLoopSource(allocator: CFAllocatorRef,
//                                             local: CFMessagePortRef,
//                                             order: CFIndex)
//         -> CFRunLoopSourceRef;
//     // CFMessagePortSetDispatchQueue
}
