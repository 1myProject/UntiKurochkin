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

const BASE: usize = 0x006DF000+0x2008;
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

pub struct Meme {
    handle: HANDLE,
}

pub const SA_COUNT: usize = 7;
impl Meme {
    pub fn new() -> Self {
        const PROC_NAME: &str = "LabIM3_st.exe";

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

        #[cfg(not(test))]
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

        let (qk, lk, csv, ck) = self.get_from_form();
        if !(5.0..=10.0).contains(&qk) {
            println!("Подозрительная Qk");
            f = true;
        }
        if !(10.0e-6..=24.0e-6).contains(&lk) {
            println!("Подозрительная Lk");
            f = true;
        }
        if !(100.0e-12..200.1e-12).contains(&csv) {
            println!("Подозрительная Csv");
            f = true;
        }
        if !(250.0e-11..=500.0e-11).contains(&ck) {
            println!("Подозрительная Ck");
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

    pub fn get_from_form(&self) -> (f64, f64, f64, f64) {
        let f= self.form();
        (f.Qk as f64, f.Lk as f64, f.Csv as f64, f.Ck as f64)
    }

    #[inline]
    fn form(&self) -> Note {self.read(Forma_ADDR)}

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

        if ok == 0 || bytes != bytes_read {
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

#[test]
fn test_struct(){
    let mem = Meme::new();
    let form = mem.form();
    println!("{:#?}", form);
    assert!(!mem.tests());
}

type STR = u16;
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug,Default)]
struct Note{

    Admin: u16,

    Tust_Video_SA1a_E: [f32;3],
    Tspad_Video_SA1a_E: [f32;3],
    Tust_Radio_SA1a_E: [f32;3],
    Tspad_Radio_SA1a_E: [f32;3],

    Fmin_0_1_SA2a_1_E: f32,
    Fmin_0_7_SA2a_1_E: f32,
    Fo_1_0_SA2a_1_E: f32,
    Fmax_0_1_SA2a_1_E: f32,
    Fmax_0_7_SA2a_1_E: f32,
    df_0_1_SA2a_1_E: f32,
    df_0_7_SA2a_1_E: f32,
    Kpr_SA2a_1_E: f32,
    Ko_SA2a_1_E: f32,
    hz1: f32,
    Family: [STR;20],
    Fmin_0_1_SA2a_2_E: f32,
    Fmin_0_7_SA2a_2_E: f32,
    Fo_1_0_SA2a_2_E: f32,
    Fmax_0_1_SA2a_2_E: f32,
    Fmax_0_7_SA2a_2_E: f32,
    df_0_1_SA2a_2_E: f32,
    df_0_7_SA2a_2_E: f32,
    Kpr_SA2a_2_E: f32,
    Ko_SA2a_2_E: f32,
    hz2:f32,
    Name: [STR;20],
    Fmin_0_1_SA2a_3_E: f32,
    Fmin_0_7_SA2a_3_E: f32,
    Fo_1_0_SA2a_3_E: f32,
    Fmax_0_1_SA2a_3_E: f32,
    Fmax_0_7_SA2a_3_E: f32,
    df_0_1_SA2a_3_E: f32,
    df_0_7_SA2a_3_E: f32,
    Kpr_SA2a_3_E: f32,
    Ko_SA2a_3_E: f32,

    Fmin_0_1_SA2a_4_E: f32,
    Fmin_0_7_SA2a_4_E: f32,
    Fo_1_0_SA2a_4_E: f32,
    Fmax_0_1_SA2a_4_E: f32,
    Fmax_0_7_SA2a_4_E: f32,
    df_0_1_SA2a_4_E: f32,
    df_0_7_SA2a_4_E: f32,
    Kpr_SA2a_4_E: f32,
    Ko_SA2a_4_E: f32,
    hz3:f32,
    hz4:f32,
    LName: [STR;20],
    Fmin_0_1_SA1_1_E: f32,
    Fmin_0_7_SA1_1_E: f32,
    Fo_1_0_SA1_1_E: f32,
    Fmax_0_1_SA1_1_E: f32,
    Fmax_0_7_SA1_1_E: f32,
    df_0_1_SA1_1_E: f32,
    df_0_7_SA1_1_E: f32,
    Kpr_SA1_1_E: f32,
    Ko_SA1_1_E: f32,

    Fmin_0_1_SA1_2_E: f32,
    Fmin_0_7_SA1_2_E: f32,
    Fo_1_0_SA1_2_E: f32,
    Fmax_0_1_SA1_2_E: f32,
    Fmax_0_7_SA1_2_E: f32,
    df_0_1_SA1_2_E: f32,
    df_0_7_SA1_2_E: f32,
    Kpr_SA1_2_E: f32,
    Ko_SA1_2_E: f32,
    Group: [STR;10],
    Fmin_0_1_SA1_3_E: f32,
    Fmin_0_7_SA1_3_E: f32,
    Fo_1_0_SA1_3_E: f32,
    Fmax_0_1_SA1_3_E: f32,
    Fmax_0_7_SA1_3_E: f32,
    df_0_1_SA1_3_E: f32,
    df_0_7_SA1_3_E: f32,
    Kpr_SA1_3_E: f32,
    Ko_SA1_3_E: f32,

    Fmin_0_1_SA1_4_E: f32,
    Fmin_0_7_SA1_4_E: f32,
    Fo_1_0_SA1_4_E: f32,
    Fmax_0_1_SA1_4_E: f32,
    Fmax_0_7_SA1_4_E: f32,
    df_0_1_SA1_4_E: f32,
    df_0_7_SA1_4_E: f32,
    Kpr_SA1_4_E: f32,
    Ko_SA1_4_E: f32,

    Tust_Video_SA3_E: [f32;3],
    Tspad_Video_SA3_E: [f32;3],

    U_IM_0_R_0_SA3_E: [f32;3],
    U_IM_0_R_5_SA3_E: [f32;3],
    R_IM_0_SA3_E: [f32;3],
    U_IM_1_R_0_SA3_E: [f32;3],
    U_IM_1_R_5_SA3_E: [f32;3],
    R_IM_1_SA3_E: [f32;3],

    Tust_SA1_E: [f32;4],
    Tspad_SA1_E: [f32;4],
    code: i16,
    Lk: f32,
    Ck: f32,
    Cn: f32,
    Cn1: f32,
    Cn2: f32,
    Cn3: f32,
    Csv: f32,
    Csvd: f32,
    Rn: f32,
    Rd: f32,
    Qk: i16,
    y21: f32,
    V1_1_1_in: f32,
    V1_1_1_out: f32,
    F1_1_1_gen: f32,

    V1_1_2_in: f32,
    V1_1_2_out: f32,
    F1_1_2_gen: f32,

    V1_1_3_in: f32,
    V1_1_3_out: f32,
    F1_1_3_gen: f32,

    V1_1_4_in: f32,
    V1_1_4_out: f32,
    F1_1_4_gen: f32,

    V1_1_5_in: f32,
    V1_1_5_out: f32,
    F1_1_5_gen: f32,

    V1_2_1_in: f32,
    V1_2_1_out: f32,
    F1_2_1_gen: f32,

    V1_2_2_in: f32,
    V1_2_2_out: f32,
    F1_2_2_gen: f32,

    V1_2_3_in: f32,
    V1_2_3_out: f32,
    F1_2_3_gen: f32,

    V1_2_4_in: f32,
    V1_2_4_out: f32,
    F1_2_4_gen: f32,

    V1_2_5_in: f32,
    V1_2_5_out: f32,
    F1_2_5_gen: f32,

    V1_3_1_in: f32,
    V1_3_1_out: f32,
    F1_3_1_gen: f32,

    V1_3_2_in: f32,
    V1_3_2_out: f32,
    F1_3_2_gen: f32,

    V1_3_3_in: f32,
    V1_3_3_out: f32,
    F1_3_3_gen: f32,

    V1_3_4_in: f32,
    V1_3_4_out: f32,
    F1_3_4_gen: f32,

    V1_3_5_in: f32,
    V1_3_5_out: f32,
    F1_3_5_gen: f32,

    V1_4_1_in: f32,
    V1_4_1_out: f32,
    F1_4_1_gen: f32,

    V1_4_2_in: f32,
    V1_4_2_out: f32,
    F1_4_2_gen: f32,

    V1_4_3_in: f32,
    V1_4_3_out: f32,
    F1_4_3_gen: f32,

    V1_4_4_in: f32,
    V1_4_4_out: f32,
    F1_4_4_gen: f32,

    V1_4_5_in: f32,
    V1_4_5_out: f32,
    F1_4_5_gen: f32,

    V2_1_in: [f32;4],
    Q2_1: [f32;4],
    F2_1_gen: [f32;4],
    F2_1_imp: [f32;4],

    V2_2_in: [f32;4],
    Q2_2: [f32;4],
    F2_2_gen: [f32;4],
    F2_2_imp: [f32;4],
    O2_1_imp: [u16;4],
    Kd2_1_imp: [i16;4],
    O2_2_imp: [u16;4],
    Kd2_2_imp: [i16;4],

    V3_1_in: [f32;3],
    Q3_1: [f32;3],
    F3_1_gen: [f32;3],
    F3_1_imp: [f32;3],
    O3_1_imp: [u16;3],
    Kd3_1_imp: [i16;3],

    V3_2_in: [f32;3],
    Q3_2: [f32;3],
    F3_2_gen: [f32;3],
    F3_2_imp: [f32;3],
    O3_2_imp: [u16;3],
    Kd3_2_imp: [i16;3],

    V3_3_in: [f32;3],
    Q3_3: [f32;3],
    F3_3_gen: [f32;3],
    F3_3_imp: [f32;3],
    O3_3_imp: [u16;3],
    Kd3_3_imp: [i16;3],

    V3_4_in: [f32;3],
    Q3_4: [f32;3],
    F3_4_gen: [f32;3],
    F3_4_imp: [f32;3],
    O3_4_imp: [u16;3],
    Kd3_4_imp: [i16;3],

    V4_1_in: [f32;3],
    Q4_1: [f32;3],
    F4_1_gen: [f32;3],
    F4_1_imp: [f32;3],
    O4_1_imp: [u16;3],
    Kd4_1_imp: [i16;4],

    V4_2_in: [f32;3],
    Q4_2: [f32;3],
    F4_2_gen: [f32;3],
    F4_2_imp: [f32;3],
    O4_2_imp: [u16;3],
    Kd4_2_imp: [i16;4],

    V5_1_1_in: f32,
    V5_1_1_out: f32,
    F5_1_1_gen: f32,

    V5_1_2_in: f32,
    V5_1_2_out: f32,
    F5_1_2_gen: f32,

    V5_1_3_in: f32,
    V5_1_3_out: f32,
    F5_1_3_gen: f32,

    V5_1_4_in: f32,
    V5_1_4_out: f32,
    F5_1_4_gen: f32,

    V5_1_5_in: f32,
    V5_1_5_out: f32,
    F5_1_5_gen: f32,

    V5_2_1_in: f32,
    V5_2_1_out: f32,
    F5_2_1_gen: f32,

    V5_2_2_in: f32,
    V5_2_2_out: f32,
    F5_2_2_gen: f32,

    V5_2_3_in: f32,
    V5_2_3_out: f32,
    F5_2_3_gen: f32,

    V5_2_4_in: f32,
    V5_2_4_out: f32,
    F5_2_4_gen: f32,

    V5_2_5_in: f32,
    V5_2_5_out: f32,
    F5_2_5_gen: f32,

    V5_3_1_in: f32,
    V5_3_1_out: f32,
    F5_3_1_gen: f32,

    V5_3_2_in: f32,
    V5_3_2_out: f32,
    F5_3_2_gen: f32,

    V5_3_3_in: f32,
    V5_3_3_out: f32,
    F5_3_3_gen: f32,

    V5_3_4_in: f32,
    V5_3_4_out: f32,
    F5_3_4_gen: f32,

    V5_3_5_in: f32,
    V5_3_5_out: f32,
    F5_3_5_gen: f32,

    V5_4_1_in: f32,
    V5_4_1_out: f32,
    F5_4_1_gen: f32,

    V5_4_2_in: f32,
    V5_4_2_out: f32,
    F5_4_2_gen: f32,

    V5_4_3_in: f32,
    V5_4_3_out: f32,
    F5_4_3_gen: f32,

    V5_4_4_in: f32,
    V5_4_4_out: f32,
    F5_4_4_gen: f32,

    V5_4_5_in: f32,
    V5_4_5_out: f32,
    F5_4_5_gen: f32,

    V6_1_in: [f32;3],
    Q6_1: [f32;3],
    F6_1_gen: [f32;3],
    F6_1_imp: [f32;3],
    O6_1_imp: [u16;3],
    Kd6_1_imp: [i16;3],

    V6_2_in: [f32;3],
    Q6_2: [f32;3],
    F6_2_gen: [f32;3],
    F6_2_imp: [f32;3],
    O6_2_imp: [u16;3],
    Kd6_2_imp: [i16;3],

    V7_1_in: [f32;3],
    Q7_1: [f32;3],
    F7_1_gen: [f32;3],
    F7_1_imp: [f32;3],
    O7_1_imp: [u16;3],
    Kd7_1_imp: [i16;3],

    V7_2_in: [f32;3],
    Q7_2: [f32;3],
    F7_2_gen: [f32;3],
    F7_2_imp: [f32;3],
    O7_2_imp: [u16;3],
    Kd7_2_imp: [i16;3],
}
