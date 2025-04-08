use std::ffi::{c_void, CString};

use windows::{core::PCSTR, Win32::{Foundation::*, System::SystemServices::*, UI::WindowsAndMessaging::{MessageBoxA, MB_OK}}};

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(dll_module: HINSTANCE, fdw_reason: u32, _: *mut ()) -> bool {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {  // DLL_PROCESS_ATTACH
            println!("attach!");
            test_func("attach");
        }
        DLL_PROCESS_DETACH => {
            println!("detach!");
            test_func("detach");
        }
        _ => {
        
        }
    }
    true
}

fn test_func(value: &str) {
    unsafe {
        let content = CString::new(value).unwrap();
        let title = CString::new("title").unwrap();

        MessageBoxA(None, PCSTR(content.as_ptr() as *const u8), PCSTR(title.as_ptr() as *const u8), MB_OK);
    }
}