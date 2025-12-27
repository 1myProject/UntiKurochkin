use crate::app::App;
use crate::step_helper::{find_max_volt_from_fv1, find_volt_from_fv1_plus};
use rand::prelude::SliceRandom;
use std::time::Instant;
use crate::memory_viewer::press_enter_for_exit;

mod app;
mod license;
mod memory_viewer;
mod open_windows;
mod step_helper;

fn step1(app: &mut App) -> f32 {
    app.setup_maket();

    let st = Instant::now();
    app.set_sas([1, 0, 1, 2]);
    app.set_fv1(1);
    app.r5_to(0.0004);
    app.set_volt1_to(0.003);

    find_max_volt_from_fv1(app);

    app.write_table1();
    app.final_table();
    app.close_tabl();

    println!("step1 time: {:?}", st.elapsed());
    app.mem.fv1()
}

fn step2(app: &mut App) {
    let st = Instant::now();
    let max_v = app.mem.vm();

    app.write_table2(0, 0);
    app.write_table2(1, 0);
    let f = app.mem.fv1();

    for (n, k) in [0.9, 0.8, 0.707, 0.5, 0.3, 0.1].iter().enumerate() {
        let n = n as i32;
        find_volt_from_fv1_plus(app, max_v * k, true);
        app.write_table2(0, n + 1);

        app.set_fv1_to(f);

        find_volt_from_fv1_plus(app, max_v * k, false);
        app.write_table2(1, n + 1);
    }

    app.open_table(2);
    app.final_table();
    app.close_tabl();

    println!("step2 time: {:?}", st.elapsed());
}

fn step3(app: &mut App, f_pch: f32) {
    let st = Instant::now();

    app.set_fv1_to(f_pch);
    for (n, i) in [0.125, 0.25, 0.5, 1., 2.].iter().enumerate() {
        app.r5_to(0.001 * i);
        app.write_table3(n as i32)
    }

    app.open_table(3);
    app.final_table();
    app.close_tabl();

    println!("step3 time: {:?}", st.elapsed());
}

fn step4(app: &mut App, f_pch: f32) {
    let st = Instant::now();
    app.set_sas([2, 0, 0, 0]);
    app.set_fv1_to(200_000.0);
    app.set_volt2_to(0.1);

    app.set_fv2(1);
    let f = f_pch + 200_000.0;
    let mut f2 = app.mem.fv2();
    while f2 != f {
        if f2 < f {
            app.fv2_more_more();
        } else {
            app.fv2_less()
        }
        app.sleep(10);
        f2 = app.mem.fv2();
    }

    app.r5_to(0.0004);

    app.write_table4();

    println!("step4 time: {:?}", st.elapsed());
}

fn step5(app: &mut App) {
    let st = Instant::now();

    app.set_volt2_to(0.1);
    for (n, i) in [0.125, 0.25, 0.5, 1., 2.].iter().enumerate() {
        app.r5_to(0.001 * i);
        app.write_table5(1, n as i32)
    }

    app.set_volt2_to(0.05);
    for (n, i) in [0.125, 0.25, 0.5, 1., 2.].iter().enumerate() {
        app.r5_to(0.001 * i);
        app.write_table5(0, n as i32)
    }

    app.open_table(5);
    app.final_table();
    app.close_tabl();

    println!("step5 time: {:?}", st.elapsed());
}

fn step6(app: &mut App) {
    let st = Instant::now();

    app.r5_to(0.0004);
    let mut v = 0.025;
    for n in 0..5 {
        app.set_volt2_to(v);
        v *= 2.;
        app.write_table6(n);
    }

    app.open_table(6);
    app.final_table();
    app.close_tabl();

    println!("step6 time: {:?}", st.elapsed());
}

fn step7(app: &mut App) {
    let st = Instant::now();

    app.set_volt2_to(0.1);
    app.r5_to(0.0004);

    for (n, i) in [1., 2., 4., 8., 16., 32., 64.].iter().enumerate() {
        app.set_volt1_to(0.001 * i);
        app.write_table7(n as i32);
    }

    app.open_table(7);
    app.final_table();
    app.close_tabl();

    println!("step7 time: {:?}", st.elapsed());
}

fn step8(app: &mut App, f_pch: f32) {
    let st = Instant::now();
    app.set_sas([2, 0, 0, 0]);

    let f_g = 1_100_000.0;

    let f_s = f_g - f_pch;
    let f_zk = f_g + f_pch;
    let f_pch05 = f_pch / 2.;

    let arr = [
        f_s,
        f_pch,
        f_zk,
        f_s / 2.,
        f_pch05,
        f_zk / 2.,
        2. * f_g - f_pch, // f'c1
        2. * f_g + f_pch, // f'c2
        f_g - f_pch05,    // f'c3
        f_g + f_pch05,    // f'c4
    ]
    .map(|x| (x / 100.).round() * 100.);

    app.set_fv2_to_1_1mhz();
    app.r5_to(0.0004);
    app.set_volt2_to(0.1);

    // let answers = app
    //     .mem
    //     .answer_tabl8()
    //     .map(|x| (x * 10_000_000.0).round() / 10_000_000.0);

    for (n, i) in arr.into_iter().enumerate() {
        app.set_fv1_to(i);
        app.find_20mv_volt_by_vg1();
        // app.set_volt1_to(answers[n]);

        app.write_table8(n as i32);
    }

    app.open_table(8);
    app.final_table();
    app.close_tabl();

    println!("step8 time: {:?}", st.elapsed());
}

fn step9(app: &mut App) -> [f32; 10] {
    let st = Instant::now();

    app.set_sas([3, 3, 0, 1]);
    app.set_volt1_to(0.04);
    app.r5_to(0.004);

    let arr = [180., 120., 60., 40., 30., 20., 15., 10., 5., 0.];
    let mut f0 = [0.0; 10];

    app.set_fv1(0);
    for (n, &i) in arr.iter().enumerate() {
        app.c3_c18_to(i);
        app.sleep(10);

        let v_max = find_max_volt_from_fv1(app);
        f0[n] = app.mem.fv1();

        app.write_table9_1(n as i32);

        find_volt_from_fv1_plus(app, v_max * 0.707, true);
        let f_l = app.mem.fv1();
        app.write_table9_2(n as i32, f_l, true);

        app.set_fv1_to(f0[n]);
        find_volt_from_fv1_plus(app, v_max * 0.707, false);
        let f_r = app.mem.fv1();

        app.write_table9_2(n as i32, f_r, false);

        app.write_table9_3(n as i32, (f_r-f_l).abs()/2.);

        app.set_fv1_to(f0[n]);
    }

    app.open_table(9);
    app.final_table();
    app.close_tabl();

    println!("step9 time: {:?}", st.elapsed());
    f0
}

fn step10(app: &mut App, f0: [f32; 10]) {
    let st = Instant::now();

    app.set_sas([3, 3, 0, 1]);
    app.c3_c18_to(90.);
    app.set_volt1_to(0.04);
    app.set_fv1(0);
    app.sleep(10);
    find_max_volt_from_fv1(app);

    app.set_sas([3, 2, 0, 1]);

    app.find_max_volt_from_c9(45.);
    let ang_min = app.mem.ang_9();

    app.open_table(10);
    let c6 = app.mem.farad_9() * 1e12;
    app.write_tabl10_call(3, 6, c6);
    app.close_tabl();

    app.set_sas([3, 2, 0, 2]);
    app.set_volt1_to(0.003);
    app.c3_c18_to(180.);

    let arr = [180., 120., 60., 40., 30., 20., 15., 10., 5., 0.];

    app.set_fv1(0);
    for (n, i) in arr.iter().enumerate() {
        app.c3_c18_to(*i);
        let mut f = f0[n]-10_000.;
        if f<100_000.{f=100_000.}
        app.set_fv1_to(f);
        app.sleep(10);

        find_max_volt_from_fv1(app);
        app.fv1_less();
        app.find_max_volt_from_c9(ang_min-30.);
        app.write_table10(n as i32, c6, f0[n]);
    }

    app.open_table(10);
    app.final_table();
    app.close_tabl();

    println!("step10 time: {:?}", st.elapsed());
}

fn step11(app: &mut App, f_pch: f32) {
    let st = Instant::now();

    app.set_sas([3, 1, 0, 0]);

    for (n, i) in [0., 180.].into_iter().enumerate() {
        let n = n as i32;
        app.c3_c18_to(i);
        while app.mem.fgetor() == 0.0 {}
        let f_geter = app.mem.fgetor();

        let fs = f_geter - f_pch - 10_000.;
        let f_zk = f_geter + f_pch - 10_000.;

        app.set_fv1_to((fs / 100.).round() * 100.);
        app.sleep(10);
        for _ in 0..2{
            find_max_volt_from_fv1(app);
            app.find_20mv_volt_by_vg1();
        }
        let u_s = app.mem.vg1();
        app.write_table11(n * 3, 0.);

        app.set_fv1_to(f_pch);
        app.find_20mv_volt_by_vg1();
        app.write_table11(n * 3 + 1, u_s);

        app.set_fv1_to((f_zk / 100.).round() * 100.);
        app.sleep(10);
        for _ in 0..2{
            find_max_volt_from_fv1(app);
            app.find_20mv_volt_by_vg1();
        }
        app.write_table11(n * 3 + 2, u_s);
    }

    app.open_table(11);
    app.final_table();
    app.close_tabl();

    println!("step11 time: {:?}", st.elapsed());
}

fn main() {
    {
        #[cfg(debug_assertions)]
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1")
        };

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

        println!("Правила:");
        println!("\t*окно лабы не должно быть заграждено ЛЮБЫМ другим окном");
        println!("\t*желательно не трогай мышку во время запуска моей проги");
        println!("\"предложения и багрепорты приветствуются\"");
        let mut arr = ["лох", "гей", "пылесос", "осознал куда поступил"];
        arr.shuffle(&mut rand::rng());

        // println!("{arr:?}");
        println!("!!!Предворительно включите лабу, нажав на SA3 и дождитесь ицинеализации!!!");
        println!("нажмите Enter если ты {})", arr[0]);
        #[cfg(not(debug_assertions))]
        press_enter_for_exit();

    }

    let st = Instant::now();

    let mut app = App::new();
    println!("Приступаю к выполнению");

    // let f_pch = 478900.;
    // let f0 = [
    //     164000.,
    //     184400.,
    //     214900.,
    //     229100.,
    //     237300.,
    //     246400.,
    //     251400.,
    //     256700.,
    //     262300.,
    //     268400.,
    // ];
    let f_pch = step1(&mut app);
    step2(&mut app);
    step3(&mut app, f_pch);
    step4(&mut app, f_pch);
    step5(&mut app);
    step6(&mut app);
    step7(&mut app);
    step8(&mut app, f_pch);
    let f0 = step9(&mut app);
    step10(&mut app, f0);
    step11(&mut app, f_pch);

    println!("Total time: {:.3}m", st.elapsed().as_secs_f32() / 60.);
    press_enter_for_exit();
}
