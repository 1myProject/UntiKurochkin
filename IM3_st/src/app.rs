use crate::memory_viewer::{press_enter_for_exit, Meme, SA_COUNT};
use crate::open_windows::{get_pos_etap6, get_pos_maket, get_pos_open_table};
use enigo::Button::Left;
use enigo::Coordinate::Abs;
use enigo::{Direction, Enigo, Key, Keyboard, Mouse, Settings};

#[derive(PartialEq)]
pub enum KIA {
    DIGIT,
    // INI,
    OSC,
}

pub struct App {
    enigo: Enigo,
    pub mem: Meme,
    freq1_bend: i32,
    cur_kia: KIA,
}

const SAFE_WAITER: u64 = 50;
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
        self.enigo.button(Left, Direction::Click).unwrap();
        #[cfg(not(debug_assertions))]
        self.sleep(SAFE_WAITER);
    }

    fn click_table(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_open_table().pos();
        self.enigo.move_mouse(xo + x, yo + y, Abs).unwrap();
        self.enigo.button(Left, Direction::Click).unwrap();
        #[cfg(not(debug_assertions))]
        self.sleep(SAFE_WAITER);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_maket().pos();
        self.enigo.move_mouse(xo + x, yo + y, Abs).unwrap();
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
            KIA::DIGIT => {
                self.click(279, 58);
                self.sleep(100);
                self.click(483, 620);
            }
            KIA::OSC => {
                self.click(279, 125);
                self.sleep(1000);
            }
        }
        self.cur_kia = kia;
    }
    pub fn set_to_maket2(&mut self) {
        self.click(183, 37);
        self.sleep(100);
        self.click(228, 102);
        self.sleep(100);
        self.click(821, 104);
        self.sleep(100);

        self.waiter_1sec_while(|| (0, 0) != get_pos_etap6().pos());

        let (xo, yo) = get_pos_etap6().pos();

        self.enigo.move_mouse(xo + 358, yo + 235, Abs).unwrap();
        self.enigo.button(Left, Direction::Click).unwrap();
        self.sleep(200);
        self.waiter_1sec_while(|| false != get_pos_maket().is_active);
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

const MORE: i32 = 254;
const MORE_MORE: i32 = 245;
const LESS: i32 = 164;
const LESS_LESS: i32 = 180;
//volt GS
impl App {
    const VG_Y: i32 = 581;
    fn vg_more(&mut self) {
        let v = self.mem.vg();
        self.click(MORE, Self::VG_Y);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_more_more(&mut self) {
        let v = self.mem.vg();
        for i in [1., 0.1, 0.01, 0.001] {
            if 0.98 * i <= v && v < 1. * i {
                return self.vg_more();
            }
        }
        self.click(MORE, Self::VG_Y);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_less(&mut self) {
        let v = self.mem.vg();
        self.click(LESS, Self::VG_Y);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }
    fn vg_less_less(&mut self) {
        let v = self.mem.vg();
        for i in [0.1, 0.01, 0.001] {
            if 1. * i <= v && v < 1.6 * i {
                return self.vg_less();
            }
        }
        self.click(LESS_LESS, Self::VG_Y);
        self.waiter_1sec_while(|| v != self.mem.vg());
    }

    fn set_vg(&mut self, indx: i32) {
        let v = self.mem.vg();
        self.click(288, 500 + indx * 31);
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
}

// freq
impl App {
    const FV_Y: i32 = 493;
    pub fn fv1_more(&mut self) {
        let f = self.mem.fv();
        self.click(MORE, Self::FV_Y);
        self.waiter_1sec_while(|| f != self.mem.fv())
    }
    pub fn fv1_more_more(&mut self) {
        let f = self.mem.fv();
        if (990_000. <= f && f <= 1_000_000.) || (295_000. <= f && f < 300_000.) {
            return self.fv1_more();
        }
        self.click(MORE_MORE, Self::FV_Y);
        self.waiter_1sec_while(|| f != self.mem.fv())
    }
    pub fn fv1_less(&mut self) {
        let f = self.mem.fv();
        if 300. == f {
            return;
        }
        self.click(LESS, Self::FV_Y);
        self.waiter_1sec_while(|| f != self.mem.fv())
    }
    pub fn set_fv1(&mut self, indx: i32) {
        let f = self.mem.fv();
        self.click(180 + 30 * indx, 649);

        if indx - self.freq1_bend == 1 && [300., 1000.].contains(&f) {
            self.sleep(200);
            return;
        }

        self.waiter_1sec_while(|| f != self.mem.fv());
    }

    pub fn set_fv1_to(&mut self, freq: f64) {
        let f = self.mem.fv();
        let freqs = [300_000., 1_000_000., 3_000_000., 10_000_000.];
        let len = freqs.len() - 1;
        let mut is_not = true;
        for i in 0..len {
            let diap = freqs[i]..freqs[i + 1];
            if !diap.contains(&freq) {
                continue;
            }
            if !diap.contains(&f) {
                self.set_fv1(i as i32);
            }
            is_not = false;
            break;
        }
        if is_not {
            eprint!("!!!!!not supported frequency!: {freq}\nexit: ");
            press_enter_for_exit();
            panic!("not supported frequency!");
        }
        for _ in 0..100_000 {
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
        self.enigo
            .move_mouse(LESS_LESS + x, Self::FV_Y + y, Abs)
            .unwrap();
        self.enigo.button(Left, Direction::Press).unwrap();
        self.enigo
            .move_mouse(MORE_MORE + x, Self::FV_Y + y, Abs)
            .unwrap();
        self.enigo.button(Left, Direction::Release).unwrap();
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
        let sa = self.mem.sa()[indx];
        const POINTS: [(i32, i32); SA_COUNT] = [
            (777, 126), //1
            (749, 269), //2
            (777, 434), //3
            (803, 262), //4
            (777, 126), //1a
            (777, 284), //2a
            (773, 433), //3a
        ];
        let (x, y) = POINTS[indx];
        self.click(x, y);
        self.waiter_1sec_while(|| sa != self.mem.sa()[indx]);
    }

    pub fn sa_num(&mut self, num: usize) {
        if num > 0 && num < 7 {
            self.set_sa_indx(num - 1);
        }
    }
}

// q & Fi
impl App {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_nepr(&mut self) {
        self.click(65, 618);
        self.sleep(SAFE_WAITER)
    }
    #[allow(clippy::wrong_self_convention)]
    pub fn to_impl(&mut self) {
        self.click(65, 618 + 23);
        self.sleep(SAFE_WAITER)
    }

    #[inline]
    pub fn scroll_q_up(&mut self) {
        self.scroll(96, 628, -1)
    }
    #[inline]
    pub fn scroll_q_down(&mut self) {
        self.scroll(96, 628, 1)
    }
    #[inline]
    pub fn scroll_fi_up(&mut self) {
        self.scroll(136, 625, -1)
    }
    #[inline]
    pub fn scroll_fi_down(&mut self) {
        self.scroll(136, 625, 1)
    }

    pub fn fi_to(&mut self, fi_khz_to: u32) {
        let fi_to = fi_khz_to * 1000;
        let mut fi = self.mem.fi();
        while fi != fi_to {
            let f = self.mem.fi();
            if fi > fi_to {
                self.scroll_fi_down()
            } else {
                self.scroll_fi_up()
            }
            self.waiter_1sec_while(|| f != self.mem.fi());

            fi = self.mem.fi();
        }
    }
    pub fn q_to(&mut self, q_to: f32) {
        let mut q = self.mem.q();
        while q != q_to {
            let qq = self.mem.q();
            if q > q_to {
                self.scroll_q_down()
            } else {
                self.scroll_q_up()
            }
            self.waiter_1sec_while(|| qq != self.mem.q());

            q = self.mem.q();
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
    pub fn open_table(&mut self, indx: i32) {
        self.click(778, 532 + indx * 32);

        while get_pos_open_table().pos() == (0, 0) {}
        #[cfg(not(debug_assertions))]
        self.sleep(1000);
        #[cfg(debug_assertions)]
        self.sleep(100);
    }

    pub fn write_table1(&mut self, col: i32, row: i32, val: f64) {
        self.open_table(0);

        let sa1 = self.mem.sa()[0];
        self.write_tabl1_5_call(sa1, col, row, val);

        self.close_tabl();
    }
    pub fn write_table2(&mut self, col: i32, val: f64) {
        self.open_table(1);

        self.write_tabl2_4_6_7_call(col, 0, val);
        self.write_tabl2_4_6_7_call(col, 1, val);

        self.close_tabl();
    }
    pub fn write_table3(&mut self, col: i32, row: i32, val: f64) {
        self.open_table(2);

        self.write_tabl3_call(col, row, val);

        self.close_tabl();
    }
    pub fn write_table4(&mut self, col: i32, val1: f64, val2: f64) {
        self.open_table(3);

        self.write_tabl2_4_6_7_call(col, 0, val1);
        self.write_tabl2_4_6_7_call(col, 1, val2);

        self.close_tabl();
    }
    pub fn write_table5(&mut self, col: i32, row: i32, val: f64) {
        self.open_table(0);

        let sa2 = self.mem.sa()[5];
        self.write_tabl1_5_call(sa2, col, row, val);

        self.close_tabl();
    }
    pub fn write_table6(&mut self, col: i32, val: f64) {
        self.open_table(1);

        self.write_tabl2_4_6_7_call(col, 0, val);
        self.write_tabl2_4_6_7_call(col, 1, val);

        self.close_tabl();
    }
    pub fn write_table7(&mut self, col: i32, row: i32, val: f64) {
        self.open_table(2);

        self.write_tabl2_4_6_7_call(col, row, val);

        self.close_tabl();
    }

    #[inline]
    fn write_tabl1_5_call(&mut self, sa: i16, col: i32, row: i32, val: f64) {
        let text = format!("{val:.1}");
        let mut x = 139;
        let mut y = 138;
        if sa % 2 == 0 {
            x += 345;
        }
        if sa > 2 {
            y += 225;
        }
        self.write_call(x + col * 80, y + row * 33, text);
    }
    #[inline]
    fn write_tabl2_4_6_7_call(&mut self, col: i32, row: i32, val: f64) {
        let text = format!("{val:.1}");

        self.write_call(133 + col * 81, 130 + row * 32, text);
    }
    #[inline]
    fn write_tabl3_call(&mut self, col: i32, row: i32, val: f64) {
        let text = format!("{val:.1}");

        self.write_call(244 + col * 82, 130 + row * 32, text);
    }

    fn write_call(&mut self, x: i32, y: i32, text: String) {
        self.click_table(x, y);
        self.click_table(x, y);
        self.enigo.key(Key::Delete, Direction::Click).unwrap();

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
            self.enigo.button(Left, Direction::Click).unwrap();
            while get_pos_was_saved().pos() != (0, 0) {}
        }
    }
}
