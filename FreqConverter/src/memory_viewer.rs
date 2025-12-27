use std::io::{self, Write};
use std::mem::{size_of, zeroed};
use std::process::exit;
use std::ptr::null_mut;
use std::{thread, time};
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
    MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{
    OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};

const SA_ADRR: usize = 0x00607078;
const VOLT_ADRR: usize = 0x006070C4;
const FREQ1_ADRR: usize = 0x006070F0;
const FREQ2_ADRR: usize = 0x006070F8;
const VOLVT_G1_ADRR: usize = 0x00607160;
const VOLVT_G2_ADRR: usize = 0x00607164;
const FARAD_C3_ADRR: usize = 0x00607208;
const FARAD_C9_ADRR: usize = 0x006071D0;
const I_E_ADRR: usize = 0x0060726C;
const TABL8_VOLT_ADRR: usize = 0x006074D8;
const FGET_ADRR: usize = 0x006070F8;

pub struct Meme {
    handle: HANDLE,
}

impl Meme {
    pub fn new() -> Self {
        let proc_name = "LabConvertor2.exe";

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

        let _base = match get_module_base(pid, proc_name) {
            Some(addr) => addr,
            None => {
                eprintln!(
                    "Модуль '{}' не найден в процессе {} (мэйби надо запустить с админкой)",
                    proc_name, pid
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
            println!("Мэйби Курочкин обновил прогу? Тогда пишите в поддержку");
            press_enter_for_exit();
            exit(5);
        }

        mem
    }

    fn tests(&self) -> bool {
        let mut f = false;
        if !(100000.0..=3000000.).contains(&self.fv1()) {
            println!("Подозрительная частота ГСВ1");
            f = true;
        }
        if !(0.0001..=10.).contains(&self.vg1()) {
            println!("Подозрительная Амплитуда ГСВ1");
            f = true;
        }
        // if !(100000.0..=3000000.).contains(&self.fv2()) {
        //     println!("Подозрительная частота ГСВ2");
        //     f=true;
        // }
        if !(0.0001..=10.).contains(&self.vg2()) {
            println!("Подозрительная Амплитуда ГСВ2");
            f = true;
        }
        for (n, i) in self.sa().into_iter().enumerate() {
            if !(1..=3).contains(&i) {
                println!("Подозрительные переключатель {}", n + 1);
                f = true;
            }
        }
        if !(0.0..=180.).contains(&self.ang_3()) {
            println!("Подозрительная ёмкасть С3,С18");
            f = true;
        }
        if !(0.0..=180.).contains(&self.ang_9()) {
            println!("Подозрительная ёмкасть С9");
            f = true;
        }
        f
    }

    pub fn sa(&self) -> [i16; 4] {
        let mut arr = [0i16; 4];
        read(self.handle, SA_ADRR, &mut arr);

        arr
    }

    pub fn vm(&self) -> f32 {
        #[cfg(not(debug_assertions))]
        thread::sleep(time::Duration::from_millis(5));

        #[cfg(debug_assertions)]
        thread::sleep(time::Duration::from_millis(1));

        let mut volt = 0f32;
        read(self.handle, VOLT_ADRR, &mut volt);

        volt
    }

    pub fn fv1(&self) -> f32 {
        let mut freq = 0f32;
        read(self.handle, FREQ1_ADRR, &mut freq);

        freq
    }

    pub fn fv2(&self) -> f32 {
        let mut freq = 0f32;
        read(self.handle, FREQ2_ADRR, &mut freq);

        freq
    }

    pub fn vg1(&self) -> f32 {
        let mut volt = 0f32;
        read(self.handle, VOLVT_G1_ADRR, &mut volt);

        volt
    }

    pub fn vg2(&self) -> f32 {
        let mut volt = 0f32;
        read(self.handle, VOLVT_G2_ADRR, &mut volt);

        volt
    }

    pub fn farad_9(&self) -> f32 {
        let mut farad = 0f32;
        read(self.handle, FARAD_C9_ADRR, &mut farad);

        farad
    }

    pub fn ang_9(&self) -> f32 {
        let farad = self.farad_9() * 1e10;

        let a = (farad - 0.1) / 0.00833333;
        (a * 10.).round() / 10.
    }

    pub fn farad_3(&self) -> f32 {
        let mut farad = 0f32;
        read(self.handle, FARAD_C3_ADRR, &mut farad);

        farad
    }

    pub fn ang_3(&self) -> f32 {
        let sa2 = self.sa()[1];
        let k = if sa2 == 2 { self.ang_9() * (0.9+0.6)/180.-0.9 } else { 0. };

        let farad = self.farad_3() * 1e10;

        let a = (farad - (1.12+k)) / 0.010444444;
        // let a = if sa2 == 2 {a-0.1} else { a };
        (a * 10.).round() / 10.
    }

    pub fn i_e(&self) -> f64 {
        let mut i = 0.0;
        read(self.handle, I_E_ADRR, &mut i);

        i
    }

    pub fn answer_tabl8(&self) -> [f32; 10] {
        let mut arr = [0f32; 10];
        read(self.handle, TABL8_VOLT_ADRR, &mut arr);

        arr
    }

    pub fn fgetor(&self) -> f32 {
        let mut fget = 0f32;
        read(self.handle, FGET_ADRR, &mut fget);

        fget
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

pub fn press_enter_for_exit() {
    // println!("Press Enter for exit...");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
}

#[cfg(debug_assertions)]
pub fn modultest(arr: [f32; 10]) {
    let proc_name = "LabConvertor2.exe";
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

    let _base = match get_module_base(pid, proc_name) {
        Some(addr) => addr,
        None => {
            eprintln!(
                "Модуль '{}' не найден в процессе {} (мэйби надо запустить с админкой)",
                proc_name, pid
            );
            press_enter_for_exit();
            exit(2);
        }
    };

    #[cfg(debug_assertions)]
    println!("Базовый адрес модуля = 0x{_base:X}");

    let h = unsafe {
        let h_process = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION,
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

    for (n, i) in [
        0xc4, 0xdc, 0xec, 0x180, 0x228, 0x258, 0x25c, 0x264, 0x26c, 0x270,
    ]
    .iter()
    .enumerate()
    {
        let mut f = 0f32;
        read(h, 0x00607000 + i, &mut f);
        let d = arr[n];
        let diff = f - d;
        println!("{f:<20}|{d:<20}|{diff}")
    }
}
