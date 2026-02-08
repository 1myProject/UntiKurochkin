use crate::memory_viewer::{press_enter_for_exit, Meme, SA_COUNT};
use crate::open_windows::{get_pos_maket, get_pos_open_table};
use enigo::Button::Left;
use enigo::Coordinate::{Abs, Rel};
use enigo::Direction::{Press, Release};
use enigo::{Button, Coordinate, Enigo, Keyboard, Mouse, Settings};
use std::cmp::Ordering;

#[derive(PartialEq)]
pub enum KIA {
    DIGIT,
    // INI,
}

pub struct App {
    enigo: Enigo,
    pub mem: Meme,
    freq1_bend: i32,
    cur_kia: KIA,
}

const SAFE_WAITER: u64=50;
impl App {
    pub fn new(mem: Meme) -> App {
        let enigo = Enigo::new(&Settings::default()).unwrap();

        App {
            enigo,
            mem,
            freq1_bend: 0,
            cur_kia: KIA::DIGIT,
        }
    }

    pub fn click(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_maket().pos();
        self.enigo.move_mouse(xo + x, yo + y, Abs).unwrap();
        self.enigo.button(Button::Left, Press).unwrap();
        self.enigo.button(Button::Left, Release).unwrap();
        #[cfg(not(debug_assertions))]
        self.sleep(SAFE_WAITER);
    }

    fn click_table(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_open_table().pos();
        self.enigo
            .move_mouse(xo + x, yo + y, Coordinate::Abs)
            .unwrap();
        self.enigo.button(Button::Left, Press).unwrap();
        self.enigo.button(Button::Left, Release).unwrap();
        #[cfg(not(debug_assertions))]
        self.sleep(SAFE_WAITER);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_maket().pos();
        self.enigo
            .move_mouse(xo + x, yo + y, Coordinate::Abs)
            .unwrap();
        #[cfg(not(debug_assertions))]
        self.sleep(SAFE_WAITER);
    }

    // down -> 1 | up -> -1
    fn scroll(&mut self, x: i32, y: i32, to: i32) {
        self.move_to(x, y);
        self.enigo.scroll(to, enigo::Axis::Vertical).unwrap();
        self.sleep(SAFE_WAITER);
    }

    pub fn sleep(&self, mils: u64) {
        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(mils));
    }

    pub fn setup_maket(&mut self) {
        //kia to digit
        // self.set_kia_to(KIA::INI);
        self.set_kia_to(KIA::DIGIT);

    }

    pub fn set_kia_to(&mut self, kia: KIA) {
        // if self.cur_kia == kia {
        //     return;
        // }

        self.click(251, 35);
        self.sleep(100);

        match kia {
            // KIA::INI => {
            //     self.click(303, 81);
            //     self.cur_kia = KIA::INI
            // }
            KIA::DIGIT => {
                self.click(279, 58);
                self.sleep(100);
                self.click(483 , 620);
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

    #[inline]
    pub fn waiter_1sec_while_vm(&self, v: f64) {
        self.waiter_1sec_while(|| v != self.mem.vm());
    }
}

//volt GS
impl App {
    const VG_Y :i32 = 581;
    fn vg_more(&mut self) {
        let v = self.mem.vg();
        self.click(254 , Self::VG_Y);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_more_more(&mut self) {
        let v = self.mem.vg();
        for i in [1., 0.1, 0.01, 0.001, 0.0001] {
            if 0.96 * i <= v && v < 1. * i {
                return self.vg_more();
            }
        }
        self.click(246, Self::VG_Y);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_less(&mut self) {
        let v = self.mem.vg();
        self.click(164, Self::VG_Y);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_less_less(&mut self) {
        let v = self.mem.vg();
        for i in [0.1, 0.01, 0.001, 0.0001] {
            if 1. * i <= v && v < 1.4 * i {
                return self.vg_less();
            }
        }
        self.click(180, Self::VG_Y);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }

    fn set_vg(&mut self, indx: i32) {
        let v = self.mem.vg();
        self.click(288 , 500 + indx * 31);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }

    pub fn set_vg_to(&mut self, volt: f64) {
        let v = self.mem.vg();
        let zers = volt.log10().floor();
        if zers != v.log10().floor() {
            self.set_vg(6 + zers as i32);
        }
        let zers = 10f64.powf(zers.abs() + 3.);
        let volt = (volt * zers).floor() / zers;
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

    pub fn find_volt_by_vg1(&mut self, v_find: f64) {
        let mut last: Option<bool> = None;
        let mut vm_r = self.mem.vm_round();
        let mut vec =[0.0; 6];
        let mut n = 0;
        while (vm_r - v_find).abs() > 0.00001 {
            let last_vg = self.mem.vg();
            let diff = (vm_r - v_find).abs();

            let vm = self.mem.vm();
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
            vec[n%6] = vm_r;
            n+=1;
            let v1 = vec[0];
            let v2 = vec[1];
            if v1 != v2 && [vec[2], vec[4]].contains(&v1) && [vec[3], vec[5]].contains(&v2) {
                break;
            }
            // vec.push(vm_r);
            // if vec.len() > 6 {
            //     vec.remove(0);
            //
            // }

            let vg = self.mem.vg();
            if vg.log10().round() == vg.log10() {
                let vg_log = vg.log10()as i32;
                let indx = 6 +  vg_log;
                if vg - last_vg > 0. {
                    self.set_vg(indx);
                } else if vg - last_vg <= 0. {
                    self.set_vg(indx - 1);
                }
                self.waiter_1sec_while(|| vg != self.mem.vg());
                vm_r = self.mem.vm_round();
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
        self.click(235, 536);
        self.waiter_1sec_while(|| f != self.mem.fv())
    }
    pub fn fv1_more_more(&mut self) {
        let f = self.mem.fv();
        if (990_000. <= f && f <= 1_000_000.) || (295_000. <= f && f < 300_000.) {
            return self.fv1_more();
        }
        self.click(226, 536);
        self.waiter_1sec_while(|| f != self.mem.fv())
    }
    pub fn fv1_less(&mut self) {
        let f = self.mem.fv();
        if 100. == f {
            return;
        }
        self.click(144, 536);
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
        self.click(54 + 30 * indx, 701);

        if indx - self.freq1_bend == 1 && [300., 1000.].contains(&f) {
            self.sleep(200);
            return;
        }

        self.waiter_1sec_while(|| f != self.mem.fv());
    }

    pub fn set_fv1_to(&mut self, freq: f64) {
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

        for _ in 0..10_000 {
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
            4 => self.sa4(),
            5 => self.sa5(),
            6 => self.sa6(),
            7 => self.sa7(),
            _ => (),
        }
    }

    pub fn sa1(&mut self) {
        let sa = self.mem.sa()[0];
        self.click(352, 550);
        self.waiter_1sec_while(|| sa != self.mem.sa()[0]);
    }

    pub fn sa2(&mut self) {
        let sa = self.mem.sa()[1];
        self.click(352 + 1 * 48, 550);
        self.waiter_1sec_while(|| sa != self.mem.sa()[1]);
    }

    pub fn sa3(&mut self) {
        let sa = self.mem.sa()[2];
        self.click(352 + 2 * 48, 550);
        self.waiter_1sec_while(|| sa != self.mem.sa()[2]);
    }

    pub fn sa4(&mut self) {
        let sa = self.mem.sa()[3];
        self.click(352 + 3 * 48, 550);
        self.waiter_1sec_while(|| sa != self.mem.sa()[3]);
    }
    pub fn sa5(&mut self) {
        let sa = self.mem.sa()[5 - 1];
        self.click(352 + 4 * 48, 550);
        self.waiter_1sec_while(|| sa != self.mem.sa()[5 - 1]);
    }
    pub fn sa6(&mut self) {
        let sa = self.mem.sa()[6 - 1];
        self.click(352 + 5 * 48, 550);
        self.waiter_1sec_while(|| sa != self.mem.sa()[6 - 1]);
    }
    pub fn sa7(&mut self) {
        let sa = self.mem.sa()[7 - 1];
        self.click(496, 685);
        self.waiter_1sec_while(|| sa != self.mem.sa()[7 - 1]);
    }
}

// Modulation
impl App {
    fn m_more_more(&mut self) {
        let m = self.mem.m();
        self.click(226, 607);
        self.waiter_1sec_while(|| m != self.mem.m());
    }

    fn m_less_less(&mut self) {
        let m = self.mem.m();
        self.click(154, 607);
        self.waiter_1sec_while(|| m != self.mem.m());
    }
    fn m_less(&mut self) {
        let m = self.mem.m();
        self.click(144, 607);
        self.waiter_1sec_while(|| m != self.mem.m());
    }

    fn f_more_more(&mut self) {
        let f = self.mem.fm();
        self.click(226, 573);
        self.waiter_1sec_while(|| f != self.mem.fm());
    }
    fn f_more(&mut self) {
        let f = self.mem.fm();
        self.click(235, 573);
        self.waiter_1sec_while(|| f != self.mem.fm());
    }

    fn f_less(&mut self) {
        let f = self.mem.fm();
        self.click(144, 573);
        self.waiter_1sec_while(|| f != self.mem.fm());
    }

    pub fn set_m_to(&mut self, procent: u32) {
        let proc = procent as f64 / 100.0;
        let mut m = self.mem.m();
        while m != proc {
            if m < proc {
                self.m_more_more();
            } else if m - proc < 0.1 {
                self.m_less();
            } else {
                self.m_less_less();
            }
            m = self.mem.m();
        }
    }

    pub fn set_f_to(&mut self, freq: u32) {
        let freq = freq as f64;
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
}

// q & Fi
impl App {
    #[inline]
    pub fn scroll_q_up(&mut self) {
        self.scroll(96, 628, -1)
    }
    #[inline]
    pub fn scroll_q_down(&mut self) {
        self.scroll(96, 628, 1)
    }
    // 
    // pub fn r8_to(&mut self, i_to: f32) {
    //     let mut i = self.mem.i8();
    //     if i == i_to {
    //         return;
    //     }
    // 
    //     const ARR_LEN: usize = 4;
    //     let mut arr: [f32; 4] = [0., 1., 0., 0.]; // can't do [0f32;4]
    //     let mut n = 0;
    //     while i != i_to {
    //         let r = self.mem.r8();
    //         if i < i_to {
    //             self.scroll_r8_down();
    //         } else {
    //             self.scroll_r8_up();
    //         }
    //         self.waiter_1sec_while(|| r != self.mem.r8());
    // 
    //         self.sleep(10);
    //         i = self.mem.i8();
    //         arr[n] = self.mem.r8();
    //         n += 1;
    //         n %= ARR_LEN;
    //         let a1 = arr[0];
    //         let a2 = arr[1];
    //         if arr[2] == a1 && arr[3] == a2 {
    //             break;
    //         }
    //     }
    // 
    //     let mut difs = [0.0; 3];
    //     difs[1] = (i_to - self.mem.i8()).abs();
    // 
    //     self.scroll_r8_down();
    //     self.sleep(10);
    //     difs[2] = (i_to - self.mem.i8()).abs();
    // 
    //     self.scroll_r8_up();
    //     self.sleep(10);
    //     self.scroll_r8_up();
    //     self.sleep(10);
    //     difs[0] = (i_to - self.mem.i8()).abs();
    // 
    //     let min = match difs.iter().min_by(float_order) {
    //         Some(a) => *a,
    //         None => {
    //             self.scroll_r8_down(); //return to first position
    //             return;
    //         }
    //     };
    //     if let Some(pos) = difs.iter().position(|&x| x == min) {
    //         for _ in 0..pos {
    //             self.scroll_r8_down();
    //             self.sleep(10);
    //         }
    //     }
    // }
    // 
    // pub fn find_05volt_by_r8_revers(&mut self, reverse: bool) {
    //     let down: fn(&mut Self);
    //     let up: fn(&mut Self);
    // 
    //     if reverse {
    //         down = |app: &mut Self| app.scroll_r8_up();
    //         up = |app: &mut Self| app.scroll_r8_down();
    //     } else {
    //         down = |app: &mut Self| app.scroll_r8_down();
    //         up = |app: &mut Self| app.scroll_r8_up();
    //     }
    // 
    //     let mut volt = self.mem.vm();
    //     while volt > 1. {
    //         down(self);
    //         volt = self.mem.vm();
    //     }
    // 
    //     while volt > 0.5{
    //         let r = self.mem.r8();
    //         if volt < 0.5 {
    //             up(self);
    //         } else {
    //             down(self);
    //         }
    //         self.waiter_1sec_while(|| r != self.mem.r8());
    // 
    //         self.sleep(10);
    //         volt = self.mem.vm();
    //     }
    // 
    //     let mut difs = [0.0; 3];
    //     let vm = self.mem.vm();
    //     difs[1] = (0.5 - vm).abs();
    // 
    //     self.scroll_r8_down();
    //     self.waiter_1sec_while_vm(vm);
    //     let vm = self.mem.vm();
    //     difs[2] = (0.5 - vm).abs();
    // 
    //     self.scroll_r8_up();
    //     self.waiter_1sec_while_vm(vm);
    //     let vm = self.mem.vm();
    //     self.scroll_r8_up();
    //     self.waiter_1sec_while_vm(vm);
    //     difs[0] = (0.5 - self.mem.vm()).abs();
    // 
    //     let min = match difs.iter().min_by(float_order) {
    //         Some(a) => *a,
    //         None => {
    //             self.scroll_r8_down(); //return to first position
    //             return;
    //         }
    //     };
    //     if let Some(pos) = difs.iter().position(|&x| x == min) {
    //         for _ in 0..pos {
    //             self.scroll_r8_down();
    //             self.sleep(10);
    //         }
    //     }
    // }
    // 
    // pub fn find_max_uk_by_r8(&mut self) {
    //     // let mut last = 1.;
    //     let mut uk = self.mem.uk();
    //     loop {
    //         let u = self.mem.uk();
    //         self.scroll_r8_up();
    //         self.waiter_1sec_while(|| u != self.mem.uk());
    // 
    //         let u = self.mem.uk();
    //         if u < uk {
    //             self.scroll_r8_down();
    //             break;
    //         }
    //         uk = self.mem.uk();
    //     }
    // 
    //     let mut uk = self.mem.uk();
    //     loop {
    //         let u = self.mem.uk();
    //         self.scroll_r8_down();
    //         self.waiter_1sec_while(|| u != self.mem.uk());
    // 
    //         let u = self.mem.uk();
    //         if u < uk {
    //             self.scroll_r8_up();
    //             break;
    //         }
    //         uk = u;
    //     }
    // 
    //     let mut difs = [0.0; 3];
    //     difs[1] = self.mem.uk();
    // 
    //     self.scroll_r8_down();
    //     difs[2] = self.mem.uk();
    // 
    //     self.scroll_r8_up();
    //     self.scroll_r8_up();
    //     difs[0] = self.mem.uk();
    // 
    //     let max = match difs.iter().max_by(float_order) {
    //         Some(a) => *a,
    //         None => {
    //             self.scroll_r8_down(); //return to first position
    //             return;
    //         }
    //     };
    //     if let Some(pos) = difs.iter().position(|&x| x == max) {
    //         for _ in 0..pos {
    //             self.scroll_r8_down();
    //         }
    //     } else {
    //         self.scroll_r8_down();
    //     }
    // }
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
    pub fn open_table(&mut self, indx: i32) {
        self.click(959, 521 + indx * 33);

        while get_pos_open_table().pos() == (0, 0) {}
        #[cfg(not(debug_assertions))]
        self.sleep(1000);
        #[cfg(debug_assertions)]
        self.sleep(100);
    }

    pub fn write_table1(&mut self, col: i32) {
        self.open_table(0);

        let vm = self.mem.vm() * 1000.;
        self.write_tabl1_call(col, 0, vm);

        let f = self.mem.fv() / 1000.;
        self.write_tabl1_call(col, 1, f);

        self.close_tabl();
    }

    pub fn write_table2(&mut self, col: i32, row: i32, val: f64) {
        self.open_table(1);

        self.write_tabl2_call(col, row, val);

        self.close_tabl();
    }

    pub fn write_table3(&mut self, col: i32, val: f64) {
        self.open_table(2);

        self.write_tabl3_call(col, val);

        self.close_tabl();
    }

    pub fn write_table4_1(&mut self, col: i32, row: i32, val: f64) {
        self.open_table(3);

        self.write_tabl4_1_call(col, row, val);

        self.close_tabl();
    }
    pub fn write_table4_2(&mut self, col: i32, row: i32, val: f64) {
        self.open_table(4);

        self.write_tabl4_2_call(col, row, val);

        self.close_tabl();
    }

    pub fn write_table4_3_5(&mut self, table: u32,  col: i32, row: i32, val: f64) {
        let indx = ((table + 2) % 7) as i32;
        // let indx = match table {
        //     3=>5,
        //     4=>6,
        //     5=>0,
        //     _=>7,
        // };
        self.open_table(indx);

        self.write_tabl4_3_5_call(col, row, val);

        self.close_tabl();
    }

    pub fn write_table4_6(&mut self, col: i32, row: i32, val: f64) {
        self.open_table(1);

        self.write_tabl4_6_call(col, row, val);

        self.close_tabl();
    }
    pub fn write_table4_7(&mut self, col: i32, row: i32, val: f64) {
        self.open_table(2);

        self.write_tabl4_7_call(col, row, val);

        self.close_tabl();
    }

    pub fn write_table5(&mut self, col: i32, row: i32, val: f64) {
        self.open_table(5);

        self.write_tabl5_call(col, row, val);

        self.close_tabl();

    }
    fn write_tabl1_call(&mut self, col: i32, row: i32, val: f64) {
        let text = format!("{val:.1}");

        self.click_table(127 + col * 81, 118 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl2_call(&mut self, col: i32, row: i32, val: f64) {
        let text = format!("{val:.1}");

        self.click_table(211 + col * 81, 120 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl3_call(&mut self, col: i32, val: f64) {
        let text = format!("{val:.1}");

        self.click_table(130 + col * 82, 121);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl4_1_call(&mut self, col: i32, row: i32, val: f64) {
        let text = format!("{val:.1}");

        self.click_table(189 + col * 82, 121 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl4_2_call(&mut self, col: i32, row: i32, val: f64) {
        let text = format!("{val:.1}");

        self.click_table(189 + col * 82, 121 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl4_3_5_call(&mut self, col: i32, row: i32, val: f64) {
        let text = format!("{val:.1}");

        self.click_table(130 + col * 82, 121 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl4_6_call(&mut self, col: i32, row: i32, val: f64) {
        let text = format!("{val:.1}");

        self.click_table(189 + col * 82, 121 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl4_7_call(&mut self, col: i32, row: i32, val: f64) {
        let text = format!("{val:.1}");

        self.click_table(130 + col * 82, 121 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl5_call(&mut self, col: i32, row: i32, val: f64) {
        let text = format!("{val:.2}");

        self.click_table(208 + col * 80, 121 + 32 * row);
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

fn float_order(x: &&f32, y: &&f32) -> Ordering {
    x.partial_cmp(y).unwrap()
}

#[test]
fn test_click(){
    let mem = Meme::new();
    let mut app = App::new(mem);
    let mut vg = app.mem.vg();
    let mut i=239;
    'a: while i<254 {
        loop {
            app.click(i, 581);
            app.sleep(300);
            if vg == app.mem.vg(){
                break 'a
            }
            if vg - app.mem.vg() != 0.001 {
                break
            }
        }
        i+=1;
    }
    println!("{}",i)
}