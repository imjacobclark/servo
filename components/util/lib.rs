/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![cfg_attr(feature = "servo", feature(core_intrinsics))]
#![cfg_attr(feature = "servo", feature(custom_derive))]
#![cfg_attr(feature = "servo", feature(fnbox))]
#![cfg_attr(feature = "servo", feature(plugin))]
#![cfg_attr(feature = "servo", feature(reflect_marker))]
#![cfg_attr(feature = "servo", plugin(serde_macros))]

#![deny(unsafe_code)]

extern crate app_units;
#[cfg(feature = "servo")] extern crate backtrace;
#[allow(unused_extern_crates)] #[macro_use] extern crate bitflags;
extern crate deque;
extern crate euclid;
extern crate getopts;
#[macro_use] extern crate heapsize;
#[cfg(feature = "servo")] extern crate ipc_channel;
#[allow(unused_extern_crates)] #[macro_use] extern crate lazy_static;
extern crate libc;
#[macro_use] extern crate log;
extern crate num_cpus;
extern crate num_traits;
extern crate rand;
extern crate rustc_serialize;
#[cfg(feature = "servo")] extern crate serde;
extern crate smallvec;
extern crate url;
#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
extern crate xdg;

use std::sync::Arc;

pub mod basedir;
pub mod cache;
#[allow(unsafe_code)] pub mod debug_utils;
pub mod geometry;
#[cfg(feature = "servo")] #[allow(unsafe_code)] pub mod ipc;
#[allow(unsafe_code)] pub mod opts;
#[cfg(feature = "servo")] pub mod panicking;
pub mod prefs;
#[cfg(feature = "servo")] pub mod print_tree;
pub mod resource_files;
pub mod str;
pub mod thread;
pub mod thread_state;
pub mod tid;
pub mod vec;
#[allow(unsafe_code)] pub mod workqueue;

#[cfg(feature = "servo")]
#[allow(unsafe_code)]
pub fn breakpoint() {
    unsafe { ::std::intrinsics::breakpoint() };
}

// Workaround for lack of `ptr_eq` on Arcs...
#[inline]
pub fn arc_ptr_eq<T: 'static>(a: &Arc<T>, b: &Arc<T>) -> bool {
    let a: &T = &**a;
    let b: &T = &**b;
    (a as *const T) == (b as *const T)
}
