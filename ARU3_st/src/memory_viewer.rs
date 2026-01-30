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

const SA_ADRR: usize = 0x006321DC;
const VM_ADRR: usize = 0x00632080;
const FV_ADRR: usize = 0x0063210C;
const VG_ADRR: usize = 0x00632090;
const M_ADRR: usize = 0x00632114;
const FM_ADRR: usize = 0x00632110;
const KG_ADRR: usize = 0x006322F4;
const R8_ADRR: usize = 0x006321C8;
const I6_ADRR: usize = 0x0063222C;

pub struct Meme {
    handle: HANDLE,
}

pub const SA_COUNT: usize = 8;
impl Meme {
    pub fn new() -> Self {
        const PROC_NAME: &str = "LabARU3_st.exe";

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

        mem
    }

    fn tests(&self) -> bool {
        let mut f = false;
        if !(100_000.0..=10_000_000.).contains(&self.fv()) {
            println!("Подозрительная частота Генератора");
            f = true;
        }
        if !(10.0..=20_000.).contains(&self.fm()) {
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
        let r8 = self.r8();
        if r8.ceil() != r8 || (0.0..=6800.).contains(&self.r8()) {
            println!("Подозрительный R8");
            f = true;
        }
        for (n, i) in self.sa().into_iter().enumerate() {
            if !(1..=2).contains(&i) {
                println!("Подозрительные переключатель SA{}", n + 1);
                f = true;
            }
        }
        f
    }

    pub fn sa(&self) -> [i16; SA_COUNT] {
        self.read(SA_ADRR)
    }

    pub fn vm(&self) -> f32 {
        #[cfg(not(debug_assertions))]
        thread::sleep(time::Duration::from_millis(5));

        #[cfg(debug_assertions)]
        thread::sleep(time::Duration::from_millis(1));

        self.read(VM_ADRR)
    }

    pub fn vm_round(&self) -> f32{
        let volt = self.vm();
        const ZERS: f32 = 1000_0.;
        (volt*ZERS).round()/ZERS
    }

    pub fn fv(&self) -> f32 {
        self.read(FV_ADRR)
    }

    pub fn vg(&self) -> f32 {
        self.read(VG_ADRR)
    }

    pub fn m(&self) -> f32 {
        self.read(M_ADRR)
    }

    pub fn fm(&self) -> f32 {
        self.read(FM_ADRR)
    }

    pub fn i8(&self) -> f32 {
        self.read(I6_ADRR)
    }

    pub fn r8(&self) -> f32 {
        self.read(R8_ADRR)
    }

    pub fn kg(&self) -> f32 {
        thread::sleep(time::Duration::from_millis(15));

        self.read(KG_ADRR)
    }

    pub fn uk(&self) -> f32 {
        const Ek: usize = 0x0063289C;
        const Ib_VT1: usize = 0x0063223C;
        const Ube_VT1: usize = 0x006322D0;

        let ek:f32 = self.read(Ek);
        let ib:f32 = self.read(Ib_VT1);
        let ube:f32 = self.read(Ube_VT1);

        let ie = self.i8();
        
        let uk = if ek - (ie+ib)*2400. > ie * 510. + ube{
            ek - (ie+ib)*2400.
        } else {
            ie * 510.
        };
        
        -uk
    }

    #[inline]
    fn read<T: Default>(&self, addr:usize) -> T {
        let mut a:T = Default::default();
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

pub fn press_enter_for_exit() {
    // println!("Press Enter for exit...");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
}

#[cfg(debug_assertions)]
pub fn modultest(arr: [f32; 10]) {
    const PROC_NAME: &str = "LabConvertor2.exe";
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
