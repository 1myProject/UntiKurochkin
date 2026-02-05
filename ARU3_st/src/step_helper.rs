use crate::App;

pub fn find_max_volt_from_fv1(app: &mut App) -> f32 {
    let mut volt = app.mem.vm();
    loop {
        let v = app.mem.vm();
        if v > 3.4 {
            app.set_vg_to(0.1);
            app.waiter_1sec_while_vm(v);
            continue;
        }
        app.fv1_more();
        app.waiter_1sec_while_vm(v);

        let v = app.mem.vm();
        if v < volt {
            app.fv1_less();
            break;
        }
        volt = v;

        match app.mem.fv() {
            300_000. => app.set_fv1(1),
            1000_000. => app.set_fv1(2),
            _ => (),
        }
    }
    let mut volt = app.mem.vm();
    loop {
        let v = app.mem.vm();
        // if v>3.4 {
        //     app.set_vg_to(0.1);
        //     app.waiter_1sec_while_vm(v);
        //     continue;
        // }
        app.fv1_less();
        app.waiter_1sec_while_vm(v);

        let v = app.mem.vm();
        if v < volt {
            app.fv1_more();
            break;
        }
        volt = v;

        match app.mem.fv() {
            300_000. => {
                app.set_fv1(0);
                app.fv_to_max();
            }
            1000_000. => {
                app.set_fv1(1);
                app.fv_to_max();
            }
            100_000. => break,
            _ => (),
        }
    }

    app.sleep(10);
    let mut difs = [0.0; 3];
    difs[1] = app.mem.vm();

    app.fv1_more();
    app.sleep(10);
    difs[2] = app.mem.vm();

    app.fv1_less();
    app.fv1_less();
    app.sleep(10);
    difs[0] = app.mem.vm();

    let max = *difs.iter().max_by(|&&x, &y| x.total_cmp(y)).unwrap();
    let pos = difs.iter().position(|&x| x == max).unwrap();
    for _ in 0..pos {
        app.fv1_more()
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
    for _ in 0..10000 {
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

        let f = app.mem.fv() / 1000.;
        for (n, i) in [300., 1000., 3000.].into_iter().enumerate() {
            if f == i {
                let indx = if revers { n } else { n + 1 } as i32;
                app.set_fv1(indx);
            }
        }
    }
    volt
}
