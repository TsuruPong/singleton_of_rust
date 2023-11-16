use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;

use crate::domain::outer_struct::OuterStruct;

pub static ARC_RWLOCK: Lazy<Arc<RwLock<OuterStruct>>> = Lazy::new(|| Arc::new(RwLock::new(OuterStruct::new(5))));