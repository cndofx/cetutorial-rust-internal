use std::{ffi::c_void, time::Duration};

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{BOOL, HINSTANCE},
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA},
            SystemServices::DLL_PROCESS_ATTACH,
        },
        UI::Input::KeyboardAndMouse::{
            GetAsyncKeyState, VIRTUAL_KEY, VK_END, VK_NUMPAD2, VK_NUMPAD3, VK_NUMPAD4, VK_NUMPAD5,
            VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8,
        },
    },
};

use mem::{deref_pointer_path, nop, patch};

mod mem;

struct HModule(HINSTANCE);

unsafe impl Send for HModule {}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(hmodule: HINSTANCE, call_reason: u32, _: *const ()) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            let hmodule = HModule(hmodule);
            std::thread::spawn(move || unsafe { main(hmodule) });
        }
        _ => {}
    }

    BOOL(1)
}

macro_rules! pcstr {
    ($str:expr) => {
        PCSTR(concat!($str, '\0').as_ptr())
    };
}

unsafe fn main(hmodule: HModule) {
    let Ok(module_base) = GetModuleHandleA(pcstr!("Tutorial-x86_64.exe")) else {
        return;
    };
    let module_base = module_base.0 as usize;

    let Ok(_) = AllocConsole() else {
        return;
    };

    print_menu();
    let mut step2_active = false;
    let mut step3_active = false;
    let mut step4_active = false;
    let mut step5_active = false;
    let mut step6_active = false;
    let mut step7_active = false;
    let mut step8_active = false;

    let mut running = true;
    while running {
        // INPUT HANDLING
        if pressed(VK_NUMPAD2) {
            if step2_active {
                step2_active = false;
                println!("Step 2 deactivated");
            } else {
                step2_active = true;
                println!("Step 2 activated");
            }
        }

        if pressed(VK_NUMPAD3) {
            if step3_active {
                step3_active = false;
                println!("Step 3 deactivated");
            } else {
                step3_active = true;
                println!("Step 3 activated");
            }
        }

        if pressed(VK_NUMPAD4) {
            if step4_active {
                step4_active = false;
                println!("Step 4 deactivated");
            } else {
                step4_active = true;
                println!("Step 4 activated");
            }
        }

        if pressed(VK_NUMPAD5) {
            if step5_active {
                restore_step5(module_base);
                step5_active = false;
                println!("Step 5 deactivated");
            } else {
                nop_step5(module_base);
                step5_active = true;
                println!("Step 5 activated");
            }
        }

        if pressed(VK_NUMPAD6) {
            if step6_active {
                step6_active = false;
                println!("Step 6 deactivated");
            } else {
                step6_active = true;
                println!("Step 6 activated");
            }
        }

        if pressed(VK_NUMPAD7) {
            if step7_active {
                restore_step7(module_base);
                step7_active = false;
                println!("Step 7 deactivated");
            } else {
                patch_step7(module_base);
                step7_active = true;
                println!("Step 7 activated");
            }
        }

        if pressed(VK_NUMPAD8) {
            if step8_active {
                step8_active = false;
                println!("Step 8 deactivated");
            } else {
                step8_active = true;
                println!("Step 8 activated");
            }
        }

        if pressed(VK_END) {
            running = false;
        }

        // END INPUT HANDLING

        if step2_active {
            write_step2(module_base);
        }

        if step3_active {
            write_step3(module_base);
        }

        if step4_active {
            write_step4(module_base);
        }

        // step 5 is handled in the key press event

        if step6_active {
            write_step6(module_base);
        }

        // step 7 is handled in the key press event

        if step8_active {
            write_step8(module_base);
        }

        std::thread::sleep(Duration::from_millis(50));
    }

    let Ok(_) = FreeConsole() else {
        return;
    };
    FreeLibraryAndExitThread(hmodule.0, 0);
}

unsafe fn write_step2(base: usize) {
    if let Some(health_addr) = deref_pointer_path(base + 0x00325A70, &[0x7F8]) {
        let health_addr = health_addr as *mut u32;
        *health_addr = 1000;
    }
}

unsafe fn write_step3(base: usize) {
    if let Some(health_addr) = deref_pointer_path(base + 0x00325A80, &[0x7F8]) {
        let health_addr = health_addr as *mut u32;
        *health_addr = 5000;
    }
}

unsafe fn write_step4(base: usize) {
    if let Some(health_addr) = deref_pointer_path(base + 0x00325AA0, &[0x818]) {
        let health_addr = health_addr as *mut f32;
        *health_addr = 5000.0;
    }
    if let Some(ammo_addr) = deref_pointer_path(base + 0x00325AA0, &[0x820]) {
        let ammo_addr = ammo_addr as *mut f64;
        *ammo_addr = 5000.0;
    }
}

unsafe fn nop_step5(base: usize) {
    let addr = base + 0x0002CB88;
    if let Ok(_) = nop(addr as *mut c_void, 2) {
        println!("nopped instructions");
    } else {
        println!("unable to nop instructions");
    }
}

unsafe fn restore_step5(base: usize) {
    let addr = base + 0x0002CB88;
    if let Ok(_) = patch(addr as *mut c_void, &[0x89, 0x10]) {
        println!("restored instructions");
    } else {
        println!("unable to restore instructions");
    }
}

unsafe fn write_step6(base: usize) {
    if let Some(addr) = deref_pointer_path(base + 0x00325AD0, &[0x0]) {
        let addr = addr as *mut u32;
        *addr = 5000;
    }
}

unsafe fn patch_step7(base: usize) {
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

unsafe fn restore_step7(base: usize) {
    let addr = base + 0x0002DB57;
    if let Ok(_) = patch(
        addr as *mut c_void,
        &[0x83, 0xAE, 0xE0, 0x07, 0x00, 0x00, 0x01],
    ) {
        println!("restored instructions");
    } else {
        println!("unable to restore instructions");
    }
}

unsafe fn write_step8(base: usize) {
    if let Some(addr) = deref_pointer_path(base + 0x00325B00, &[0x10, 0x18, 0x0, 0x18]) {
        let addr = addr as *mut u32;
        *addr = 5000;
    }
}

fn print_menu() {
    println!("Numpad 2: Solve step 2");
    println!("Numpad 3: Solve step 3");
    println!("Numpad 4: Solve step 4");
    println!("Numpad 5: Solve step 5");
    println!("Numpad 6: Solve step 6");
    println!("Numpad 7: Solve step 7");
    println!("Numpad 8: Solve step 8");
}

fn pressed(key: VIRTUAL_KEY) -> bool {
    unsafe { GetAsyncKeyState(key.0 as i32) & 1 == 1 }
}
