use crate::mem::deref_pointer_path;

pub unsafe fn write_step6(base: usize) {
    if let Some(addr) = deref_pointer_path(base + 0x00325AD0, &[0x0]) {
        let addr = addr as *mut u32;
        *addr = 5000;
    }
}
