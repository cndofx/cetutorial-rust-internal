use std::ffi::c_void;

use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

pub unsafe fn deref_pointer_path(base: usize, offsets: &[usize]) -> Option<usize> {
    let mut addr = base;
    for offset in offsets {
        addr = *(addr as *const usize);
        if addr == 0 {
            return None;
        }
        addr += offset;
    }
    Some(addr)
}

pub unsafe fn nop(dest: *mut c_void, size: usize) -> windows::core::Result<()> {
    let mut old = PAGE_PROTECTION_FLAGS(0);

    VirtualProtect(dest, size, PAGE_EXECUTE_READWRITE, &mut old as *mut _)?;

    dest.write_bytes(0x90, size);

    VirtualProtect(dest, size, old, &mut old as *mut _)?;

    Ok(())
}
