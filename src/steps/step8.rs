use crate::mem::deref_pointer_path;

pub unsafe fn write_step8(base: usize) {
    if let Some(addr) = deref_pointer_path(base + 0x00325B00, &[0x10, 0x18, 0x0, 0x18]) {
        let addr = addr as *mut u32;
        *addr = 5000;
    }
}
