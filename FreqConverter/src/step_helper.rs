use crate::App;

pub fn find_max_volt_from_fv1(app: &mut App) -> f32 {
    let mut volt = app.mem.vm();
    loop {
        app.fv1_more();
        let v = app.mem.vm();
        if v < volt {
            app.fv1_less();
            break;
        }
        volt = v;

        match app.mem.fv1() {
            300_000. => app.set_fv1(1),
            1000_000. => app.set_fv1(2),
            _ => (),
        }
    }
    volt
}

// pub fn find_min_volt(app: &mut App) -> f32 {
//     let mut volt = app.mem.vm();
//     for _ in 0..10000 {
//         app.click_fv1_more();
//         let v = app.mem.vm();
//         if v > volt {
//             app.click_fv1_less();
//             break;
//         }
//         volt = v;
//
//         match app.mem.fv1() {
//             300. => app.set_fv1(1),
//             1000. => app.set_fv1(2),
//             _ => (),
//         }
//     }
//     volt
// }

pub fn find_volt_from_fv1_plus(app: &mut App, find_volt: f32, revers: bool) -> f32 {
    let mut volt = 0.;
    loop {
        if revers {
            app.fv1_less();
        } else {
            app.fv1_more();
        }
        let v = app.mem.vm();
        if v < find_volt {
            if !revers {
                app.fv1_less();
            } else {
                app.fv1_more();
            }
            break;
        }
        volt = v;

        if app.mem.fv1() == 300_000. {
            if revers {
                app.set_fv1(0);
                // app.freq_to_max(); 
            } else {
                app.set_fv1(1);
            }
        }
    }
    volt
}
