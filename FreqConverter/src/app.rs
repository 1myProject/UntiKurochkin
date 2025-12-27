use crate::memory_viewer::{press_enter_for_exit, Meme};
use crate::open_windows::{get_pos_maket, get_pos_open_table};
use enigo::Direction::{Press, Release};
use enigo::{Button, Coordinate, Enigo, Key, Keyboard, Mouse, Settings};

pub struct App {
    enigo: Enigo,
    pub mem: Meme,
    freq1_bend: i32,
}

impl App {
    pub fn new() -> App {
        let enigo = Enigo::new(&Settings::default()).unwrap();

        // use screen_size::get_primary_screen_size;
        // let (w, h) = get_primary_screen_size().unwrap();
        //
        // let x = (w / 2) as i32;
        // let y = (h / 2) as i32;

        App {
            enigo,
            mem: Meme::new(),
            freq1_bend: 0,
        }
    }

    fn click(&mut self, x: i32, y: i32) {
        self.enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
        self.enigo.button(Button::Left, Press).unwrap();
        self.enigo.button(Button::Left, Release).unwrap();
    }

    fn click_maket(&mut self, x: i32, y: i32) {
        let (x_of, y_of) = get_pos_maket();
        self.click(x + x_of, y + y_of-7);
    }

    fn click_table(&mut self, x: i32, y: i32) {
        let (x_of, y_of) = get_pos_open_table();
        self.click(x + x_of, y + y_of);
    }

    pub fn sleep(&self, mils: u64) {
        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(mils));
    }

    pub fn setup_maket(&mut self) {
        //set V -> mV
        self.click_maket(252, 44);
        self.sleep(100);
        self.click_maket(311, 90);
        self.sleep(100);
        self.click_maket(790, 669);

        // m=30%
        for _ in 0..30 {
            self.click_maket(155, 571);
        }
    }

    fn waiter_1sec_while(&self, fun: impl Fn() -> bool) {
        for _ in 0..1000 {
            if fun() {
                break;
            }
            self.sleep(1)
        }
    }
}

//volt GS
impl App {
    fn volt1_more(&mut self) {
        let v = self.mem.vg1();
        self.click_maket(257, 606);
        while v == self.mem.vg1() {}
        self.waiter_1sec_while(|| v != self.mem.vg1());
    }
    fn volt1_more_more(&mut self) {
        let v = self.mem.vg1();
        for i in [1., 0.1, 0.01, 0.001] {
            if 0.993 * i <= v && v < 1. * i {
                return self.volt1_more();
            }
        }
        self.click_maket(248, 606);
        self.waiter_1sec_while(|| v != self.mem.vg1());
    }
    fn volt1_less(&mut self) {
        let v = self.mem.vg1();
        self.click_maket(154, 608);
        self.waiter_1sec_while(|| v != self.mem.vg1());
    }
    fn volt1_less_less(&mut self) {
        let v = self.mem.vg1();
        for i in [1., 0.1, 0.01, 0.001] {
            if 1. * i <= v && v < 1.6 * i {
                return self.volt1_less();
            }
        }
        self.click_maket(165, 608);
        self.waiter_1sec_while(|| v != self.mem.vg1());
    }

    fn set_volt1(&mut self, indx: i32) {
        let v = self.mem.vg1();
        self.click_maket(108 + indx * 37, 660);
        self.waiter_1sec_while(|| v != self.mem.vg1());
    }

    pub fn set_volt1_to(&mut self, volt: f32) {
        let v = self.mem.vg1();
        let volt = match volt {
            0.0001..0.001 => {
                if !(0.0001..0.001).contains(&v) {
                    self.set_volt1(4)
                }
                (volt * 10_000_000.).floor() / 10_000_000.
            }
            0.001..0.01 => {
                if !(0.001..0.01).contains(&v) {
                    self.set_volt1(3)
                }
                (volt * 1_000_000.).floor() / 1_000_000.
            }
            0.01..0.1 => {
                if !(0.01..0.1).contains(&v) {
                    self.set_volt1(2)
                }
                (volt * 100_000.).floor() / 100_000.
            }
            0.1..1. => {
                if !(0.1..1.).contains(&v) {
                    self.set_volt1(1)
                }
                (volt * 10_000.).floor() / 10_000.
            }
            1.0..10.0 => {
                if !(1.0..10.0).contains(&v) {
                    self.set_volt1(0)
                }
                (volt * 1_000.).floor() / 1_000.
            }
            _ => panic!("not supported volt2: {}", volt),
        };
        self.waiter_1sec_while(|| v != self.mem.vg1());

        let mut v = self.mem.vg1();
        let mut dif = (v - volt).abs() + 1.;
        while (v - volt).abs() < dif {
            dif = (v - volt).abs();
            if v < volt {
                self.volt1_more_more()
            } else {
                self.volt1_less_less()
            }
            v = self.mem.vg1();
        }

        while (v - volt).abs() > 0.000_000_1 {
            if v < volt {
                self.volt1_more();
            } else {
                self.volt1_less();
            }
            v = self.mem.vg1();
        }
    }

    fn volt2_more(&mut self) {
        let v = self.mem.vg2();
        self.click_maket(513, 607);
        while v == self.mem.vg2() {}
    }

    fn volt2_more_more(&mut self) {
        let v = self.mem.vg2();
        for i in [1., 0.1, 0.01, 0.001] {
            if 0.993 * i <= v && v < 1. * i {
                return self.volt2_more();
            }
        }
        self.click_maket(502, 607);
        while v == self.mem.vg2() {}
    }

    fn volt2_less(&mut self) {
        let v = self.mem.vg2();
        self.click_maket(410, 608);
        while v == self.mem.vg2() {}
    }

    fn volt2_less_less(&mut self) {
        let v = self.mem.vg2();
        for i in [1., 0.1, 0.01, 0.001] {
            if 1. * i <= v && v < 1.6 * i {
                return self.volt2_less();
            }
        }
        self.click_maket(418, 608);
        while v == self.mem.vg2() {}
    }

    pub fn set_volt2_to(&mut self, volt: f32) {
        let v = self.mem.vg2();
        match volt {
            0.001..0.01 => {
                if !(0.001..0.01).contains(&v) {
                    self.click_maket(513, 660)
                }
            }
            0.01..0.1 => {
                if !(0.01..0.1).contains(&v) {
                    self.click_maket(476, 660)
                }
            }
            0.1..1. => {
                if !(0.1..1.).contains(&v) {
                    self.click_maket(439, 660)
                }
            }
            1.0..10.0 => {
                if !(1.0..10.0).contains(&v) {
                    self.click_maket(402, 660)
                }
            }
            _ => panic!("not supported volt2: {}", volt),
        }
        self.waiter_1sec_while(|| v != self.mem.vg2());

        let mut v = self.mem.vg2();
        while v != volt {
            if v < volt {
                self.volt2_more_more()
            } else {
                self.volt2_less_less()
            }
            v = self.mem.vg2();
        }
    }

    pub fn find_20mv_volt_by_vg1(&mut self) {
        for _ in 0..4 {
            let vm = self.mem.vm();
            let zm = vm.log10();
            if -2. <= zm && zm < -1. {
                break;
            }

            let vg = self.mem.vg1();
            let zg = (vg.log10() - 1.).ceil().abs() as i32;
            let zg = if zg > 4 {
                4
            } else if zg < 0 {
                0
            } else {
                zg
            };
            let zm = zm.floor() as i32 + 2;
            let indx = zg + zm;
            let indx = if indx > 4 {
                4
            } else if indx < 0 {
                0
            } else {
                indx
            };
            if indx == zg {
                break;
            }
            self.set_volt1(indx);
            self.waiter_1sec_while(|| vm != self.mem.vm());
        }

        let mut last: Option<bool> = None;
        let mut vm = self.mem.vm();
        let mut vec = Vec::new();
        while (vm - 0.02).abs() > 0.000005 {
            let last_vg = self.mem.vg1();
            let diff = (vm - 0.02).abs();
            if vm < 0.02 {
                if diff > 0.002 {
                    self.volt1_more_more()
                } else {
                    self.volt1_more();
                }
                last = Some(false);
            } else {
                if diff > 0.002 {
                    self.volt1_less_less()
                } else {
                    self.volt1_less()
                }
                last = Some(true);
            }

            self.waiter_1sec_while(|| vm != self.mem.vm());
            vm = self.mem.vm();
            vec.push(vm);
            if vec.len()>6{
                vec.remove(0);
                let v1 = vec[0];
                let v2 = vec[1];
                if v1 != v2 && [vec[2], vec[4]].contains(&v1) && [vec[3], vec[5]].contains(&v2) {
                    break
                }
            }
            

            let vg = self.mem.vg1();
            if vg.log10().round() == vg.log10() {
                if vg - last_vg > 0. {
                    let indx = (vg.log10() - 1.).abs() as i32;
                    self.set_volt1(indx - 1);
                } else {
                    let indx = (vg.log10() - 1.).abs() as i32;
                    self.set_volt1(indx);
                }
            }
            vm = self.mem.vm();
        }

        let vm = self.mem.vm();
        match last {
            Some(true) => {
                self.volt1_more();
            }
            Some(false) => self.volt1_less(),
            _ => (),
        }
        self.waiter_1sec_while(|| vm != self.mem.vm());
    }
}

// freq
// const FREQ_SLEEP: u64 = 10;
impl App {
    pub fn fv1_more(&mut self) {
        let f = self.mem.fv1();
        self.click_maket(205, 501);
        while f == self.mem.fv1() {}
    }
    pub fn fv1_more_more(&mut self) {
        let f = self.mem.fv1();
        if (987000. <= f && f <= 1000000.) || (295000. <= f && f < 300000.) {
            return self.fv1_more();
        }
        self.click_maket(195, 501);
        while f == self.mem.fv1() {}
    }
    pub fn fv1_less(&mut self) {
        let f = self.mem.fv1();
        if 100. == f {
            return;
        }
        self.click_maket(154, 502);
        while f == self.mem.fv1() {}
    }
    pub fn fv1_less_less(&mut self) {
        let f = self.mem.fv1();
        if 100. == f {
            return;
        }
        if (100_000. <= f && f < 121000.) || (300_000. <= f && f < 343000.) {
            return self.fv1_less();
        }
        self.click_maket(163, 502);
        while f == self.mem.fv1() {}
    }
    pub fn set_fv1(&mut self, indx: i32) {
        let f = self.mem.fv1();
        self.click_maket(235, 502 + 21 * indx);

        if indx - self.freq1_bend == 1 && [300., 1000.].contains(&f) {
            self.sleep(200);
            return;
        }

        self.waiter_1sec_while(|| f != self.mem.fv1());
    }

    pub fn set_fv1_to(&mut self, freq: f32) {
        let f = self.mem.fv1();
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
            },
        };

        for _ in 0..10000 {
            let f = self.mem.fv1();
            if f == freq {
                break;
            } else if freq > f {
                self.fv1_more_more();
            } else {
                self.fv1_less();
            }
        }
    }

    pub fn fv2_more(&mut self) {
        let f = self.mem.fv2();
        self.click_maket(460, 501);
        loop {
            self.sleep(10);
            if f != self.mem.fv1() {
                break;
            }
        }
    }
    pub fn fv2_more_more(&mut self) {
        let f = self.mem.fv2();
        if (987000. <= f && f <= 1000000.) || (295000. <= f && f < 300000.) {
            return self.fv2_more();
        }
        self.click_maket(449, 501);
        loop {
            self.sleep(10);
            if f != self.mem.fv1() {
                break;
            }
        }
    }
    pub fn fv2_less(&mut self) {
        let f = self.mem.fv2();
        if [100.].contains(&f) {
            return;
        }
        self.click_maket(408, 502);
        loop {
            self.sleep(10);
            if f != self.mem.fv1() {
                break;
            }
        }
    }
    pub fn set_fv2(&mut self, indx: i32) {
        let f = self.mem.fv2();
        self.click_maket(489, 502 + 21 * indx);

        self.waiter_1sec_while(|| f != self.mem.fv1());
    }

    pub fn set_fv2_to_1_1mhz(&mut self) {
        const FREQ: f32 = 1_100_000.0;
        self.set_fv2(2);
        let mut f = self.mem.fv2();
        while f != FREQ {
            if FREQ > f{
                self.fv2_more_more();
            } 
            else { 
                self.fv2_less();
            }
            f = self.mem.fv2();
        }
    }
    // pub fn freq_to_max(&mut self) {
    //     self.enigo.move_mouse(156, 558, Coordinate::Abs).unwrap();
    //     self.enigo.button(Button::Left, Press).unwrap();
    //     self.enigo.move_mouse(68, 0, Coordinate::Rel).unwrap();
    //     self.enigo.button(Button::Left, Release).unwrap();
    //
    //     self.sleep(100);
    // }
}

// SA
impl App {
    pub fn set_sas(&mut self, sp_need: [i16; 4]) {
        let mut sp_orig = self.mem.sa();
        for i in 0..4 {
            loop {
                if sp_need[i] == 0 || sp_need[i] == sp_orig[i] {
                    break;
                }
                self.set_sa_indx(i);
                sp_orig = self.mem.sa();
            }
        }
    }

    fn set_sa_indx(&mut self, indx: usize) {
        match indx + 1 {
            1 => self.sa1(),
            2 => self.sa2(),
            3 => self.sa3(),
            4 => self.sa4(),
            _ => (),
        }
    }

    pub fn sa1(&mut self) {
        let sa = self.mem.sa()[0];
        self.click_maket(608, 684);
        self.waiter_1sec_while(|| sa != self.mem.sa()[0]);
    }

    pub fn sa2(&mut self) {
        let sa = self.mem.sa()[1];
        self.click_maket(605, 524);
        self.waiter_1sec_while(|| sa != self.mem.sa()[1]);
    }

    pub fn sa3(&mut self) {
        let sa = self.mem.sa()[2];
        self.click_maket(685, 557);
        self.waiter_1sec_while(|| sa != self.mem.sa()[2]);
    }

    pub fn sa4(&mut self) {
        let sa = self.mem.sa()[3];
        self.click_maket(684, 687);
        self.waiter_1sec_while(|| sa != self.mem.sa()[3]);
    }
}

// Ie
impl App {
    pub fn r5_to(&mut self, i_e: f64) {
        let i = self.mem.i_e();


        self.click_maket(324, 725);
        if i == 0.0 {
            self.enigo.scroll(1000, enigo::Axis::Vertical).unwrap();
            while i == self.mem.i_e() {}
        }

        let mut i = self.mem.i_e();
        let incr = if i < i_e {
            1
        } else if i > i_e {
            -1
        } else {
            return;
        };
        let mut last_i: f64;

        loop {
            self.enigo.scroll(incr, enigo::Axis::Vertical).unwrap();

            while i == self.mem.i_e() {}

            last_i = i;
            i = self.mem.i_e();
            if i_e == i {
                break;
            }

            if (last_i < i_e && i_e < i) || (i < i_e && i_e < last_i) {
                if (i_e - i).abs() > (i_e - last_i).abs() {
                    self.enigo.scroll(-incr, enigo::Axis::Vertical).unwrap();
                }
                break;
            }
        }
    }
}

//C3,C18,C9
impl App {
    pub fn c3_c18_to(&mut self, val: f32) {
        let mut ang = self.mem.ang_3();
        if ang == val {
            return;
        }

        self.click_maket(418, 722);
        // self.enigo.scroll(2000, enigo::Axis::Vertical).unwrap();
        // while ang == self.mem.ang_3() && ang != 0. {}
        let side = if ang < val { -1 } else { 1 };

        ang = self.mem.ang_3();
        while ang != val {
            let c = self.mem.farad_3();
            self.enigo.scroll(side, enigo::Axis::Vertical).unwrap();
            while c == self.mem.farad_3() {}
            // self.sleep(1);

            ang = self.mem.ang_3();
        }
    }
    pub fn find_max_volt_from_c9(&mut self, to_min: f32) {
        self.click_maket(505, 721);
        let mut ang = self.mem.ang_9();
        while ang > to_min {
            let c = self.mem.farad_9();
            self.enigo.scroll(1, enigo::Axis::Vertical).unwrap();
            while c == self.mem.farad_9() {}
            ang = self.mem.ang_9();
        }

        self.sleep(10);
        let mut volt = self.mem.vm();
        loop {
            let c = self.mem.farad_9();
            self.enigo.scroll(-1, enigo::Axis::Vertical).unwrap();
            while c == self.mem.farad_9() {}
            self.sleep(20);

            let v = self.mem.vm();
            if v < volt {
                // println!("{v}| {volt}");
                self.enigo.scroll(1, enigo::Axis::Vertical).unwrap();
                self.sleep(1);
                break;
            }
            volt = v;
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
// const OPEN_TABLE_SLEEP: u64 = 400;
const WRITE_TABL_CLICK_T: u64 = 100;
// const WRITE_TABL_CLICK_T: u64 = 300;
// const TABLE_FINAL_CLICK_T: u64 = 1000;
impl App {
    pub fn open_table(&mut self, num: i32) {
        if num == 11 {
            self.click_maket(877, 351);
        } else {
            let col = 1 - num % 2;
            let row = (num - 1) / 2;
            self.click_maket(835 + col * 84, 144 + row * 42);
        }

        while get_pos_open_table() == (0, 0) {}
        #[cfg(not(debug_assertions))]
        self.sleep(2000);
        #[cfg(debug_assertions)]
        self.sleep(100);
    }

    pub fn write_table1(&mut self) {
        self.open_table(1);

        let f = self.mem.fv1() / 1000.;
        self.write_tabl1_call(0, f);

        self.write_tabl1_call(1, 3.);

        let v = self.mem.vm() * 1000.;
        self.write_tabl1_call(2, v);

        let k = v / 3.;
        self.write_tabl1_call(3, k);
    }

    pub fn write_table2(&mut self, col: i32, row: i32) {
        self.open_table(2);

        let f = self.mem.fv1() / 1000.;
        self.write_tabl2_call(col, row, f);

        self.close_tabl();
    }

    pub fn write_table3(&mut self, row: i32) {
        self.open_table(3);

        let v = self.mem.vm() * 1000.;
        self.write_tabl3_call(row, v);
        self.close_tabl();
    }
    pub fn write_table4(&mut self) {
        self.open_table(4);

        self.write_tabl4_call(0, 200.0);

        let f = self.mem.fv2() / 1000.;
        self.write_tabl4_call(1, f);

        self.write_tabl4_call(2, 3.);

        let vm = self.mem.vm() * 1000.;
        self.write_tabl4_call(3, vm);

        let k = vm / 3.;
        self.write_tabl4_call(4, k);

        self.final_table();
        self.close_tabl();
    }
    pub fn write_table5(&mut self, col: i32, row: i32) {
        self.open_table(5);

        let v = self.mem.vm() * 1000.;
        self.write_tabl5_call(col, row, v);

        self.close_tabl();
    }
    pub fn write_table6(&mut self, row: i32) {
        self.open_table(6);

        let v = self.mem.vm() * 1000.;
        self.write_tabl6_call(row, v);
        self.close_tabl();
    }
    pub fn write_table7(&mut self, row: i32) {
        self.open_table(7);

        let v = self.mem.vm() * 1000.;
        self.write_tabl7_call(row, v);
        self.close_tabl();
    }
    pub fn write_table8(&mut self, col: i32) {
        self.open_table(8);
        //
        let f = self.mem.fv1() / 1000.;
        self.write_tabl8_call(col, 0, f);

        let v = self.mem.vg1() * 1000.;
        self.write_tabl8_call(col, 1, v);

        self.close_tabl();
    }
    pub fn write_table9_1(&mut self, col: i32) {
        self.open_table(9);

        let f = self.mem.fv1() / 1000.;
        self.write_tabl9_call(col, 0, f);

        let v = self.mem.vm() * 1000.;
        self.write_tabl9_call(col, 1, v);

        self.close_tabl();
    }
    pub fn write_table9_2(&mut self, col: i32, f: f32, left: bool) {
        self.open_table(9);

        let f = f / 1000.;
        let mut text = to_human_value(format!("{f:.1}"));
        if left {
            text.push_str("-");
        }

        self.click_table(127 + col * 75, 185); // row = 2
        self.sleep(WRITE_TABL_CLICK_T);
        for _ in 0..7{
            self.enigo.key(Key::RightArrow, Press).unwrap();
            self.sleep(1);
            self.enigo.key(Key::RightArrow, Release).unwrap();
            self.sleep(10);
        }
        self.enigo.text(text.as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);


        self.close_tabl();
    }
    pub fn write_table9_3(&mut self, col: i32, f: f32) {
        self.open_table(9);

        let f = f / 1000.;
        self.write_tabl9_call(col, 3, f);

        self.close_tabl();
    }
    pub fn write_table10(&mut self, col: i32, c6: f32, f0: f32) {
        self.open_table(10);

        let f = self.mem.fv1() / 1000.;
        self.write_tabl10_call(col, 0, f);

        let v = self.mem.vm() * 1000.;
        self.write_tabl10_call(col, 1, v);

        let ang = self.mem.ang_9();
        self.write_tabl10_call(col, 2, ang);

        let c9 = self.mem.farad_9() * 1e12;
        self.write_tabl10_call(col, 3, c9);

        let dc = c9 - c6;
        self.write_tabl10_call(col, 4, dc);

        let df = f0 / 1000. - f;
        self.write_tabl10_call(col, 5, df);

        self.close_tabl();
    }
    pub fn write_table11(&mut self, row: i32, u_s: f32) {
        self.open_table(11);

        loop{
            let v = self.mem.vg1() * 1000.;

            if u_s != 0. {
                if v <= u_s{
                    continue;
                }
                self.write_tabl11_call(0, row, v);
                let s = 20. * (v / u_s / 1000.).log10();
                self.write_tabl11_call(1, row, s);
                break;
            }
            else {
                self.write_tabl11_call(0, row, v);
                break;
            }
        }


        self.close_tabl();
    }

    fn write_tabl1_call(&mut self, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(270, 131 + row * 30);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl2_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(197 + col * 79, 163 + row * 32);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl3_call(&mut self, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.sleep(WRITE_TABL_CLICK_T);
        self.click_table(242, 133 + row * 30);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl4_call(&mut self, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(143, 131 + row * 30);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl5_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(136 + 98 * col, 163 + 31 * row);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl6_call(&mut self, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(176, 132 + 32 * row);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl7_call(&mut self, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(179, 131 + 32 * row);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl8_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(98 + col * 93, 133 + row * 26);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl9_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(127 + col * 75, 133 + row * 26);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    pub fn write_tabl10_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.1}");

        self.click_table(131 + col * 89, 133 + row * 27);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    fn write_tabl11_call(&mut self, col: i32, row: i32, val: f32) {
        let text = format!("{val:.2}");

        self.click_table(222 + col * 89, 120 + row * 26);
        self.sleep(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep(WRITE_TABL_CLICK_T);
    }

    pub fn close_tabl(&mut self) {
        self.click_table(120, 42);
        while get_pos_open_table() != (0, 0) {}
    }

    pub fn final_table(&mut self) {
        // #[cfg(not(debug_assertions))]
        {
            use crate::open_windows::get_pos_was_saved;
            self.click_table( 47, 42);

            while get_pos_was_saved() == (0, 0) {}
            let (x, y) = get_pos_was_saved();
            self.click(x + 322, y + 126);
            while get_pos_was_saved() != (0, 0) {}
        }
    }
}
