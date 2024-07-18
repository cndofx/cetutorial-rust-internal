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
        },
    },
};

use mem::{deref_pointer_path, nop};

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

unsafe fn main(hmodule: HModule) {
    let Ok(module_base) = GetModuleHandleA(pcstr("Tutorial-x86_64.exe")) else {
        return;
    };
    let module_base = module_base.0 as usize;

    let Ok(_) = AllocConsole() else {
        return;
    };

    print_menu();

    let mut running = true;
    while running {
        if pressed(VK_NUMPAD2) {
            solve_step2(module_base);
        }

        if pressed(VK_NUMPAD3) {
            solve_step3(module_base);
        }

        if pressed(VK_NUMPAD4) {
            solve_step4(module_base);
        }

        if pressed(VK_NUMPAD5) {
            solve_step5(module_base);
        }

        if pressed(VK_END) {
            running = false;
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    let Ok(_) = FreeConsole() else {
        return;
    };
    FreeLibraryAndExitThread(hmodule.0, 0);
}

unsafe fn solve_step2(base: usize) {
    println!("solving step 2");
    if let Some(health_addr) = deref_pointer_path(base + 0x00325A70, &[0x7F8]) {
        println!("{health_addr:#010X}");
        let health_addr = health_addr as *mut u32;
        println!("health before: {}", *health_addr);
        *health_addr = 1000;
        println!("health after : {}", *health_addr);
    } else {
        println!("null pointer");
    }
}

unsafe fn solve_step3(base: usize) {
    println!("solving step 3");
    if let Some(health_addr) = deref_pointer_path(base + 0x00325A80, &[0x7F8]) {
        println!("{health_addr:#010X}");
        let health_addr = health_addr as *mut u32;
        println!("health before: {}", *health_addr);
        *health_addr = 5000;
        println!("health after : {}", *health_addr);
    } else {
        println!("null pointer");
    }
}

unsafe fn solve_step4(base: usize) {
    println!("solving step 4");
    if let Some(health_addr) = deref_pointer_path(base + 0x00325AA0, &[0x818]) {
        println!("{health_addr:#010X}");
        let health_addr = health_addr as *mut f32;
        println!("health before: {}", *health_addr);
        *health_addr = 5000.0;
        println!("health after : {}", *health_addr);
    } else {
        println!("null pointer");
    }
    if let Some(ammo_addr) = deref_pointer_path(base + 0x00325AA0, &[0x820]) {
        println!("{ammo_addr:#010X}");
        let ammo_addr = ammo_addr as *mut f64;
        println!("ammo before: {}", *ammo_addr);
        *ammo_addr = 5000.0;
        println!("ammo after : {}", *ammo_addr);
    } else {
        println!("null pointer");
    }
}

unsafe fn solve_step5(base: usize) {
    println!("solving step 5");
    let addr = base + 0x0002CB88;
    if let Ok(_) = nop(addr as *mut c_void, 2) {
        println!("nopped instructions");
    } else {
        println!("unable to nop instructions");
    }
}

fn print_menu() {
    println!("Numpad 2: Solve step 2");
    println!("Numpad 3: Solve step 3");
    println!("Numpad 4: Solve step 4");
    println!("Numpad 5: Solve step 5");
}

fn pcstr(str: &str) -> PCSTR {
    PCSTR(format!("{str}\0").as_ptr())
}

fn pressed(key: VIRTUAL_KEY) -> bool {
    unsafe { GetAsyncKeyState(key.0 as i32) & 1 == 1 }
}
