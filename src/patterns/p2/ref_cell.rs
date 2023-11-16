use std::cell::RefCell;
use crate::domain::outer_struct::OuterStruct;

thread_local!(pub static REF_CELL: RefCell<OuterStruct> = RefCell::new(OuterStruct::new(3)));