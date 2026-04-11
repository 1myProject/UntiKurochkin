use crate::memory_viewer::{Meme};
use crate::open_windows::get_pos_maket;
use crate::values::*;
use enigo::Button::Left;
use enigo::Coordinate::Abs;
use enigo::{Direction, Enigo, Mouse, Settings};

pub struct App {
    enigo: Enigo,
    pub mem: Meme,
}

#[cfg(not(debug_assertions))]
const SAFE_WAITER: u64 = 50;
impl App {
    pub fn new(mem: Meme) -> App {
        let enigo = Enigo::new(&Settings::default()).unwrap();

        App { enigo, mem }
    }

    pub fn click(&mut self, x: i32, y: i32) {
        let (xo, yo) = get_pos_maket().pos();
        self.enigo.move_mouse(xo + x, yo + y, Abs).unwrap();
        self.enigo.button(Left, Direction::Click).unwrap();
        #[cfg(not(debug_assertions))]
        self.sleep(SAFE_WAITER);
    }

    pub fn sleep(&self, mils: u64) {
        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(mils));
    }

    pub fn setup_maket(&mut self) {
        self.click(301, 403); // auto scale

        // set daipazone
        let diap = self.mem.diap() - 1;
        self.click(301 + 44 * (diap % 5), 651 + 33 * (diap / 5))
    }

    pub fn points_of_sopr_n(&mut self, n: i32) {
        match n {
            1 => self.click(314, 567),
            2 => {
                self.click(391, 567);
                self.click(356, 596);
            }
            3 => {
                self.click(391, 567);
                self.click(416, 596);
            }
            4 => self.click(463, 567),
            _ => (),
        };
    }

    fn wait_1sec_while_change<T, F>(&self, val: T, get: F)
    where 
        T: PartialEq,
        F: Fn(&Meme) -> T
    {
        for _ in 0..1000{
            if val != get(&self.mem){
                break;
            }
            self.sleep(99);
        }
    } 
    fn to_val<M: Move>(&mut self, zn: M, to: f32, pogr: f32) {
        let mut last_v = zn.val(self);
        let to_left = last_v > to;
        while (last_v - to).abs() < pogr {
            match (last_v > to, to_left) {
                (true, true) => zn.less_less(self),
                (true, false) => zn.less(self),
                (false, true) => zn.more(self),
                (false, false) => zn.more_more(self),
            }
            self.wait_1sec_while_change(last_v, M::val2);
            last_v = zn.val(self);
        }
    }
}

impl App {
    pub fn setting_diapazon(&mut self) {
        self.to_val(CD {}, self.mem.cdo(), 1E-12*0.01);
        self.to_val(LC {}, self.mem.lco(), 1E-6);
    }

    #[inline]
    pub fn set_lg(&mut self){
        self.to_val(LG {}, self.mem.lgo(), 1e-6*0.1);
    }
    #[inline]
    pub fn set_cposl(&mut self){
        self.to_val(CPOS {}, self.mem.cposo(), 1e-12);
    }
    #[inline]
    pub fn set_cpar(&mut self){
        self.to_val( CPAR{}, self.mem.cparo(), 1e-12*0.01);
    }

}
