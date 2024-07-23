use std::ffi::c_void;

use crate::mem::patch;

pub unsafe fn patch_step7(base: usize) {
    let addr = base + 0x0002DB57;
    if let Ok(_) = patch(
        addr as *mut c_void,
        &[0x83, 0x86, 0xE0, 0x07, 0x00, 0x00, 0x02],
    ) {
        println!("patched instructions");
    } else {
        println!("unable to patch instructions");
    }
}

pub unsafe fn restore_step7(base: usize) {
    let addr = base + 0x0002DB57;
    let patch = patch(
        addr as *mut c_void,
        &[0x83, 0xAE, 0xE0, 0x07, 0x00, 0x00, 0x01],
    );
    if let Ok(_) = patch {
        println!("restored instructions");
    } else {
        println!("unable to restore instructions");
    }
}
