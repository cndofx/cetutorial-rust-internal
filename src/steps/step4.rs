use crate::mem::deref_pointer_path;

pub unsafe fn write_step4(base: usize) {
    if let Some(health_addr) = deref_pointer_path(base + 0x00325AA0, &[0x818]) {
        let health_addr = health_addr as *mut f32;
        *health_addr = 5000.0;
    }
    if let Some(ammo_addr) = deref_pointer_path(base + 0x00325AA0, &[0x820]) {
        let ammo_addr = ammo_addr as *mut f64;
        *ammo_addr = 5000.0;
    }
}
