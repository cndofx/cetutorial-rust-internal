use crate::mem::deref_pointer_path;

pub unsafe fn write_step3(base: usize) {
    if let Some(health_addr) = deref_pointer_path(base + 0x00325A80, &[0x7F8]) {
        let health_addr = health_addr as *mut u32;
        *health_addr = 5000;
    }
}
