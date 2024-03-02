use winapi::um::winuser::{CallNextHookEx, SetWindowsHookExA, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN};
use winapi::um::winuser::{GetMessageW, MSG, TranslateMessage, DispatchMessageW};
use std::ptr::null_mut;
use std::os::raw::c_int;

unsafe extern "system" fn keyboard_hook_proc(n_code: c_int, w_param: usize, l_param: isize) -> isize {
    if n_code >= 0 && w_param == WM_KEYDOWN as usize {
        let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
        let key = match kb_struct.vkCode {
            // Numbers
            0x30 => "0",
            0x31 => "1",
            0x32 => "2",
            0x33 => "3",
            0x34 => "4",
            0x35 => "5",
            0x36 => "6",
            0x37 => "7",
            0x38 => "8",
            0x39 => "9",
            // Letters
            0x41 => "A",
            0x42 => "B",
            0x43 => "C",
            0x44 => "D",
            0x45 => "E",
            0x46 => "F",
            0x47 => "G",
            0x48 => "H",
            0x49 => "I",
            0x4A => "J",
            0x4B => "K",
            0x4C => "L",
            0x4D => "M",
            0x4E => "N",
            0x4F => "O",
            0x50 => "P",
            0x51 => "Q",
            0x52 => "R",
            0x53 => "S",
            0x54 => "T",
            0x55 => "U",
            0x56 => "V",
            0x57 => "W",
            0x58 => "X",
            0x59 => "Y",
            0x5A => "Z",
            // Function Keys
            0x70 => "F1",
            0x71 => "F2",
            0x72 => "F3",
            0x73 => "F4",
            0x74 => "F5",
            0x75 => "F6",
            0x76 => "F7",
            0x77 => "F8",
            0x78 => "F9",
            0x79 => "F10",
            0x7A => "F11",
            0x7B => "F12",
            // Special Keys
            0x08 => "Backspace",
            0x09 => "Tab",
            0x0D => "Enter",
            0x20 => "Space",
            0x25 => "Left Arrow",
            0x26 => "Up Arrow",
            0x27 => "Right Arrow",
            0x28 => "Down Arrow",
            0x2E => "Delete",
            0x1B => "Escape",
            0x2C => "Print Screen",
            0x2D => "Insert",
            0x2F => "Help",
            0x5B => "Left Windows",
            0x5C => "Right Windows",
            0x5D => "Applications",
            0xA0 => "Shift",
            0xA1 => "Shift",
            0xA2 => "Ctrl",
            0xA3 => "Ctrl",
            _ => "Unknown",
        };
        println!("Key pressed: {}", key);
    }

    CallNextHookEx(null_mut(), n_code, w_param, l_param)
}

fn main() {
    unsafe {
        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_hook_proc), null_mut(), 0);
        if hook.is_null() {
            eprintln!("Failed to set keyboard hook");
            return;
        }

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        UnhookWindowsHookEx(hook);
    }
}
