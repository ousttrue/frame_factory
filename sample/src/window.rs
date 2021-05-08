use std::os::windows::ffi::OsStrExt;
use std::{ffi::OsStr, mem, ptr};

use winapi::shared::minwindef::*;
use winapi::{
    shared::windef,
    um::{libloaderapi, winuser},
};

fn utf16_str(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
}

unsafe extern "system" fn wndproc(
    window: windef::HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        winuser::WM_DESTROY => {
            winuser::PostQuitMessage(0);
            0
        }
        _ => winuser::DefWindowProcW(window, msg, wparam, lparam),
    }
}

pub fn create_window(class_name: &str, window_name: &str) -> Result<windef::HWND, &'static str> {
    unsafe {
        let class_name = utf16_str(class_name);
        let wcex = winuser::WNDCLASSEXW {
            cbSize: mem::size_of::<winuser::WNDCLASSEXW>() as u32,
            style: winuser::CS_HREDRAW | winuser::CS_VREDRAW | winuser::CS_OWNDC,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: libloaderapi::GetModuleHandleW(ptr::null()),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: ptr::null_mut(),
        };
        if winuser::RegisterClassExW(&wcex) == 0 {
            return Err("RegisterClassExWS");
        }

        let window_name = utf16_str(window_name);
        let hwnd = winuser::CreateWindowExW(
            winuser::WS_EX_APPWINDOW,
            class_name.as_ptr(),
            window_name.as_ptr(),
            winuser::WS_OVERLAPPEDWINDOW,
            winuser::CW_USEDEFAULT,
            winuser::CW_USEDEFAULT,
            1280,
            720,
            ptr::null_mut(),
            ptr::null_mut(),
            libloaderapi::GetModuleHandleW(ptr::null()),
            ptr::null_mut(),
        );
        if hwnd.is_null() {
            return Err("CreateWindowExW");
        }

        winuser::ShowWindow(hwnd, winuser::SW_SHOW);
        winuser::UpdateWindow(hwnd);
        Ok(hwnd)
    }
}

pub struct FrameIter {
    pub hwnd: windef::HWND,
}

impl Iterator for FrameIter {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut msg: winuser::MSG = mem::zeroed();
            while winuser::PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, winuser::PM_REMOVE) > 0 {
                if msg.message == winuser::WM_QUIT {
                    return None;
                };
                winuser::TranslateMessage(&msg);
                winuser::DispatchMessageW(&msg);
            }
        }

        Some(())
    }
}

pub fn main_loop(hwnd: windef::HWND) -> FrameIter {
    FrameIter { hwnd }
}
