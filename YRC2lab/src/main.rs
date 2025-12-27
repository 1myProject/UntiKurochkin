mod license;
mod memory_viwer;
mod step_helper;

use crate::memory_viwer::Meme;
use crate::step_helper::{find_from_volt_plus, find_max_volt, find_min_volt, set_to_freq};
use crate::memory_viwer::press_enter_for_exit;
use enigo::{
    Button, Coordinate,
    Direction::{Press, Release},
    Enigo, Keyboard, Mouse, Settings,
};
use std::time::Instant;

#[derive(PartialEq)]
enum Screen {
    LAB,
    OTCH1,
    OTCH2,
}

impl Screen {
    fn is_lab(&self) -> bool {
        *self == Screen::LAB
    }

    fn is_otch1(&self) -> bool {
        *self == Screen::OTCH1
    }

    fn is_otch2(&self) -> bool {
        *self == Screen::OTCH2
    }
}

struct App {
    enigo: Enigo,
    x: i32,
    y: i32,

    screen: Screen,
    freq_bend: i32,
}

impl App {
    fn new() -> App {
        use screen_size::get_primary_screen_size;
        let enigo = Enigo::new(&Settings::default()).unwrap();

        let (w, h) = get_primary_screen_size().unwrap();

        let x = ((w - 985) / 2) as i32;
        let y = ((h - 758) / 2) as i32;

        App {
            enigo,
            x,
            y,
            screen: Screen::LAB,
            freq_bend: 0,
        }
    }

    fn click(&mut self, x: i32, y: i32) {
        let Self {
            enigo,
            x: base_x,
            y: base_y,
            screen: _,
            freq_bend: _,
        } = self;
        enigo
            .move_mouse(*base_x + x, *base_y + y, Coordinate::Abs)
            .unwrap();
        enigo.button(Button::Left, Press).unwrap();
        enigo.button(Button::Left, Release).unwrap();
    }

    fn sleep_click(&self, mils: u64) {
        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(mils));
    }

    fn setup_maket(&mut self){
        //set V -> mV
        self.click(804,689);

        //set M -> 0%
        for _ in 0..60{
            self.click(144, 624);
        }
    }
}

//volt GS
impl App {
    fn set_volt_to_10mv(&mut self) {
        if !self.screen.is_lab() {
            return;
        }
        self.click(266, 659);
        self.sleep_click(300);
    }

    fn set_volt_to_25mv(&mut self) {
        if !self.screen.is_lab() {
            return;
        }
        self.set_volt_to_10mv();
        for _ in 0..150 {
            self.click(203, 661);
            self.sleep_click(1);
        }
    }

    fn set_volt_to_1v(&mut self) {
        self.click(265, 681);
        self.sleep_click(300);
        self.enigo
            .move_mouse(self.x + 156, self.y + 661, Coordinate::Abs)
            .unwrap();
        self.enigo.button(Button::Left, Press).unwrap();
        self.enigo.move_mouse(68, 0, Coordinate::Rel).unwrap();
        self.enigo.button(Button::Left, Release).unwrap();

        self.sleep_click(100);
    }
}

// freq
// const FREQ_SLEEP: u64 = 10;
impl App {
    fn click_freq_more(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let f = mem.get_freq();
        // if [300., 1000., 3000.].contains(&f){
        //     return;
        // }
        self.click(233, 557);
        loop {
            // self.sleep_click(1);
            if f != mem.get_freq() {
                break;
            }
        }
    }

    fn click_freq_less(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let f = mem.get_freq();
        if [100.].contains(&f) {
            return;
        }
        // if f == 100.{
        //     panic!("less")
        // }
        self.click(144, 558);
        loop {
            // self.sleep_click(1);
            if f != mem.get_freq() {
                break;
            }
        }
    }

    fn set_freq(&mut self, indx: i32, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let f = mem.get_freq();
        // if indx == self.freq_bend && [100., 300.,1000.].contains(&f){
        //     return;
        // }
        self.click(52 + 31 * indx, 720);
        if (self.freq_bend, indx, f) == (0, 1, 300.) {
            self.sleep_click(200);
            return;
        }

        if (self.freq_bend, indx, f) == (1, 2, 1000.) {
            self.sleep_click(200);
            return;
        }
        loop {
            self.sleep_click(1);
            if f != mem.get_freq() {
                break;
            }
        }
    }

    fn freq_to_max(&mut self) {
        if !self.screen.is_lab() {
            return;
        }

        self.enigo
            .move_mouse(self.x + 156, self.y + 558, Coordinate::Abs)
            .unwrap();
        self.enigo.button(Button::Left, Press).unwrap();
        self.enigo.move_mouse(68, 0, Coordinate::Rel).unwrap();
        self.enigo.button(Button::Left, Release).unwrap();

        self.sleep_click(100);
    }
}

// SA
// const SA_SLEEP: u64 = 300;
// const SA9_TIME: u64 = 800;
impl App {
    fn set_sas(&mut self, mem: &Meme, sp_need: [i16; 10]) {
        let mut sp_orig = mem.get_sa();
        for i in 0..10 {
            loop {
                if sp_need[i] == 0 || sp_need[i] == sp_orig[i] {
                    break;
                }
                self.set_sa_indx(i, mem);
                sp_orig = mem.get_sa();
            }
        }
    }

    fn set_sa_indx(&mut self, indx: usize, mem: &Meme) {
        match indx + 1 {
            1 => self.sa1(mem),
            2 => self.sa2(mem),
            3 => self.sa3(mem),
            4 => self.sa4(mem),
            5 => self.sa5(mem),
            6 => self.sa6(mem),
            7 => self.sa7(mem),
            8 => self.sa8(mem),
            9 => self.sa9(mem),
            10 => self.sa10(mem),
            _ => (),
        }
    }

    #[inline]
    fn sa1(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let sa = mem.get_sa()[0];
        self.click(361, 520);
        for _ in 0..1000 {
            self.sleep_click(1);
            if mem.get_sa()[0] != sa {
                break;
            }
        }
    }
    #[inline]
    fn sa2(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let sa = mem.get_sa()[1];
        self.click(447, 543);
        for _ in 0..1000 {
            self.sleep_click(1);
            if mem.get_sa()[1] != sa {
                break;
            }
        }
    }
    #[inline]
    fn sa3(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let sa = mem.get_sa()[2];
        self.click(571, 547);
        for _ in 0..1000 {
            self.sleep_click(1);
            if mem.get_sa()[2] != sa {
                break;
            }
        }
    }
    #[inline]
    fn sa4(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let sa = mem.get_sa()[3];
        self.click(651, 527);
        for _ in 0..1000 {
            self.sleep_click(1);
            if mem.get_sa()[3] != sa {
                break;
            }
        }
    }
    #[inline]
    fn sa5(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let sa = mem.get_sa()[4];
        self.click(700, 530);
        for _ in 0..1000 {
            self.sleep_click(1);
            if mem.get_sa()[4] != sa {
                break;
            }
        }
    }
    #[inline]
    fn sa6(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let sa = mem.get_sa()[5];
        self.click(361, 681);
        for _ in 0..1000 {
            self.sleep_click(1);
            if mem.get_sa()[5] != sa {
                break;
            }
        }
    }
    #[inline]
    fn sa7(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let sa = mem.get_sa()[6];
        self.click(417, 669);
        for _ in 0..1000 {
            self.sleep_click(1);
            if mem.get_sa()[6] != sa {
                break;
            }
        }
    }
    #[inline]
    fn sa8(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let sa = mem.get_sa()[7];
        self.click(510, 677);
        for _ in 0..1000 {
            self.sleep_click(1);
            if mem.get_sa()[7] != sa {
                break;
            }
        }
    }
    #[inline]
    fn sa9(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let sa = mem.get_sa()[8];
        self.click(593, 670);
        for _ in 0..1000 {
            self.sleep_click(1);
            if mem.get_sa()[8] != sa {
                break;
            }
        }
    }
    #[inline]
    fn sa10(&mut self, mem: &Meme) {
        if !self.screen.is_lab() {
            return;
        }
        let sa = mem.get_sa()[9];
        self.click(701, 669);
        for _ in 0..1000 {
            self.sleep_click(1);
            if mem.get_sa()[9] != sa {
                break;
            }
        }
    }
}


fn to_human_value(text: String) -> String {
    if !text.contains("."){
        return text;
    }
    let text = text.trim_end_matches('0');
    if text.ends_with("."){
        text.trim_end_matches(".").to_string()
    }else { 
        text.to_string()
    }
}
//table
const WRITE_TABL_CLICK_T: u64 = 500;
const WRITE_TABL_TXT_T: u64 = 300;
const TABLE_FINAL_CLICK_T: u64 = 1000;
impl App {
    #[inline]
    fn open_tabl1(&mut self) {
        if !self.screen.is_lab() {
            return;
        }
        self.click(843, 256);

        self.screen = Screen::OTCH1;
        self.sleep_click(500);
    }
    #[inline]
    fn open_tabl2(&mut self) {
        if !self.screen.is_lab() {
            return;
        }
        self.click(913, 260);
        self.screen = Screen::OTCH1;
        self.sleep_click(500);
    }
    #[inline]
    fn open_tabl3(&mut self) {
        if !self.screen.is_lab() {
            return;
        }
        self.click(840, 296);
        self.screen = Screen::OTCH1;
        self.sleep_click(500);
    }
    #[inline]
    fn open_tabl4(&mut self) {
        if !self.screen.is_lab() {
            return;
        }
        self.click(912, 295);
        self.screen = Screen::OTCH2;
        self.sleep_click(500);
    }
    #[inline]
    fn open_tabl5(&mut self) {
        if !self.screen.is_lab() {
            return;
        }
        self.click(844, 337);
        self.screen = Screen::OTCH2;
        self.sleep_click(500);
    }
    #[inline]
    fn open_tabl6(&mut self) {
        if !self.screen.is_lab() {
            return;
        }
        self.click(918, 331);
        self.screen = Screen::OTCH2;
        self.sleep_click(500);
    }

    fn close_otch(&mut self) {
        match self.screen {
            Screen::LAB => {
                println!("is not otch");
                return;
            }
            Screen::OTCH1 => {
                self.click(716, 277);
            }
            Screen::OTCH2 => {
                self.click(906, 656);
            }
        }
        self.screen = Screen::LAB;
        self.sleep_click(500);
    }

    #[inline]
    fn write_tb1(&mut self, mem: &Meme, c: i32, r: i32) {
        let freq = mem.get_freq();

        self.open_tabl1();

        self.write_tabl1_call(c, r, freq);

        self.close_otch();
    }

    #[inline]
    fn write_tb2(&mut self, mem: &Meme, c: i32, r: i32) {
        let freq = mem.get_freq();
        self.open_tabl2();
        self.write_tabl2_call(c * 2, r, freq);
        self.close_otch();

        let volt = mem.get_volts() * 100.;
        self.open_tabl2();
        self.write_tabl2_call(c * 2 + 1, r, volt);
        self.close_otch();
    }

    #[inline]
    fn write_tb3(&mut self, mem: &Meme, c: i32, r: i32) {
        let freq = mem.get_freq();
        self.open_tabl3();
        self.write_tabl3_call(c * 2, r, freq);
        self.close_otch();

        let volt = mem.get_volts() * 1000.;
        self.open_tabl3();
        self.write_tabl3_call(c * 2 + 1, r, volt);
        self.close_otch();
    }

    #[inline]
    fn write_tb4(&mut self, mem: &Meme, c: i32, r: i32) {
        let volt = mem.get_volts() * 1000.;
        self.open_tabl4();
        self.write_tabl4_call(c, r, volt);
        self.close_otch();
    }

    #[inline]
    fn write_tb5(&mut self, mem: &Meme, c: i32, r: i32) {
        let freq = mem.get_freq();
        self.open_tabl5();
        self.write_tabl5_call(c * 2, r, freq);
        self.close_otch();

        let volt = mem.get_volts() * 1000.;
        self.open_tabl5();
        self.write_tabl5_call(c * 2 + 1, r, volt);
        self.close_otch();
    }

    #[inline]
    fn write_tb6(&mut self, mem: &Meme, c: i32) {
        let freq = mem.get_freq();
        self.open_tabl6();
        self.write_tabl6_call(c, 0, freq);
        self.close_otch();

        let volt = mem.get_volts() * 1000.;
        self.open_tabl6();
        self.write_tabl6_call(c, 1, volt);
        self.close_otch();
    }

    #[inline]
    fn write_tabl1_call(&mut self, col: i32, row: i32, val: f32) {
        if !self.screen.is_otch1() {
            println!("is not otch1");
            return;
        }

        let text = if val % 1. == 0. {
            format!("{val:.0}")
        } else {
            format!("{val:.1}")
        };

        self.click(279 + 112 * col, 230 + 27 * row);
        self.sleep_click(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep_click(WRITE_TABL_TXT_T);
    }
    #[inline]
    fn write_tabl2_call(&mut self, col: i32, row: i32, val: f32) {
        if !self.screen.is_otch1() {
            println!("is not otch1");
            return;
        }

        let text = if val % 1. == 0. {
            format!("{val:.0}")
        } else {
            format!("{val:.1}")
        };

        self.click(121 + 94 * col, 529 + 28 * row);
        self.sleep_click(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep_click(WRITE_TABL_TXT_T);
    }
    #[inline]

    fn write_tabl3_call(&mut self, col: i32, row: i32, val: f32) {
        if !self.screen.is_otch1() {
            println!("is not otch1");
            return;
        }

        let text = if val % 1. == 0. {
            format!("{val:.0}")
        } else {
            format!("{val:.1}")
        };

        self.click(121 + 94 * col, 698 + 28 * row);
        self.sleep_click(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep_click(WRITE_TABL_TXT_T);
    }
    #[inline]
    fn write_tabl4_call(&mut self, col: i32, row: i32, val: f32) {
        if !self.screen.is_otch2() {
            println!("is not otch2");
            return;
        }

        let text = format!("{val:.2}");

        self.click(212 + 108 * col, 203 + 28 * row);
        self.sleep_click(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep_click(WRITE_TABL_TXT_T);
    }
    #[inline]
    fn write_tabl5_call(&mut self, col: i32, row: i32, val: f32) {
        if !self.screen.is_otch2() {
            println!("is not otch2");
            return;
        }

        let text = if val > 1000. || val % 1. == 0. {
            format!("{val:.0}")
        } else {
            format!("{val:.1}")
        };

        self.click(91 + 80 * col, 435 + 27 * row);
        self.sleep_click(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep_click(WRITE_TABL_TXT_T);
    }
    #[inline]
    fn write_tabl6_call(&mut self, col: i32, row: i32, val: f32) {
        if !self.screen.is_otch2() {
            println!("is not otch2");
            return;
        }

        let text = if val % 1. == 0. {
            format!("{val:.0}")
        } else {
            format!("{val:.1}")
        };

        self.click(221 + 60 * col, 706 + 43 * row);
        self.sleep_click(WRITE_TABL_CLICK_T);
        self.enigo.text(to_human_value(text).as_str()).unwrap();
        self.sleep_click(WRITE_TABL_TXT_T);
    }

    #[inline]
    fn final_tabl1(&mut self) {
        if !self.screen.is_otch1() {
            println!("is not otch1");
            return;
        }
        self.click(706, 150);
        self.sleep_click(TABLE_FINAL_CLICK_T)
    }
    #[inline]
    fn final_tabl2(&mut self) {
        if !self.screen.is_otch1() {
            println!("is not otch1");
            return;
        }
        self.click(705, 186);
        self.sleep_click(TABLE_FINAL_CLICK_T)
    }
    #[inline]
    fn final_tabl3(&mut self) {
        if !self.screen.is_otch1() {
            println!("is not otch1");
            return;
        }
        self.click(705, 223);
        self.sleep_click(TABLE_FINAL_CLICK_T)
    }
    #[inline]
    fn final_tabl4(&mut self) {
        if !self.screen.is_otch2() {
            println!("is not otch2");
            return;
        }
        self.click(788, 656);
        self.sleep_click(TABLE_FINAL_CLICK_T)
    }
    #[inline]
    fn final_tabl5(&mut self) {
        if !self.screen.is_otch2() {
            println!("is not otch2");
            return;
        }
        self.click(787, 682);
        self.sleep_click(TABLE_FINAL_CLICK_T)
    }
    #[inline]
    fn final_tabl6(&mut self) {
        if !self.screen.is_otch2() {
            println!("is not otch2");
            return;
        }
        self.click(788, 710);
        self.sleep_click(TABLE_FINAL_CLICK_T)
    }
}

fn step1(app: &mut App, mem: &Meme) {
    let st = Instant::now();

    app.set_sas(mem, [1, 0, 1, 2, 2, 0, 0, 1, 1, 1]);
    app.set_sas(mem, [1, 0, 1, 2, 1, 0, 0, 1, 1, 1]);
    app.set_sas(mem, [1, 0, 1, 2, 2, 0, 0, 1, 1, 1]);

    app.set_volt_to_10mv();

    for sa5 in 0..2 {
        let sa5 = 1 - sa5;
        app.set_freq(1, mem);

        let volt = find_max_volt(app, mem);

        app.write_tb1(mem, sa5 * 2, 0);

        app.write_tb1(mem, sa5 * 2 + 1, 0);

        let freq_c = mem.get_freq();
        for (n, k) in [0.9, 0.8, 0.7, 0.5, 0.3, 0.1].into_iter().enumerate() {
            set_to_freq(app, mem, freq_c);
            find_from_volt_plus(app, mem, volt * k, true);

            app.write_tb1(mem, sa5 * 2, (n + 1) as i32);

            set_to_freq(app, mem, freq_c);
            find_from_volt_plus(app, mem, volt * k, false);

            app.write_tb1(mem, sa5 * 2 + 1, (n + 1) as i32);
        }

        app.sa5(mem);
    }

    app.open_tabl1();
    app.final_tabl1();
    app.close_otch();

    println!("Total time: {}s", st.elapsed().as_secs());
}

fn step2(app: &mut App, mem: &Meme) {
    let st = Instant::now();

    app.set_sas(mem, [1, 1, 1, 1, 1, 0, 0, 1, 1, 1]);

    app.set_freq(1, mem);
    app.set_volt_to_10mv();

    for i in 0..5 {
        find_max_volt(app, mem);

        app.write_tb2(mem, i, 0);
        app.sa2(mem);
    }

    app.set_sas(mem, [1, 1, 1, 1, 2, 0, 0, 1, 1, 1]);
    app.set_freq(1, mem);

    for i in 0..5 {
        find_max_volt(app, mem);

        app.write_tb2(mem, i, 1);
        app.sa2(mem);
    }

    app.open_tabl2();
    app.final_tabl2();
    app.close_otch();

    println!("Total time: {}s", st.elapsed().as_secs());
}

fn step3(app: &mut App, mem: &Meme) {
    let st = Instant::now();

    app.set_sas(mem, [1, 0, 1, 2, 1, 0, 0, 1, 1, 2]);
    app.set_freq(0, mem);
    app.set_volt_to_25mv();

    for row in 0..2 {
        set_to_freq(app, mem, 200.);

        app.write_tb3(mem, 0, row);

        app.set_freq(1, mem);
        find_max_volt(app, mem);
        let freq2 = mem.get_freq();

        app.write_tb3(mem, 1, row);

        // for _ in 0..2 {
        //     app.click_freq_more()
        // }

        find_min_volt(app, mem);
        let freq4 = mem.get_freq();

        let freq3 = (freq2 + freq4) / 2.;
        let freq3 = (freq3 * 10.).round() / 10.;
        set_to_freq(app, mem, freq3);
        app.write_tb3(mem, 2, row);

        set_to_freq(app, mem, freq4);
        app.write_tb3(mem, 3, row);

        set_to_freq(app, mem, 800.);
        app.write_tb3(mem, 4, row);

        app.sa5(mem);
    }

    app.open_tabl3();
    app.final_tabl3();
    app.close_otch();

    println!("Total time: {}s", st.elapsed().as_secs());
}

fn step4(app: &mut App, mem: &Meme) {
    let st = Instant::now();

    let sa = [2, 1, 1, 1, 1, 0, 0, 1, 1, 1];

    app.set_sas(mem, sa.clone());
    set_to_freq(app, mem, 465.);
    app.set_volt_to_1v();
    app.sleep_click(200);

    for sa2 in 0..5 {
        app.write_tb4(mem, sa2, 0);
        app.sa2(mem);
        app.sleep_click(200);
    }
    app.sa4(mem);
    app.sleep_click(200);

    for sa2 in 0..5 {
        app.write_tb4(mem, sa2, 2);
        app.sa2(mem);
        app.sleep_click(200);
    }
    app.sa4(mem);
    app.sa5(mem);
    app.sleep_click(200);

    for sa2 in 0..5 {
        app.write_tb4(mem, sa2, 1);
        app.sa2(mem);
        app.sleep_click(200);
    }
    app.sa4(mem);
    app.sleep_click(200);

    for sa2 in 0..5 {
        app.write_tb4(mem, sa2, 3);
        app.sa2(mem);
        app.sleep_click(200);
    }

    app.open_tabl4();
    app.final_tabl4();
    app.close_otch();

    println!("Total time: {}s", st.elapsed().as_secs());
}

fn step5(app: &mut App, mem: &Meme) {
    let st = Instant::now();

    app.set_sas(mem, [1, 0, 2, 2, 0, 2, 2, 1, 1, 1]);
    app.set_volt_to_10mv();

    for sa6 in 0..2 {
        let sa6 = 1 - sa6;
        for sa8 in 0..3 {
            app.set_freq(1, mem);
            app.sleep_click(500);
            let volt3 = find_max_volt(app, mem);
            let freq3 = mem.get_freq();

            find_min_volt(app, mem);
            let freq4 = mem.get_freq();

            let volt5 = find_max_volt(app, mem);
            let freq5 = mem.get_freq();

            let max_volt = volt3.max(volt5);

            set_to_freq(app, mem, freq3);
            find_from_volt_plus(app, mem, max_volt * 0.707, true);
            let freq2 = mem.get_freq();

            find_from_volt_plus(app, mem, max_volt * 0.1, true);
            app.write_tb5(mem, sa8 * 2 + sa6, 0);

            set_to_freq(app, mem, freq2);
            app.write_tb5(mem, sa8 * 2 + sa6, 1);

            set_to_freq(app, mem, freq3);
            app.write_tb5(mem, sa8 * 2 + sa6, 2);

            set_to_freq(app, mem, freq4);
            app.write_tb5(mem, sa8 * 2 + sa6, 3);

            set_to_freq(app, mem, freq5);
            app.write_tb5(mem, sa8 * 2 + sa6, 4);

            find_from_volt_plus(app, mem, max_volt * 0.707, false);
            app.write_tb5(mem, sa8 * 2 + sa6, 5);

            find_from_volt_plus(app, mem, max_volt * 0.1, false);
            app.write_tb5(mem, sa8 * 2 + sa6, 6);

            app.sa8(mem);
        }
        app.sa6(mem);
    }

    app.open_tabl5();
    app.final_tabl5();
    app.close_otch();

    println!("Total time: {}s", st.elapsed().as_secs());
}

fn step6(app: &mut App, mem: &Meme) {
    let st = Instant::now();

    app.set_sas(mem, [0, 0, 3, 0, 0, 0, 0, 1, 1, 0]);
    app.set_freq(0, mem);
    app.set_volt_to_10mv();

    {
        let volt3 = find_max_volt(app, mem);
        let freq3 = mem.get_freq();

        find_min_volt(app, mem);
        let freq4 = mem.get_freq();

        let volt5 = find_max_volt(app, mem);
        let freq5 = mem.get_freq();

        find_min_volt(app, mem);
        let freq6 = mem.get_freq();

        let volt7 = find_max_volt(app, mem);
        let freq7 = mem.get_freq();

        let max_volt = volt3.max(volt5.max(volt7));

        set_to_freq(app, mem, freq3);
        find_from_volt_plus(app, mem, max_volt * 0.707, true);
        let freq2 = mem.get_freq();

        find_from_volt_plus(app, mem, max_volt * 0.1, true);
        app.write_tb6(mem, 0);

        set_to_freq(app, mem, freq2);
        app.write_tb6(mem, 1);

        set_to_freq(app, mem, freq3);
        app.write_tb6(mem, 2);

        set_to_freq(app, mem, freq4);
        app.write_tb6(mem, 3);

        set_to_freq(app, mem, freq5);
        app.write_tb6(mem, 4);

        set_to_freq(app, mem, freq6);
        app.write_tb6(mem, 5);

        set_to_freq(app, mem, freq7);
        app.write_tb6(mem, 6);

        find_from_volt_plus(app, mem, max_volt * 0.707, false);
        app.write_tb6(mem, 7);

        find_from_volt_plus(app, mem, max_volt * 0.1, false);
        app.write_tb6(mem, 8);
    }

    app.open_tabl6();
    app.final_tabl6();
    app.close_otch();

    println!("Total time: {}s", st.elapsed().as_secs());
}

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1") };

    #[cfg(not(debug_assertions))]
    {
        use crate::license::license;
        println!("Проверка компа");
        if !license().unwrap() {
            println!("эта прога не зафиксирована для этого компьютера");
            use std::process::exit;
            exit(3);
        }
    }

    // thread::spawn(move || {
    //     let callback = move |event: Event| match event.event_type {
    //         EventType::KeyPress(key) => {
    //             if key == Key::Escape {
    //                 exit(0);
    //             }
    //         }
    //         _ => (),
    //     };
    //
    //     This function blocks, but now it's in its own thread
    //     if let Err(error) = listen(callback) {
    //         println!("Cannot listen to events: {:?}", error);
    //     }
    // });

    println!("Правила:");
    println!("\t*окно лабы должно быть по центру экрана (по факту не трогайте его позицию после открытия)");
    println!("\t*окно лабы не должно быть заграждено ЛЮБЫМ другим окном");
    println!("\t*желательно не трогай мышку во время запуска моей проги, пока она сама не скажет вам нажать нажать на Enter");
    println!("\"предложения и багрепорты приветствуются\"");
    println!("нажмите Enter пж;)");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();

    let st = Instant::now();

    let mem = Meme::new();

    // loop {
    //     mem.set_freq(465.);
    // }

    let mut app = App::new();

    app.setup_maket();

    let mess = "Если выскочило окно после финализации, закрой её с таблицами и нажми Enter в консоли чтоб продолжить: ";

    step1(&mut app, &mem);
    step2(&mut app, &mem);
    step3(&mut app, &mem);
    step4(&mut app, &mem);
    println!("{mess}");
    press_enter_for_exit();

    step5(&mut app, &mem);
    println!("{mess}");
    press_enter_for_exit();

    step6(&mut app, &mem);

    println!("Total time: {:.3}m", st.elapsed().as_secs_f32() / 60.);
}
