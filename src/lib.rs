use std::{io::Write, time::Duration};

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{BOOL, HINSTANCE, HWND},
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA},
            SystemServices::DLL_PROCESS_ATTACH,
        },
        UI::{
            Input::KeyboardAndMouse::{
                GetAsyncKeyState, VIRTUAL_KEY, VK_END, VK_NUMPAD1, VK_NUMPAD2,
            },
            WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE},
        },
    },
};

struct HModule(HINSTANCE);

unsafe impl Send for HModule {}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(hmodule: HINSTANCE, call_reason: u32, _: *const ()) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            let hmodule = HModule(hmodule);
            std::thread::spawn(move || main(hmodule));
        }
        _ => {}
    }

    BOOL(1)
}

fn main(hmodule: HModule) {
    unsafe {
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

fn print_menu() {
    println!("Numpad 2: Solve step 2");
    println!("Numpad 3: Solve step 3");
}

fn pcstr(str: &str) -> PCSTR {
    PCSTR(format!("{str}\0").as_ptr())
}

fn pressed(key: VIRTUAL_KEY) -> bool {
    unsafe { GetAsyncKeyState(key.0 as i32) & 1 == 1 }
}

unsafe fn deref_pointer_path(base: usize, offsets: &[usize]) -> Option<usize> {
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