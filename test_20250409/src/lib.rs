use hooking_api::utils::{get_filename_by_module, get_module_list};

use windows::{core::PCSTR, Win32::{Foundation::*, System::{SystemServices::*, Threading::GetCurrentProcess}, UI::WindowsAndMessaging::{MessageBoxA, MB_OK}}};
use windows::Win32::System::Diagnostics::Debug::OutputDebugStringA;

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(dll_module: HINSTANCE, fdw_reason: u32, _: *mut ()) -> bool {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {  // DLL_PROCESS_ATTACH
            test_func();
        }
        DLL_PROCESS_DETACH => {
            
        }
        _ => {
        
        }
    }
    true
}

pub fn test_func() {
    let vec = get_module_list(None).unwrap();
    unsafe {
        let handle = GetCurrentProcess();
        for module in vec {
            let moduel_name = get_filename_by_module(handle, module).unwrap();
            let debug_str = format!("[hook_test] {}", moduel_name);
            OutputDebugStringA(PCSTR(debug_str.as_ptr()));
        }
    }
}