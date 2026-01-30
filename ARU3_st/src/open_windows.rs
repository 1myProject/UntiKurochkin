use std::ffi::OsString;
use std::os::windows::prelude::*;
use windows_sys::Win32::Foundation::{HWND, RECT};
use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, GetForegroundWindow, GetWindowRect, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible};

#[derive(Default)]
pub struct WinInf {
    name: String,
    rect: RECT,
    pub is_active: bool,
}

impl WinInf {
    #[inline]
    pub fn pos(self) -> (i32, i32) {
        (self.rect.left, self.rect.top)
    }
}

#[inline]
pub fn get_pos_was_saved() -> WinInf {
    get_pos_window("Запись данных")
}
#[inline]
pub fn get_pos_open_table() -> WinInf {
    get_pos_window("Результаты измерения")
}

#[inline]
pub fn get_pos_maket() -> WinInf {
    get_pos_window("Виртуальная лабораторная работа \"Автоматическая регулировка усиления\"")
}

fn get_pos_window(name: &str) -> WinInf {
    let mut data: WinInf = WinInf {
        name: name.to_string(),
        ..Default::default()
    };
    let lparam = &mut data as *mut WinInf as isize;

    unsafe {
        EnumWindows(Some(enum_windows_callback), lparam);
    }

    data
}

extern "system" fn enum_windows_callback(hwnd: HWND, lparam: isize) -> i32 {
    unsafe {
        if IsWindowVisible(hwnd) != 0 {
            let length = GetWindowTextLengthW(hwnd);
            if length > 0 {
                let mut buffer = vec![0u16; (length + 1) as usize];
                GetWindowTextW(hwnd, buffer.as_mut_ptr(), length + 1);

                let title = OsString::from_wide(&buffer[0..length as usize])
                    .to_string_lossy()
                    .into_owned();

                #[cfg(test)]
                println!("{}", title);

                #[cfg(not(test))]
                {
                    let dt = &mut *(lparam as *mut WinInf);
                    if title != dt.name {
                        return 1;
                    }

                    let mut rect = RECT::default();
                    if GetWindowRect(hwnd, &mut rect) != 0 {
                        dt.rect = rect;

                        // Проверяем, является ли это окно активным
                        let foreground_hwnd = GetForegroundWindow();
                        dt.is_active = hwnd == foreground_hwnd;

                        return 0;
                    }
                }
            }
        }
    }
    1 // Возвращаем 1 для продолжения перечисления окон
}

#[test]
fn all_tabls() {
    unsafe {
        EnumWindows(Some(enum_windows_callback), 0);
    }
}
