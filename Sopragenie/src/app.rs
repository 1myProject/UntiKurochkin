use std::ptr::addr_of;
use crate::memory_viewer::{press_enter_for_exit, Meme};
use crate::open_windows::{get_pos_maket};
use enigo::Button::Left;
use enigo::Coordinate::Abs;
use enigo::{Direction, Enigo, Mouse, Settings};
use crate::values::*;

pub struct App {
    enigo: Enigo,
    pub mem: Meme,
}

const SAFE_WAITER: u64 = 50;
impl App {
    pub fn new(mem: Meme) -> App {
        let enigo = Enigo::new(&Settings::default()).unwrap();

        App {
            enigo,
            mem,
        }
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
        self.click(301 , 403); // auto scale

        // set daipazone
        let diap= self.mem.DIAP()-1;
        self.click(301+44*(diap%5) , 651+33*(diap/5))
    }

    pub fn points_of_sopr_N(&mut self, N: i32)  {
        match N {
            1 => self.click(314 , 567),
            2 => {
                self.click(391 , 567);
                self.click(356 , 596);
            },
            3 => {
                self.click(391 , 567);
                self.click(416 , 596);
            },
            4 => self.click(463 , 567),
            _ => ()
        };
    }

    pub fn waiter_1sec_while(&self, fun: impl Fn() -> bool) {
        for _ in 0..1000 {
            if fun() {
                break;
            }
            self.sleep(1)
        }
    }
    
    fn to_val(&mut self, zn: impl Move, to: f32){
               
    }
}

impl App {

    pub fn setting_diapazon(&mut self) {
        self.to_val(LC{}, self.mem.lco());
        self.to_val(CD{}, self.mem.cdo());
    }
}
