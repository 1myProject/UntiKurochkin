use crate::App;
use crate::memory_viwer::Meme;

pub fn find_max_volt(app: &mut App, mem: &Meme) -> f32 {
    let mut volt = mem.get_volts();
    for _ in 0..10000 {
        app.click_freq_more(mem);
        let v = mem.get_volts();
        if v < volt {
            app.click_freq_less(mem);
            break;
        }
        volt = v;

        match mem.get_freq() {
            300. => app.set_freq(1, mem),
            1000. => app.set_freq(2, mem),
            _ => (),
        }
    }
    volt
}

pub fn find_min_volt(app: &mut App, mem: &Meme) -> f32 {
    let mut volt = mem.get_volts();
    for _ in 0..10000 {
        app.click_freq_more(mem);
        let v = mem.get_volts();
        if v > volt {
            app.click_freq_less(mem);
            break;
        }
        volt = v;

        match mem.get_freq() {
            300. => app.set_freq(1, mem),
            1000. => app.set_freq(2, mem),
            _ => (),
        }
    }
    volt
}

pub fn find_from_volt_plus(app: &mut App, mem: &Meme, find_volt: f32, revers: bool) -> f32 {
    let mut volt = 0.;
    for _ in 0..10000 {
        if revers {
            app.click_freq_less(mem);
        } else {
            app.click_freq_more(mem);
        }
        let v = mem.get_volts();
        if v < find_volt {
            if !revers {
                app.click_freq_less(mem);
            } else {
                app.click_freq_more(mem);
            }
            break;
        }
        volt = v;

        if mem.get_freq() == 300. {
            if revers {
                app.set_freq(0, mem);
                app.freq_to_max();
            } else {
                app.set_freq(1, mem);
            }
        }
    }
    volt
}

pub fn set_to_freq(app: &mut App, mem: &Meme, find_freq: f32) {
    let f = mem.get_freq();
    match find_freq {
        100.0..300.0 => if !(100.0..300.0).contains(&f) {
            app.set_freq(0, mem);
        },
        300.0..1000.0 => if !(300.0..1000.0).contains(&f) {
            app.set_freq(1, mem);
        },
        1000.0..3000.0 => if !(1000.0..3000.0).contains(&f) {
            app.set_freq(2, mem);
        },
        _=>panic!("not supported frequency!"),
    };

    // let f = mem.get_freq();
    // if f == find_freq {
    //     return;
    // }

    // if f>find_freq {
    //     let count = ((f-find_freq)*10.) as u32;
    //     for _ in 0..count {
    //         app.click_freq_less(mem);
    //     }
    // }
    // if f<find_freq {
    //     let count = ((find_freq-f)*10.) as u32;
    //     for _ in 0..count {
    //         app.click_freq_more(mem);
    //     }
    // }

    app.sleep_click(200);

    for _ in 0..10000 {
        let f = mem.get_freq();
        if f == find_freq {
            break;
        }
        else if find_freq>f {
            app.click_freq_more(mem);
        } else {
            app.click_freq_less(mem);
        }
    }
    
}