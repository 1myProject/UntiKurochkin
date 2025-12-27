use std::io::{self, Write};
use std::mem::{size_of, zeroed};
use std::process::exit;
use std::ptr::null_mut;
use std::{thread, time};
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, MODULEENTRY32, Module32First, Module32Next, PROCESSENTRY32,
    Process32First, Process32Next, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};

const SA_ADRR: usize = 0x00673114;
const VOLT_ADRR: usize = 0x00673044;
const FREQ_ADRR: usize = 0x0067307C;
// const FEQ_BAND_ADDR: usize = 0x007A1308;

pub struct Meme {
    handle: HANDLE,
}

impl Meme {
    pub fn new() -> Self {
        println!();

        let proc_name = "LabYPC2.exe";
        let module_name = proc_name;

        println!("Ищу процесс: {}", proc_name);

        let pid = loop {
            match find_process_id(proc_name) {
                Some(id) => break id,
                None => {
                    eprintln!("Процесс '{}' не найден", proc_name);
                    println!("перед тем как начнем. Запустите лабораторную");
                    press_enter_for_exit();
                    continue;
                    // press_enter_for_exit();
                    // exit(1);
                }
            }
        };
        #[cfg(debug_assertions)]
        println!("PID = {}", pid);

        let _base = match get_module_base(pid, module_name) {
            Some(addr) => addr,
            None => {
                eprintln!("Модуль '{}' не найден в процессе {} (мэйби надо запустить с админкой)", module_name, pid);
                press_enter_for_exit();
                exit(2);
            }
        };

        #[cfg(debug_assertions)]
        println!("Базовый адрес модуля = 0x{_base:X}");

        let h = unsafe {
            let h_process = OpenProcess(
                PROCESS_QUERY_INFORMATION
                    | PROCESS_VM_READ
                    | PROCESS_VM_WRITE
                    | PROCESS_VM_OPERATION,
                0,
                pid,
            );
            if h_process == null_mut() {
                eprintln!(
                    "Ошибка OpenProcess: {}",
                    windows_sys::Win32::Foundation::GetLastError()
                );
                exit(3);
            }
            h_process
        };

        Self { handle: h }
    }

    pub fn get_sa(&self) -> [i16; 10] {
        let mut arr = [0i16; 10];
        read(self.handle, SA_ADRR, &mut arr);

        arr
    }

    pub fn get_volts(&self)-> f32{
        thread::sleep(time::Duration::from_millis(1));

        let mut volt = 0f32;
        read(self.handle, VOLT_ADRR, &mut volt);

        volt
    }

    pub fn get_freq(&self) -> f32 {

        let mut freq = 0f32;
        read(self.handle, FREQ_ADRR, &mut freq);

        freq
    }

    // pub fn get_freq_band(&self) -> i16{
    //     let mut band = 0i16;
    //     read_val(self.handle, FEQ_BAND_ADDR, &mut band);

    //     band
    // }

    // pub fn set_freq(&self, val: f32) {
    //     write_val(self.handle, FREQ_ADRR, &val);
    // }
}

impl Drop for Meme {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}

fn find_process_id(proc_name: &str) -> Option<u32> {
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap == INVALID_HANDLE_VALUE {
            return None;
        }

        let mut entry: PROCESSENTRY32 = zeroed();
        entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

        let mut ret: Option<u32> = None;
        if Process32First(snap, &mut entry) != 0 {
            loop {
                let arr = std::mem::transmute::<&[i8; 260], &[u8; 260]>(&entry.szExeFile);
                let name = String::from_utf8_lossy(arr).to_string();
                // println!("{name}");
                if name.starts_with(proc_name) {
                    if ret != None {
                        CloseHandle(snap as HANDLE);
                        println!("запущено >1 процесса. Я не знаю от какого брать значения");
                        press_enter_for_exit();
                        exit(4);
                    }
                    ret = Some(entry.th32ProcessID);
                }

                if Process32Next(snap, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snap as HANDLE);
        ret
    }
}

fn get_module_base(pid: u32, module_name: &str) -> Option<usize> {
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid);
        if snap == INVALID_HANDLE_VALUE {
            return None;
        }

        let mut me32: MODULEENTRY32 = zeroed();
        me32.dwSize = size_of::<MODULEENTRY32>() as u32;

        if Module32First(snap, &mut me32) != 0 {
            loop {
                let arr = std::mem::transmute::<&[i8; 256], &[u8; 256]>(&me32.szModule);
                let name = String::from_utf8_lossy(arr);
                if name
                    .trim_end_matches('\0')
                    .eq_ignore_ascii_case(module_name)
                {
                    CloseHandle(snap as HANDLE);
                    return Some(me32.modBaseAddr as usize);
                }
                if Module32Next(snap, &mut me32) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snap as HANDLE);
    }
    None
}

fn read<T>(h_process: HANDLE, addr: usize, buf: &mut T) {
    let bytes = size_of_val(buf);
    let mut bytes_read: usize = 0;

    unsafe {
        let ok = ReadProcessMemory(
            h_process,
            addr as *const _,
            buf as *mut _ as *mut _,
            bytes,
            &mut bytes_read,
        );

        if ok == 0 || bytes_read != bytes_read {
            eprintln!(
                "Ошибка ReadProcessMemory: {}",
                windows_sys::Win32::Foundation::GetLastError()
            );
            press_enter_for_exit();
            exit(4);
        }
    }
}

// fn write_arr<T>(h_process: HANDLE, addr: usize, buf: &[T]) {
//     let bytes = size_of_val(buf);
//     let mut bytes_read: usize = 0;
//
//     unsafe {
//         let ok = WriteProcessMemory(
//             h_process,
//             addr as *const _,
//             buf as *const _ as *const _,
//             bytes,
//             &mut bytes_read,
//         );
//
//         if ok == 0 || bytes_read != bytes {
//             eprintln!(
//                 "Ошибка ReadProcessMemory: {}",
//                 windows_sys::Win32::Foundation::GetLastError()
//             );
//             press_enter_for_exit();
//             exit(4);
//         }
//     }
// }
// fn write_val<T>(h_process: HANDLE, addr: usize, buf: &T) {
//     let bytes = size_of_val(buf);
//     let mut bytes_read: usize = 0;
//
//     unsafe {
//         let ok = WriteProcessMemory(
//             h_process,
//             addr as *const _,
//             buf as *const _ as *const _,
//             bytes,
//             &mut bytes_read,
//         );
//
//         if ok == 0 || bytes_read != bytes {
//             eprintln!(
//                 "Ошибка ReadProcessMemory: {}",
//                 windows_sys::Win32::Foundation::GetLastError()
//             );
//             press_enter_for_exit();
//             exit(4);
//         }
//     }
// }

pub fn press_enter_for_exit() {
    // println!("Press Enter for exit...");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
}
