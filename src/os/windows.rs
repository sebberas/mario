use sdl2::{video, VideoSubsystem};
use windows::core::*;
use windows::w;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::os::MenuItem;

pub struct Menu {
    handle: HMENU,
}

pub fn new_menu(items: Vec<MenuItem>) -> Menu {
    let mut handle = unsafe { CreateMenu().unwrap() };
    for (i, item) in items.iter().enumerate() {
        let menu_title = PWSTR(w!("MENU Man").as_ptr() as *mut _);
        let menu_item = MENUITEMINFOW {
            cbSize: std::mem::size_of::<MENUITEMINFOW>() as _,
            fMask: MENU_ITEM_MASK(0) | MIIM_STRING,
            fType: MFT_STRING,
            fState: MENU_ITEM_STATE(0),
            wID: 0,
            hSubMenu: HMENU::default(),
            hbmpChecked: HBITMAP::default(),
            hbmpUnchecked: HBITMAP::default(),
            dwItemData: 0,
            dwTypeData: menu_title,
            cch: 0,
            hbmpItem: HBITMAP::default(),
        };

        unsafe { InsertMenuItemW(handle, 0, false, &menu_item) };
    }

    todo!()
}

pub struct Window {
    instance: HINSTANCE,
    handle: HWND,
}

impl Window {
    unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
        DefWindowProcW(hwnd, msg, wp, lp)
    }

    pub fn new(title: &str) -> Self {
        let instance = unsafe { GetModuleHandleW(PCWSTR::null()).unwrap() };

        let wcex = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as _,
            style: CS_OWNDC,
            lpfnWndProc: Some(Self::wndproc),
            hInstance: instance,
            lpszClassName: w!("mario"),
            ..Default::default()
        };

        assert_ne!(unsafe { RegisterClassExW(&wcex) }, 0);

        let handle = unsafe {
            CreateWindowExW(
                WS_EX_APPWINDOW,
                w!("mario"),
                w!("Mario"),
                WS_OVERLAPPEDWINDOW,
                0,
                0,
                640,
                480,
                HWND::default(),
                HMENU::default(),
                instance,
                None,
            )
        };

        Self { instance, handle }
    }

    pub fn set_menu(&mut self, menu: &Menu) {
        unsafe { SetMenu(self.handle, menu.handle) };
    }

    pub fn into_sdl2(self, subsystem: VideoSubsystem) -> video::Window {
        // SAFETY:
        unsafe { video::Window::from_ll(subsystem, self.handle.0 as *mut _) }
    }
}
