use std::ffi::c_void;

use crate::mem::{nop, patch};

pub unsafe fn nop_step5(base: usize) {
    let addr = base + 0x0002CB88;
    if let Ok(_) = nop(addr as *mut c_void, 2) {
        println!("nopped instructions");
    } else {
        println!("unable to nop instructions");
    }
}

pub unsafe fn restore_step5(base: usize) {
    let addr = base + 0x0002CB88;
    if let Ok(_) = patch(addr as *mut c_void, &[0x89, 0x10]) {
        println!("restored instructions");
    } else {
        println!("unable to restore instructions");
    }
}
