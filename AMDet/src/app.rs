use crate::memory_viewer::{press_enter_for_exit, Meme, SA_COUNT};
use crate::open_windows::{get_pos_init, get_pos_maket, get_pos_open_table};
use enigo::Direction::{Press, Release};
use enigo::{Button, Coordinate, Enigo, Keyboard, Mouse, Settings};
use enigo::Button::Left;
use enigo::Coordinate::{Abs, Rel};

#[derive(PartialEq)]
pub enum KIA {
    DIGIT,
    INI,
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
            cur_kia: KIA::DIGIT,
        }
    }

    fn click(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_maket().pos();
        self.enigo
            .move_mouse(xo + x, yo + y, Coordinate::Abs)
            .unwrap();
        self.enigo.button(Button::Left, Press).unwrap();
        self.enigo.button(Button::Left, Release).unwrap();
    }

    fn click_table(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_open_table().pos();
        self.enigo
            .move_mouse(xo + x, yo + y, Coordinate::Abs)
            .unwrap();
        self.enigo.button(Button::Left, Press).unwrap();
        self.enigo.button(Button::Left, Release).unwrap();
    }

    fn move_to(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_maket().pos();
        self.enigo
            .move_mouse(xo + x, yo + y, Coordinate::Abs)
            .unwrap();
    }

    // down -> 1 | up -> -1
    fn scroll(&mut self, x: i32, y: i32, to: i32) {
        self.move_to(x, y);
        self.enigo.scroll(to, enigo::Axis::Vertical).unwrap();
    }

    pub fn sleep(&self, mils: u64) {
        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(mils));
    }

    pub fn setup_maket(&mut self) {
        //kia to digit
        self.set_kia_to(KIA::DIGIT);
        self.set_kia_to(KIA::INI);
        self.set_kia_to(KIA::DIGIT);

        self.set_sas([1, 0, 0, 0, 0, 0, 2, 0, 0]);
        self.sa7();
        while get_pos_init().pos() == (0, 0) {}
        while get_pos_init().pos() != (0, 0) {}

        while !self.r8_to_04v() {
            self.sa7();
            self.sleep(100);
            self.set_sas([1, 0, 0, 0, 0, 0, 0, 0, 0]);
            self.sa7();
            while get_pos_init().pos() == (0, 0) {}
            while get_pos_init().pos() != (0, 0) {}
            self.set_sas([3, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }

    pub fn set_kia_to(&mut self, kia: KIA) {
        if self.cur_kia == kia {
            return;
        }

        self.click(253, 37);
        self.sleep(100);

        match kia {
            KIA::INI => {
                self.click(257, 130);
                self.cur_kia = KIA::INI
            }
            KIA::DIGIT => {
                self.click(270, 84);
                self.sleep(100);
                self.click(742, 659);
                self.cur_kia = KIA::DIGIT
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
        self.click(251, 637);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_more_more(&mut self) {
        let v = self.mem.vg();
        for i in [10., 1., 0.1, 0.01, 0.001, 0.0001] {
            if 0.9 * i <= v && v < 1. * i {
                return self.vg_more();
            }
        }
        self.click(241, 637);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_less(&mut self) {
        let v = self.mem.vg();
        self.click(167, 635);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_less_less(&mut self) {
        let v = self.mem.vg();
        for i in [1., 0.1, 0.01, 0.001, 0.0001] {
            if 1. * i <= v && v < 1.6 * i {
                return self.vg_less();
            }
        }
        self.click(170, 638);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }

    fn set_vg(&mut self, indx: i32) {
        let v = self.mem.vg();
        self.click(282, 537 + indx * 24);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }

    pub fn set_vg_to(&mut self, volt: f32) {
        let v = self.mem.vg();
        let volt = match volt {
            0.00001..0.0001 => {
                if !(0.00001..0.0001).contains(&v) {
                    self.set_vg(0)
                }
                (volt * 100_000_000.).floor() / 100_000_000.
            }
            0.0001..0.001 => {
                if !(0.0001..0.001).contains(&v) {
                    self.set_vg(1)
                }
                (volt * 10_000_000.).floor() / 10_000_000.
            }
            0.001..0.01 => {
                if !(0.001..0.01).contains(&v) {
                    self.set_vg(2)
                }
                (volt * 1_000_000.).floor() / 1_000_000.
            }
            0.01..0.1 => {
                if !(0.01..0.1).contains(&v) {
                    self.set_vg(3)
                }
                (volt * 100_000.).floor() / 100_000.
            }
            0.1..1. => {
                if !(0.1..1.).contains(&v) {
                    self.set_vg(4)
                }
                (volt * 10_000.).floor() / 10_000.
            }
            1.0..10.0 => {
                if !(1.0..10.0).contains(&v) {
                    self.set_vg(5)
                }
                (volt * 1_000.).floor() / 1_000.
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
        self.click(250, 531);
        self.waiter_1sec_while(|| f != self.mem.fv())
    }
    pub fn fv1_more_more(&mut self) {
        let f = self.mem.fv();
        if (987000. <= f && f <= 1000000.) || (295000. <= f && f < 300000.) {
            return self.fv1_more();
        }
        self.click(238, 531);
        self.waiter_1sec_while(|| f != self.mem.fv())
    }
    pub fn fv1_less(&mut self) {
        let f = self.mem.fv();
        if 100. == f {
            return;
        }
        self.click(158, 530);
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
        self.click(70 + 30 * indx, 695);

        if indx - self.freq1_bend == 1 && [300., 1000.].contains(&f) {
            self.sleep(200);
            return;
        }

        self.waiter_1sec_while(|| f != self.mem.fv());
    }

    pub fn set_fv1_to(&mut self, freq: f32) {
        let f = self.mem.fv();
        match freq {
            100_000.0..300_000.0 => {
                if !(100000.0..300000.0).contains(&f) {
                    self.set_fv1(0);
                }
            }
            300_000.0..1_000_000.0 => {
                if !(300000.0..1000000.0).contains(&f) {
                    self.set_fv1(1);
                }
            }
            1_000_000.0..3_000_000.0 => {
                if !(1000000.0..3000000.0).contains(&f) {
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
        self.enigo.move_mouse(175+x, 533+y, Abs).unwrap();
        self.enigo.button(Left, Press).unwrap();
        self.enigo.move_mouse(65, 0, Rel).unwrap();
        self.enigo.button(Left, Release).unwrap();
        self.waiter_1sec_while(|| f != self.mem.fv());
    }
}

// SA
impl App {
    #[inline]
    pub fn ssa9(&mut self) {
        self.sa9();
        self.sleep(500);
        self.sa9();
    }

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
            4 => self.sa4(),
            5 => self.sa5(),
            6 => self.sa6(),
            7 => self.sa7(),
            8 => self.sa8(),
            9 => self.sa9(),
            _ => (),
        }
    }

    pub fn sa1(&mut self) {
        let sa = self.mem.sa()[0];
        self.click(405, 553);
        self.waiter_1sec_while(|| sa != self.mem.sa()[0]);
    }

    pub fn sa2(&mut self) {
        let sa = self.mem.sa()[1];
        self.click(492, 538);
        self.waiter_1sec_while(|| sa != self.mem.sa()[1]);
    }

    pub fn sa3(&mut self) {
        let sa = self.mem.sa()[2];
        self.click(542, 531);
        self.waiter_1sec_while(|| sa != self.mem.sa()[2]);
    }

    pub fn sa4(&mut self) {
        let sa = self.mem.sa()[3];
        self.click(591, 535);
        self.waiter_1sec_while(|| sa != self.mem.sa()[3]);
    }
    pub fn sa5(&mut self) {
        let sa = self.mem.sa()[5 - 1];
        self.click(639, 532);
        self.waiter_1sec_while(|| sa != self.mem.sa()[5 - 1]);
    }
    pub fn sa6(&mut self) {
        let sa = self.mem.sa()[6 - 1];
        self.click(406, 694);
        self.waiter_1sec_while(|| sa != self.mem.sa()[6 - 1]);
    }
    pub fn sa7(&mut self) {
        let sa = self.mem.sa()[7 - 1];
        self.click(540, 688);
        self.waiter_1sec_while(|| sa != self.mem.sa()[7 - 1]);
    }
    pub fn sa8(&mut self) {
        let sa = self.mem.sa()[8 - 1];
        self.click(588, 682);
        self.waiter_1sec_while(|| sa != self.mem.sa()[8 - 1]);
    }
    pub fn sa9(&mut self) {
        let sa = self.mem.sa()[9 - 1];
        self.click(635, 682);
        self.waiter_1sec_while(|| sa != self.mem.sa()[9 - 1]);
    }
}

// Modulation
impl App {
    fn m_more_more(&mut self) {
        let m = self.mem.m();
        self.click(240, 601);
        self.waiter_1sec_while(|| m != self.mem.m());
    }

    fn m_less_less(&mut self) {
        let m = self.mem.m();
        self.click(170, 601);
        self.waiter_1sec_while(|| m != self.mem.m());
    }
    fn m_less(&mut self) {
        let m = self.mem.m();
        self.click(160, 601);
        self.waiter_1sec_while(|| m != self.mem.m());
    }

    fn f_more_more(&mut self) {
        let f = self.mem.fm();
        self.click(240, 566);
        self.waiter_1sec_while(|| f != self.mem.fm());
    }
    fn f_more(&mut self) {
        let f = self.mem.fm();
        self.click(250, 566);
        self.waiter_1sec_while(|| f != self.mem.fm());
    }

    fn f_less(&mut self) {
        let f = self.mem.fm();
        self.click(158, 566);
        self.waiter_1sec_while(|| f != self.mem.fm());
    }

    pub fn set_m_to(&mut self, procent: u32) {
        let proc = procent as f32 / 100.0;
        let mut m = self.mem.m();
        while m != proc {
            if m < proc {
                self.m_more_more();
            } else if m-proc < 0.1 {
                self.m_less();
            } else{
                self.m_less_less();
            }
            m = self.mem.m();
        }
    }

    pub fn set_f_to(&mut self, freq: u32) {
        let freq = freq as f32;
        let mut f = self.mem.fm();
        while freq != f {
            if f < freq {
                if freq-f>=1000.{
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
}

// PV
impl App {
    pub fn r8_to_04v(&mut self) -> bool {
        if self.mem.sa()[0] != 3 {
            return false;
        }
        let mut pv = self.mem.pv();

        while pv > 0.4 {
            self.scroll(493, 720, 1);
            pv = self.mem.pv();
        }

        if pv == 0.4 {
            return true;
        }

        let mut arr = Vec::new();
        while pv != 0.4 {
            if pv < 0.4 {
                self.scroll(493, 720, -1);
            } else {
                self.scroll(493, 720, 1);
            }
            self.waiter_1sec_while(|| pv != self.mem.pv());

            pv = self.mem.pv();
            arr.push(pv);
            if arr.len() > 6 {
                arr.remove(0);

                let a1 = arr[0];
                let a2 = arr[1];
                if [arr[2], arr[4]].contains(&a1) && [arr[3], arr[5]].contains(&a2) {
                    return false;
                }
            }
        }
        true
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
// const OPEN_TABLE_SLEEP: u64 = 400;
const WRITE_TABL_CLICK_T: u64 = 100;
// const WRITE_TABL_CLICK_T: u64 = 300;
// const TABLE_FINAL_CLICK_T: u64 = 1000;
impl App {
    pub fn open_table(&mut self, num: i32) {
        if num == 9 {
            self.click(895, 306);
        } else {
            let col = 1 - num % 2;
            let row = (num - 1) / 2;
            self.click(853 + col * 84, 141 + row * 42);
        }

        while get_pos_open_table().pos() == (0, 0) {}
        #[cfg(not(debug_assertions))]
        self.sleep(1000);
        #[cfg(debug_assertions)]
        self.sleep(100);
    }

    pub fn write_table1(&mut self, col: i32, row: i32) {
        self.open_table(1);

        let answ = self.mem.table1();

        self.write_tabl1_call(col, row, answ[(col * 4 + row) as usize]);
        self.close_tabl();
    }
    pub fn write_table3(&mut self, col: i32, row: i32) {
        self.open_table(3);

        let v = self.mem.vm() * 1000.;
        self.write_tabl3_call(col, row, v);

        self.close_tabl();
    }
    pub fn write_table5(&mut self, col: i32, row: i32, val: f32) {
        self.open_table(5);

        self.write_tabl5_call(col, row, val);

        self.close_tabl();
    }
    pub fn write_table6(&mut self, col: i32, row: i32, val: f32) {
        self.open_table(6);

        self.write_tabl6_call(col, row, val);
        self.close_tabl();
    }
    pub fn write_table7(&mut self, row: i32, val1: f32, val2: f32) {
        self.open_table(7);

        self.write_tabl7_call(0, row, val1);
        self.write_tabl7_call(1, row, val2);

        self.close_tabl();
    }
    pub fn write_table8(&mut self, col: i32, row: i32, val: f32) {
        self.open_table(8);

        self.write_tabl8_call(col, row, val);

        self.close_tabl();
    }
    pub fn write_table9(&mut self, col: i32, row: i32, val: f32) {
        self.open_table(9);

        self.write_tabl9_call(col, row, val);

        self.close_tabl();
    }

    fn write_tabl1_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(169 + col * 82, 152 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    pub fn write_tabl2_call(&mut self, col: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(159 + col * 127, 122);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl3_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(202 + col * 82, 166 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    pub fn write_tabl4_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(155 + col * 80, 126 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl5_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.2}");

        self.click_table(165 + 82 * col, 161 + 32 * row);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl6_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.2}");

        self.click_table(158 + col * 80, 159 + 32 * row);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl7_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        let x = 242 + col * 80;
        self.click_table(x, 157 + 34 * row);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl8_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        let y = 130 + row * 33;
        self.click_table(252 + col * 80, y);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl9_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(138 + col * 80, 126 + row * 32);
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
            self.enigo
                .move_mouse(x + 322, y + 126, Coordinate::Abs)
                .unwrap();
            self.enigo.button(Button::Left, Press).unwrap();
            self.enigo.button(Button::Left, Release).unwrap();
            while get_pos_was_saved().pos() != (0, 0) {}
        }
    }
}
