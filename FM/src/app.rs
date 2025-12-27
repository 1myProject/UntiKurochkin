use crate::memory_viewer::{press_enter_for_exit, Meme, SA_COUNT};
use crate::open_windows::{get_pos_maket, get_pos_open_table};
use enigo::Button::Left;
use enigo::Coordinate::{Abs, Rel};
use enigo::Direction::{Press, Release};
use enigo::{Enigo, Key, Keyboard, Mouse, Settings};


#[derive(PartialEq, Clone, Copy)]
pub enum KIA {
    DIGIT_DC,
    DIGIT_AC,
    INI,
    AFC,
}

pub struct App {
    enigo: Enigo,
    pub mem: Meme,
    freq1_bend: i32,
    cur_kia: KIA,
}

impl App {
    pub fn new() -> App {
        let enigo = Enigo::new(&Settings::default()).unwrap();

        App {
            enigo,
            mem: Meme::new(),
            freq1_bend: 0,
            cur_kia: KIA::DIGIT_AC,
        }
    }

    pub fn form(&mut self) {
        self.enigo.move_mouse(1229, 722, Abs).unwrap();
        self.enigo.button(Left, Press).unwrap();
        self.enigo.button(Left, Release).unwrap();


        self.enigo.text("Traskovsky\nRoman\nVitalievich\n341301\n").unwrap();
        self.enigo.key(Key::Tab, Press);
        self.enigo.key(Key::Tab, Release);

        self.enigo.key(Key::Space, Press);
        self.enigo.key(Key::Space, Release);

        self.sleep(1000);
    }

    fn click(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_maket().pos();
        self.enigo.move_mouse(xo + x, yo + y, Abs).unwrap();
        self.enigo.button(Left, Press).unwrap();
        self.enigo.button(Left, Release).unwrap();
    }

    fn click_table(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_open_table().pos();
        self.enigo.move_mouse(xo + x, yo + y, Abs).unwrap();
        self.enigo.button(Left, Press).unwrap();
        self.enigo.button(Left, Release).unwrap();
    }

    fn move_to(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_maket().pos();
        self.enigo.move_mouse(xo + x, yo + y, Abs).unwrap();
    }

    pub fn sleep(&self, mils: u64) {
        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(mils));
    }

    pub fn setup_maket(&mut self) {
        //kia to digit
        self.set_kia_to(KIA::INI);
        self.set_kia_to(KIA::DIGIT_AC);
    }

    pub fn set_kia_to(&mut self, kia: KIA) {
        if self.cur_kia == kia {
            return;
        }

        self.click(253, 37);
        self.sleep(100);

        self.cur_kia = match kia {
            KIA::INI => {
                self.click(273, 172);
                KIA::INI
            }
            KIA::DIGIT_AC => {
                self.click(254, 58);
                self.sleep(100);
                self.click(500, 600);
                self.sleep(100);
                self.click(561, 638);
                KIA::DIGIT_AC
            }
            KIA::DIGIT_DC => {
                self.click(254, 58);
                self.sleep(100);
                self.click(500, 600);
                self.sleep(100);
                self.click(527, 638);
                KIA::DIGIT_DC
            }
            KIA::AFC => {
                self.click(287, 121);
                KIA::AFC
            }
        }
    }

    pub fn waiter_1sec_while(&self, fun: impl Fn() -> bool) {
        for _ in 0..1000 {
            if fun() {
                break;
            }
            self.sleep(1)
        }
    }

    pub fn waiter_1sec_while_vm(&self, v: f32) {
        self.waiter_1sec_while(|| v != self.mem.vm());
    }
}

//volt GS
impl App {
    fn vg_more(&mut self) {
        let v = self.mem.vg();
        self.click(279, 580);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_more_more(&mut self) {
        let v = self.mem.vg();
        for i in [1., 0.1, 0.01, 0.001, 0.0001, 0.00001] {
            if 0.98 * i <= v && v < 1. * i {
                return self.vg_more();
            }
        }
        self.click(271, 579);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_less(&mut self) {
        let v = self.mem.vg();
        self.click(191, 581);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_less_less(&mut self) {
        let v = self.mem.vg();
        for i in [0.1, 0.01, 0.001, 0.0001] {
            if 1. * i <= v && v < 1.4 * i {
                return self.vg_less();
            }
        }
        self.click(203, 583);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }

    fn set_vg(&mut self, indx: i32) {
        let v = self.mem.vg();
        self.click(314, 481 + indx * 24);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }

    pub fn set_vg_to(&mut self, volt: f32) {
        let v = self.mem.vg();
        let volt = match volt {
            0.000_001..0.000_01 => {
                if !(0.000_001..0.000_01).contains(&v) {
                    self.set_vg(0)
                }
                (volt * 1_000_000_000.).floor() / 1_000_000_000.
            }
            0.000_01..0.000_1 => {
                if !(0.00001..0.0001).contains(&v) {
                    self.set_vg(1)
                }
                (volt * 100_000_000.).floor() / 100_000_000.
            }
            0.000_1..0.001 => {
                if !(0.0001..0.001).contains(&v) {
                    self.set_vg(2)
                }
                (volt * 10_000_000.).floor() / 10_000_000.
            }
            0.001..0.01 => {
                if !(0.001..0.01).contains(&v) {
                    self.set_vg(3)
                }
                (volt * 1_000_000.).floor() / 1_000_000.
            }
            0.01..0.1 => {
                if !(0.01..0.1).contains(&v) {
                    self.set_vg(4)
                }
                (volt * 100_000.).floor() / 100_000.
            }
            0.1..=1. => {
                if !(0.1..=1.).contains(&v) {
                    self.set_vg(5)
                }
                (volt * 10_000.).floor() / 10_000.
            }
            _ => panic!("not supported volt2: {}", volt),
        };
        self.waiter_1sec_while(|| v != self.mem.vg());

        let mut v = self.mem.vg();
        let mut dif = (v - volt).abs() + 1.;
        while (v - volt).abs() < dif {
            dif = (v - volt).abs();
            if v < volt {
                self.vg_more_more()
            } else {
                self.vg_less_less()
            }
            v = self.mem.vg();
        }

        while (v - volt).abs() > 0.000_000_1 {
            if v < volt {
                self.vg_more();
            } else {
                self.vg_less();
            }
            v = self.mem.vg();
        }
    }

    pub fn find_volt_by_vg1(&mut self, v_find: f32) {
        let mut last: Option<bool> = None;
        let mut vm_r = self.mem.vm_round();
        let mut vec = Vec::new();
        while (vm_r - v_find).abs() > 0.00001 {
            let last_vg = self.mem.vg();
            let diff = (vm_r - v_find).abs();

            let vm = self.mem.vm();
            if vm >= 3.47 {
                let indx = 4 + last_vg.log10() as i32;
                self.set_vg(indx);
                vm_r = self.mem.vm();
                continue;
            }
            if vm_r < v_find {
                if diff > 0.002 {
                    self.vg_more_more()
                } else {
                    self.vg_more();
                }
                last = Some(false);
            } else {
                if diff > 0.002 {
                    self.vg_less_less()
                } else {
                    self.vg_less()
                }
                last = Some(true);
            }

            self.waiter_1sec_while(|| vm != self.mem.vm());
            vm_r = self.mem.vm_round();
            vec.push(vm_r);
            if vec.len() > 6 {
                vec.remove(0);
                let v1 = vec[0];
                let v2 = vec[1];
                if v1 != v2 && [vec[2], vec[4]].contains(&v1) && [vec[3], vec[5]].contains(&v2) {
                    break;
                }
            }

            let vg = self.mem.vg();
            if vg.log10().round() == vg.log10() {
                let indx = 5 + vg.log10() as i32;
                if vg - last_vg > 0. {
                    self.set_vg(indx);
                } else {
                    self.set_vg(indx - 1);
                }
                vm_r = self.mem.vm();
            }
        }

        let vm = self.mem.vm();
        match last {
            Some(true) => {
                self.vg_more();
            }
            Some(false) => self.vg_less(),
            _ => (),
        }
        self.waiter_1sec_while(|| vm != self.mem.vm());

        let mut arr = [0.; 5];
        arr[2] = (self.mem.vm() - v_find).abs();

        let vm = self.mem.vm();
        self.vg_less();
        self.waiter_1sec_while(|| vm != self.mem.vm());
        arr[1] = (self.mem.vm() - v_find).abs();

        let vm = self.mem.vm();
        self.vg_less();
        self.waiter_1sec_while(|| vm != self.mem.vm());
        arr[0] = (self.mem.vm() - v_find).abs();

        let vm = self.mem.vm();
        self.vg_more();
        self.vg_more();
        self.vg_more();
        self.waiter_1sec_while(|| vm != self.mem.vm());
        arr[3] = (self.mem.vm() - v_find).abs();

        let vm = self.mem.vm();
        self.vg_more();
        self.waiter_1sec_while(|| vm != self.mem.vm());
        arr[4] = (self.mem.vm() - v_find).abs();

        let min = *arr.iter().min_by(|&&x, &y| x.total_cmp(y)).unwrap();
        let indx = 4 - arr.iter().position(|&x| x == min).unwrap();
        let mv = self.mem.vm();
        for _ in 0..indx {
            self.vg_less()
        }
        self.waiter_1sec_while(|| mv != self.mem.vm());
    }
}

// freq
// const FREQ_SLEEP: u64 = 10;
impl App {
    pub fn fv1_more(&mut self) {
        let f = self.mem.fv();
        self.click(281, 472);
        self.waiter_1sec_while(|| f != self.mem.fv())
    }
    pub fn fv1_more_more(&mut self) {
        let f = self.mem.fv();
        if (987000. <= f && f <= 1000000.) || (295000. <= f && f < 300000.) {
            return self.fv1_more();
        }
        self.click(273, 472);
        self.waiter_1sec_while(|| f != self.mem.fv())
    }
    pub fn fv1_less(&mut self) {
        let f = self.mem.fv();
        if 100. == f {
            return;
        }
        self.click(191, 472);
        self.waiter_1sec_while(|| f != self.mem.fv())
    }
    // pub fn fv1_less_less(&mut self) {
    //     let f = self.mem.fv();
    //     if 100. == f {
    //         return;
    //     }
    //     if (100_000. <= f && f < 121000.) || (300_000. <= f && f < 343000.) {
    //         return self.fv1_less();
    //     }
    //     self.click(167, 531);
    //     while f == self.mem.fv() {}
    // }
    pub fn set_fv1(&mut self, indx: i32) {
        let f = self.mem.fv();
        self.click(128 + 29 * indx, 636);

        if indx - self.freq1_bend == 1 && [300., 1000.].contains(&f) {
            self.sleep(200);
            return;
        }

        self.waiter_1sec_while(|| f != self.mem.fv());
    }

    pub fn set_fv1_to(&mut self, freq: f32) {
        let f = self.mem.fv();
        match freq {
            300_000.0..1_000_000.0 => {
                if !(300000.0..1000000.0).contains(&f) {
                    self.set_fv1(0);
                }
            }
            1_000_000.0..3_000_000.0 => {
                if !(1000000.0..3000000.0).contains(&f) {
                    self.set_fv1(1);
                }
            }
            3_000_000.0..10_000_000.0 => {
                if !(3_000_000.0..10_000_000.0).contains(&f) {
                    self.set_fv1(2);
                }
            }
            _ => {
                eprint!("!!!!!not supported frequency!: {freq}\nexit: ");
                press_enter_for_exit();
                panic!("not supported frequency!");
            }
        };

        for _ in 0..10000 {
            let f = self.mem.fv();
            if f == freq {
                break;
            } else if freq > f {
                self.fv1_more_more();
            } else {
                self.fv1_less();
            }
        }
    }

    pub fn fv_to_max(&mut self) {
        let f = self.mem.fv();
        let (x, y) = get_pos_maket().pos();
        self.enigo.move_mouse(175 + x, 533 + y, Abs).unwrap();
        self.enigo.button(Left, Press).unwrap();
        self.enigo.move_mouse(65, 0, Rel).unwrap();
        self.enigo.button(Left, Release).unwrap();
        self.waiter_1sec_while(|| f != self.mem.fv());
    }
}

// SA
impl App {
    pub fn set_sas(&mut self, sp_need: [i16; SA_COUNT]) {
        let mut sp_orig = self.mem.sa();
        for i in 0..SA_COUNT {
            loop {
                if sp_need[i] == 0 || sp_need[i] == sp_orig[i] {
                    break;
                }
                self.set_sa_indx(i);
                sp_orig = self.mem.sa();
                // #[cfg(not(debug_assertions))]
                // self.sleep(1000);
                // #[cfg(debug_assertions)]
                // self.sleep(500);
            }
        }
    }

    fn set_sa_indx(&mut self, indx: usize) {
        match indx + 1 {
            1 => self.sa1(),
            2 => self.sa2(),
            3 => self.sa3(),
            _ => (),
        }
    }

    pub fn sa1(&mut self) {
        let sa = self.mem.sa()[0];
        self.click(781, 124);
        self.waiter_1sec_while(|| sa != self.mem.sa()[0]);
    }

    pub fn sa2(&mut self) {
        let sa = self.mem.sa()[1];
        self.click(779, 268);
        self.waiter_1sec_while(|| sa != self.mem.sa()[1]);
    }

    pub fn sa3(&mut self) {
        let sa = self.mem.sa()[2];
        self.click(757, 388);
        self.waiter_1sec_while(|| sa != self.mem.sa()[2]);
    }
}

// Modulation
impl App {
    fn f_more_more(&mut self) {
        let f = self.mem.fm();
        self.click(271, 510);
        self.waiter_1sec_while(|| f != self.mem.fm());
    }
    fn f_more(&mut self) {
        let f = self.mem.fm();
        self.click(280, 509);
        self.waiter_1sec_while(|| f != self.mem.fm());
    }

    fn f_less(&mut self) {
        let f = self.mem.fm();
        self.click(191, 510);
        self.waiter_1sec_while(|| f != self.mem.fm());
    }

    pub fn set_f_to(&mut self, freq: u32) {
        let freq = freq as f32;
        let mut f = self.mem.fm();
        while freq != f {
            if f < freq {
                if freq - f >= 1000. {
                    self.f_more_more();
                } else {
                    self.f_more();
                }
            } else {
                self.f_less();
            }
            f = self.mem.fm();
        }
    }

    fn fd_more_more(&mut self) {
        let f = self.mem.fd();
        // if f >= 490_000.{
        //     self.fd_more();
        //     return;
        // }
        self.click(272, 544);
        self.waiter_1sec_while(|| f != self.mem.fd());
    }
    fn fd_more(&mut self) {
        let f = self.mem.fd();
        self.click(275, 544);
        self.waiter_1sec_while(|| f != self.mem.fd());
    }
    fn fd_less_less(&mut self) {
        let f = self.mem.fd();
        if f <= 30_000. {
            self.fd_less();
            return;
        }
        self.click(201, 542);
        self.waiter_1sec_while(|| f != self.mem.fd());
    }

    fn fd_less(&mut self) {
        let f = self.mem.fd();
        self.click(191, 544);
        self.waiter_1sec_while(|| f != self.mem.fd());
    }

    pub fn set_fd_to(&mut self, freq: f32) {
        let mut fd = self.mem.fd();
        while fd != freq {
            if fd < freq {
                self.fd_more_more();
            } else if fd - freq < 10_000. {
                self.fd_less();
            } else {
                self.fd_less_less();
            }
            fd = self.mem.fd();
        }
    }
}

fn to_human_value(text: String) -> String {
    if !text.contains(".") {
        return text;
    }
    let text = text.trim_end_matches('0');
    if text.ends_with(".") {
        text.trim_end_matches(".").to_string()
    } else {
        text.to_string()
    }
}
//table
const WRITE_TABL_CLICK_T: u64 = 100;
impl App {
    pub fn open_table(&mut self, num: i32) {
        self.click(795, 482 + (num - 1) * 26);

        while get_pos_open_table().pos() == (0, 0) {}
        #[cfg(not(debug_assertions))]
        self.sleep(1000);
        #[cfg(debug_assertions)]
        self.sleep(100);
    }

    pub fn write_table1(&mut self, col: i32, row: i32) {
        self.open_table(1);

        let vm = self.mem.vm() * 1000.;

        self.write_tabl1_call(col, row, vm);
        self.close_tabl();
    }

    pub fn write_table2_4(&mut self, t: i32, col: i32) {
        let kia = self.cur_kia;

        let f = self.mem.fv() / 1000.;
        self.set_kia_to(KIA::AFC);
        self.open_table(t);

        self.write_tabl2_4_call(col, 0, f);
        self.close_tabl();
        self.set_kia_to(kia);

        let vm = self.mem.vm() * 1000.;
        self.set_kia_to(KIA::AFC);
        self.open_table(t);

        self.write_tabl2_4_call(col, 1, vm);
        self.close_tabl();
        self.set_kia_to(kia);
    }

    pub fn write_table3_5(&mut self, t: i32, col: i32) {
        let kg = self.mem.kg() * 100.;
        let vm = self.mem.vm() * 1000.;

        let kia = self.cur_kia;
        self.set_kia_to(KIA::AFC);

        self.open_table(t);

        self.write_tabl3_5_call(col, 0, vm);
        self.write_tabl3_5_call(col, 1, kg);

        self.close_tabl();

        self.set_kia_to(kia);
    }

    fn write_tabl1_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(199 + col * 80, 121 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    pub fn write_tabl2_4_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(138 + col * 80, 123 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl3_5_call(&mut self, col: i32, row: i32, val: f32) {
        let text = if val > 1. {
            format!("{val:.1}")
        } else {
            format!("{val:.2}")
        };

        self.click_table(131 + col * 80, 122 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    pub fn close_tabl(&mut self) {
        self.click_table(122, 41);
        while get_pos_open_table().pos() != (0, 0) {}
    }

    pub fn final_table(&mut self) {
        // #[cfg(not(debug_assertions))]
        {
            use crate::open_windows::get_pos_was_saved;
            self.click_table(47, 42);

            while get_pos_was_saved().pos() == (0, 0) {}
            let (x, y) = get_pos_was_saved().pos();
            self.enigo.move_mouse(x + 322, y + 126, Abs).unwrap();
            self.enigo.button(Left, Press).unwrap();
            self.enigo.button(Left, Release).unwrap();
            while get_pos_was_saved().pos() != (0, 0) {}
        }
    }
}
