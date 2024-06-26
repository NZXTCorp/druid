// Copyright 2020 the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! macOS AppKit bindings.

#![allow(clippy::upper_case_acronyms, non_snake_case, non_upper_case_globals)]

use bitflags::bitflags;
use cocoa::base::id;
use cocoa::foundation::NSRect;
use objc::{class, msg_send, sel, sel_impl};

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    pub static NSRunLoopCommonModes: id;
}

bitflags! {
    pub struct NSTrackingAreaOptions: i32 {
        const MouseEnteredAndExited = 1;
        const MouseMoved = 1 << 1;
        const CursorUpdate = 1 << 2;
        // What's 1 << 3?
        const ActiveWhenFirstResponder = 1 << 4;
        const ActiveInKeyWindow = 1 << 5;
        const ActiveInActiveApp = 1 << 6;
        const ActiveAlways = 1 << 7;
        const AssumeInside = 1 << 8;
        const InVisibleRect = 1 << 9;
        const EnabledDuringMouseDrag = 1 << 10;
    }
}

pub trait NSTrackingArea: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSTrackingArea), alloc]
    }

    unsafe fn initWithRect_options_owner_userInfo(
        self,
        rect: NSRect,
        options: NSTrackingAreaOptions,
        owner: id,
        userInfo: id,
    ) -> id;
}

impl NSTrackingArea for id {
    unsafe fn initWithRect_options_owner_userInfo(
        self,
        rect: NSRect,
        options: NSTrackingAreaOptions,
        owner: id,
        userInfo: id,
    ) -> id {
        msg_send![self, initWithRect:rect options:options owner:owner userInfo:userInfo]
    }
}

pub trait NSView: Sized {
    unsafe fn addTrackingArea(self, trackingArea: id) -> id;
}

impl NSView for id {
    unsafe fn addTrackingArea(self, trackingArea: id) -> id {
        msg_send![self, addTrackingArea: trackingArea]
    }
}
