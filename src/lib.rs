// Copyright 2014-2015 Sam Doshi (sam@metal-fish.co.uk)
// Copyright 2013-2014 Philippe Delrieu (philippe.delrieu@free.fr)
//
// Licensed under either of
//          Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
//          MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT).
// This file may not be copied, modified, or distributed except according to those terms.
mod device;
mod ffi;
mod vdevice;
pub use device::*;
pub use vdevice::VirtualDevice;
mod io;
pub use io::*;

pub use ffi::PmError;
pub mod types;
pub use types::*;
mod context;
pub use context::*;

pub const HDRLENGTH: i32 = 50;
pub const PM_HOST_ERROR_MSG_LEN: i32 = 256;
