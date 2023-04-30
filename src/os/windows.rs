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
    fn new_menu_recursive(parent: HMENU, items: Vec<MenuItem>, id: &mut u32) {
        for (i, item) in items.into_iter().enumerate() {
            match item {
                MenuItem::Action { title, action } => {
                    let title = title.encode_utf16().chain([0]).collect::<Vec<_>>();
                    let item = MENUITEMINFOW {
                        cbSize: std::mem::size_of::<MENUITEMINFOW>() as _,
                        fMask: MENU_ITEM_MASK(0) | MIIM_ID | MIIM_STRING,
                        fType: MFT_STRING,
                        fState: MENU_ITEM_STATE(0),
                        wID: *id,
                        hSubMenu: HMENU::default(),
                        hbmpChecked: HBITMAP::default(),
                        hbmpUnchecked: HBITMAP::default(),
                        dwItemData: 0,
                        dwTypeData: PWSTR::from_raw(title.as_ptr() as *mut _),
                        cch: 0,
                        hbmpItem: HBITMAP::default(),
                    };

                    *id += 1;

                    unsafe { InsertMenuItemW(parent, i as _, true, &item) };
                }
                MenuItem::Divider => {}
                MenuItem::SubMenu { title, items } => {
                    // Handle submenu
                    let sub = unsafe { CreateMenu().unwrap() };
                    new_menu_recursive(sub, items, id);

                    let title = title.encode_utf16().chain([0]).collect::<Vec<_>>();
                    let item = MENUITEMINFOW {
                        cbSize: std::mem::size_of::<MENUITEMINFOW>() as _,
                        fMask: MENU_ITEM_MASK(0) | MIIM_SUBMENU | MIIM_STRING,
                        fType: MFT_STRING,
                        fState: MENU_ITEM_STATE(0),
                        wID: 0,
                        hSubMenu: sub,
                        hbmpChecked: HBITMAP::default(),
                        hbmpUnchecked: HBITMAP::default(),
                        dwItemData: 0,
                        dwTypeData: PWSTR::from_raw(title.as_ptr() as *mut _),
                        cch: 0,
                        hbmpItem: HBITMAP::default(),
                    };

                    unsafe { InsertMenuItemW(parent, i as _, true, &item) };
                }
            }
        }
    }

    let mut id = 1;
    let handle = unsafe { CreateMenu().unwrap() };
    new_menu_recursive(handle, items, &mut id);

    Menu { handle }
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

        let title: Vec<_> = title.encode_utf16().chain([0]).collect();
        let title = PCWSTR::from_raw(title.as_ptr());

        let handle = unsafe {
            CreateWindowExW(
                WS_EX_APPWINDOW,
                w!("mario"),
                title,
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

        println!("got here");

        Self { instance, handle }
    }

    pub fn set_menu(&mut self, menu: &Menu) {
        unsafe { SetMenu(self.handle, menu.handle) };
    }

    pub fn into_sdl2(self, subsystem: VideoSubsystem) -> video::Window {
        // SAFETY:
        let raw = unsafe { sdl2::sys::SDL_CreateWindowFrom(self.handle.0 as *mut _) };
        unsafe { video::Window::from_ll(subsystem, raw) }
    }
}
