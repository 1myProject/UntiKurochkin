use std::ffi::OsString;
use std::os::windows::prelude::*;
use windows_sys::Win32::Foundation::{HWND, RECT};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowRect, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
};

struct Data {
    name: String,
    rect: RECT,
}

impl Data {
    fn convert(self) -> (i32, i32) {
        (self.rect.left, self.rect.top)
    }
}

pub fn get_pos_was_saved() -> (i32, i32) {
    let mut data: Data = Data {
        name: "Запись данных".to_string(),
        rect: Default::default(),
    };
    let lparam = &mut data as *mut Data as isize;

    unsafe {
        EnumWindows(Some(enum_windows_callback), lparam);
    }

    data.convert()
}

pub fn get_pos_open_table() -> (i32, i32) {
    let mut data: Data = Data {
        name: "Форма для заполнения таблиц".to_string(),
        rect: Default::default(),
    };
    let lparam = &mut data as *mut Data as isize;

    unsafe {
        EnumWindows(Some(enum_windows_callback), lparam);
    }

    data.convert()
}

pub fn get_pos_maket() -> (i32, i32) {
    let mut data: Data = Data {
        name: "Виртуальная лабораторная работа \"Преобразователи частота\"".to_string(),
        rect: Default::default(),
    };
    let lparam = &mut data as *mut Data as isize;

    unsafe {
        EnumWindows(Some(enum_windows_callback), lparam);
    }

    data.convert()
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
                    let dt = &mut *(lparam as *mut Data);
                    if title != dt.name {
                        return 1;
                    }

                    let mut rect = RECT::default();
                    if GetWindowRect(hwnd, &mut rect) != 0 {
                        dt.rect = rect;
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
