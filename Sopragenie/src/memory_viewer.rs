use std::io::{self, Write};
use std::mem::{size_of, zeroed};
use std::process::exit;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::Debug::{
    ReadProcessMemory,
    // WriteProcessMemory,
};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
    MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};

const BASE: usize = 0x0049B000;
const LCO_ADRR: usize = BASE + 0x068;
const LC_ADRR: usize = LCO_ADRR - 0x4;
const CDO_ADRR: usize = BASE + 0x08C;
const CD_ADRR: usize = CDO_ADRR - 0x4;
const LGO_ADRR: usize = BASE + 0x070;
// const LG_ADRR: usize = LGO_ADRR-0x4;
const CPOSO_ADRR: usize = BASE + 0x094;
const CPOS_ADRR: usize = CPOSO_ADRR - 0x4;
const CPARO_ADRR: usize = BASE + 0x0A0;
const CPAR_ADRR: usize = CPARO_ADRR - 0x4;

// const ERR_ADRR: usize = BASE + 0x10C;
// const SCALE_ADRR: usize = BASE + 0x12C;
const DIAP_ADRR: usize = BASE + 0x036;

pub struct Meme {
    handle: HANDLE,
}

impl Meme {
    pub fn new() -> Self {
        const PROC_NAME: &str = "LabSopragenie.exe";

        println!("Ищу процесс: {}", PROC_NAME);

        let pid = loop {
            match find_process_id(PROC_NAME) {
                Some(id) => break id,
                None => {
                    eprintln!("Процесс '{}' не найден", PROC_NAME);
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

        let _base = match get_module_base(pid, PROC_NAME) {
            Some(addr) => addr,
            None => {
                eprintln!(
                    "Модуль '{}' не найден в процессе {} (мэйби надо запустить с админкой)",
                    PROC_NAME, pid
                );
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

    #[inline]
    pub fn lco(&self) -> f32 {
        let lco: f32 = self.read(LCO_ADRR);
        const ZERS: f32 = 1E9;
        (lco * ZERS).round() / ZERS
    }
    #[inline]
    pub fn lc(&self) -> f32 {
        self.read(LC_ADRR)
    }
    #[inline]
    pub fn cdo(&self) -> f32 {
        let cdo: f32 = self.read(CDO_ADRR);
        const ZERS: f32 = 1E14;
        (cdo * ZERS).round() / ZERS
    }
    #[inline]
    pub fn cd(&self) -> f32 {
        self.read(CD_ADRR)
    }
    #[inline]
    pub fn lgo(&self) -> f32 {
        let lgo: f32 = self.read(LGO_ADRR);
        const ZERS: f32 = 1E9;
        (lgo * ZERS).round() / ZERS
    }
    // #[inline]
    // pub fn lg(&self) -> f32{self.read(LG_ADRR)}
    #[inline]
    pub fn cposo(&self) -> f32 {
        let cposo: f32 = self.read(CPOSO_ADRR);

        let zers = if self.cpar() == 0.0 { 1E13 } else { 1E12 };
        (cposo * zers).round() / zers
    }
    #[inline]
    pub fn cpos(&self) -> f32 {
        self.read(CPOS_ADRR)
    }
    #[inline]
    pub fn cparo(&self) -> f32 {
        let cparo: f32 = self.read(CPARO_ADRR);

        let zers = if self.cpos() == 100.0 { 1E13 } else { 1E14 };
        (cparo * zers).round() / zers
    }
    #[inline]
    pub fn cpar(&self) -> f32 {
        self.read(CPAR_ADRR)
    }

    // #[inline]
    // pub fn err(&self) -> f32 {self.read(ERR_ADRR)}
    // #[inline]
    // pub fn scale(&self) -> f32 {self.read(SCALE_ADRR)}
    #[inline]
    pub fn diap(&self) -> i32 {
        self.read::<i16>(DIAP_ADRR) as i32
    }

    fn read<T: Default>(&self, addr: usize) -> T {
        let mut a: T = Default::default();
        read(self.handle, addr, &mut a);
        a
    }
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
                    if !ret.is_none() {
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

fn get_module_base(pid: u32, proc_name: &str) -> Option<usize> {
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
                if name.trim_end_matches('\0').eq_ignore_ascii_case(proc_name) {
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

fn read<T: ?Sized>(h_process: HANDLE, addr: usize, buf: &mut T) {
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

// fn write<T>(h_process: HANDLE, addr: usize, buf: &T) {
//     let bytes = size_of_val(buf);
//     let mut bytes_read: usize = 0;
//
//     #[cfg(debug_assertions)]
//     println!("записываю по адресу 0x{:X}", addr);
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
