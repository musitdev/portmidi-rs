// Copyright 2014-2015 Sam Doshi (sam@metal-fish.co.uk)
// Copyright 2013-2014 Philippe Delrieu (philippe.delrieu@free.fr)
//
// Licensed under either of
//          Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
//          MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT).
// This file may not be copied, modified, or distributed except according to those terms.
use std::ptr;
use std::os::raw::c_char;

mod ffi;
mod device;
pub use device::*;
mod io;
pub use io::*;
mod global;
pub use global::*;

pub use ffi::PmError;
pub mod types;
pub use types::*;

pub const HDRLENGTH: i32 = 50;
pub const PM_HOST_ERROR_MSG_LEN: i32 = 256;
