use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::domain::outer_struct::OuterStruct;

pub static mut UNSAFE_MUTEX: Lazy<Mutex<OuterStruct>> = Lazy::new(|| Mutex::new(OuterStruct::new(1)));