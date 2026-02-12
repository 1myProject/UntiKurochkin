use std::io::{self, Write};
use std::mem::{size_of, zeroed};
use std::process::exit;
use std::ptr::null_mut;
use std::{thread, time};
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

const BASE: usize = 0x006D8000;
const SA_ADRR: usize = BASE + 0x168;
const VM_ADRR: usize = BASE + 0x1F0;
// const VM_MAX_ADRR: usize = BASE + 0x084;
const FV_ADRR: usize = BASE + 0x090;
const VG_ADRR: usize = BASE + 0x0D8;
const M_ADRR: usize = BASE + 0x0D0;
const FM_ADRR: usize = BASE + 0x0C8;
const Q_ADRR: usize = BASE + 0x1C0;
const FI_ADRR: usize = BASE + 0x1B4;

const Tust2_ADRR: usize = VM_ADRR + 0x11C;
const Tdspad_ADRR: usize = VM_ADRR + 0x10C;
const Forma_ADDR: usize = BASE + 0x390;
const Lk_ADDR: usize = Forma_ADDR + 0x264;
const Ck_ADDR: usize = Forma_ADDR + 0x268;
const Csv_ADDR: usize = Forma_ADDR + 0x27C;
const Qk_ADDR: usize = Forma_ADDR + 0x28C;

pub struct Meme {
    handle: HANDLE,
}

pub const SA_COUNT: usize = 7;
impl Meme {
    pub fn new() -> Self {
        const PROC_NAME: &str = "LabIM3.exe";

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

        let mem = Self { handle: h };

        if mem.tests() {
            println!("Мэйби Курочкин обновил прогу? Тогда делайте issue");
            press_enter_for_exit();
            exit(5);
        }

        // mem.set_vm_max();

        mem
    }

    fn tests(&self) -> bool {
        let mut f = false;
        if !(300_000.0..=10_000_000.).contains(&self.fv()) {
            println!("Подозрительная частота Генератора");
            f = true;
        }
        let fm = self.fm();
        if fm != 0. && !(10.0..=20_000.).contains(&fm) {
            println!("Подозрительная модулирующая частота Генератора");
            f = true;
        }
        if !(0.0..=1.).contains(&self.m()) {
            println!("Подозрительный процент модуляции");
            f = true;
        }
        if !(0.000_001..=1.).contains(&self.vg()) {
            println!("Подозрительная Амплитуда Генератора");
            f = true;
        }
        if !(1.0..=10.).contains(&self.q()) {
            println!("Подозрительная Амплитуда Генератора");
            f = true;
        }
        if !(1000..=50000).contains(&self.fi()) {
            println!("Подозрительная Амплитуда Генератора");
            f = true;
        }

        if !(10.0..=15.0).contains(&self.qk()) {
            println!("Подозрительная Qk");
            f = true;
        }
        if !(10.0e-6..=24.0e-6).contains(&self.lk()) {
            println!("Подозрительная Qk");
            f = true;
        }
        if !(100.0e-12..=200.0e-12).contains(&self.csv()) {
            println!("Подозрительная Qk");
            f = true;
        }
        if !(250.0e-11..=500.0e-11).contains(&self.ck()) {
            println!("Подозрительная Qk");
            f = true;
        }

        for (n, i) in self.sa().into_iter().enumerate() {
            if !(1..=5).contains(&i) {
                println!("Подозрительные переключатель SA{}", n + 1);
                f = true;
            }
        }
        f
    }

    #[inline]
    pub fn sa(&self) -> [i16; SA_COUNT] {
        self.read(SA_ADRR)
    }

    #[inline]
    pub fn vm(&self) -> f64 {
        #[cfg(not(debug_assertions))]
        thread::sleep(time::Duration::from_millis(5));

        #[cfg(debug_assertions)]
        thread::sleep(time::Duration::from_millis(1));

        self.read(VM_ADRR)
    }

    pub fn vm_round(&self) -> f64 {
        let volt = self.vm();
        const ZERS: f64 = 10_000.;
        (volt * ZERS).round() / ZERS
    }

    #[inline]
    pub fn fv(&self) -> f64 {
        self.read(FV_ADRR)
    }

    #[inline]
    pub fn vg(&self) -> f64 {
        self.read(VG_ADRR)
    }

    #[inline]
    pub fn m(&self) -> f64 {
        self.read(M_ADRR)
    }

    #[inline]
    pub fn fm(&self) -> f64 {
        self.read(FM_ADRR)
    }

    #[inline]
    pub fn q(&self) -> f32 {
        self.read(Q_ADRR)
    }

    #[inline]
    pub fn fi(&self) -> u32 {
        self.read(FI_ADRR)
    }

    #[inline]
    pub fn tust2(&self) -> f64 {
        self.read(Tust2_ADRR)
    }

    #[inline]
    pub fn tdspad(&self) -> f64 {
        self.read(Tdspad_ADRR)
    }

    #[inline]
    pub fn lk(&self) -> f64 {
        self.read::<f32>(Lk_ADDR) as f64
    }

    #[inline]
    pub fn ck(&self) -> f64 {
        self.read::<f32>(Ck_ADDR) as f64
    }

    #[inline]
    pub fn csv(&self) -> f64 {
        self.read::<f32>(Csv_ADDR) as f64
    }

    #[inline]
    pub fn qk(&self) -> f64 {
        self.read::<i16>(Qk_ADDR) as f64
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
