use std::sync::MutexGuard;

use mutable_singleton_test::{
    domain::outer_struct::OuterStruct, patterns::{
        p1::unsafe_mutex::UNSAFE_MUTEX,
        p2::ref_cell::REF_CELL,
        p3::arc_rwlock::ARC_RWLOCK
    }
};

fn main() {
    disp(1, || {
        unsafe {
            let mut outer_guard: MutexGuard<OuterStruct> = UNSAFE_MUTEX.lock().unwrap();
            outer_guard.show();
            outer_guard.update(2);
            outer_guard.show();
        }
    });

    disp(2, || {
        REF_CELL.with(|v| {
            v.borrow().show();
            *v.borrow_mut() = OuterStruct::new(4);
        });
        REF_CELL.with(|v| {
            v.borrow().show();
        });
    });

    disp(3, || {
        ARC_RWLOCK.read().unwrap().show();
        *ARC_RWLOCK.write().unwrap() = OuterStruct::new(6);
        ARC_RWLOCK.read().unwrap().show();
    })
}

fn disp<F: Fn()>(num: u8, f: F) {
    println!("+---------------p{} start---------------------------+",num);
    f();
    println!("+---------------p{} end-----------------------------+",num);
}
