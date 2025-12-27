use crate::app::{App, KIA};
use crate::memory_viewer::press_enter_for_exit;
use crate::step_helper::find_max_volt_from_fv1;
use rand::prelude::SliceRandom;
use std::f32::consts::SQRT_2;
use std::time::Instant;

mod app;
mod license;
mod memory_viewer;
mod open_windows;
mod step_helper;

fn step1(app: &mut App) {
    let st = Instant::now();
    app.set_sas([2, 2, 0, 2, 0, 2, 0, 1, 2]);
    // app.set_fv1(1);
    app.set_m_to(30);
    app.set_f_to(1000);

    app.set_vg_to(0.1);
    find_max_volt_from_fv1(app);

    for col in 0..2 {
        for (row, i) in [0.5, 1., 2., 3.].into_iter().enumerate() {
            let row = row as i32;
            app.find_volt_by_vg1(i);
            app.sa9();
            app.sleep(500);
            app.sa9();
            app.write_table1(col, row);
        }
        app.sa6()
    }

    app.open_table(1);
    app.final_table();
    app.close_tabl();

    println!("step1 time: {:?}", st.elapsed());
}
fn step2(app: &mut App) {
    let st = Instant::now();
    app.set_sas([2, 2, 0, 2, 0, 2, 0, 1, 2]);
    app.set_m_to(30);
    app.set_f_to(1000);
    app.set_vg_to(0.1);
    app.find_volt_by_vg1(2.12);

    app.sa9();

    app.open_table(2);

    let u_outf = app.mem.vm() * SQRT_2;
    app.write_tabl2_call(0, u_outf * 1000.);

    let kd = u_outf / 0.3 / 3.;
    app.write_tabl2_call(1, kd);

    app.final_table();
    app.close_tabl();

    println!("step2 time: {:?}", st.elapsed());
}
fn step3(app: &mut App) {
    let st = Instant::now();
    app.set_sas([2, 2, 0, 2, 0, 2, 0, 1, 2]);
    app.set_m_to(40);

    let arr = [50, 100, 200, 500, 1000, 2000, 5000, 10_000];

    for sa4 in 0..2{
        let sa4 = 1-sa4;
        app.set_sas([0, 0, 0, (sa4+1) as i16, 0, 0, 0, 0, 2]);

        // app.set_fv1(1);
        find_max_volt_from_fv1(app);
        app.find_volt_by_vg1(2.);
        app.sa9();

        for (n, &i) in arr.iter().enumerate() {
            app.set_f_to(i);
            app.ssa9();

            app.write_table3(sa4, n as i32);
        }
    }

    app.open_table(3);
    app.final_table();
    app.close_tabl();

    println!("step3 time: {:?}", st.elapsed());
}
fn step4(app: &mut App) {
    let st = Instant::now();
    app.set_sas([2, 2, 0, 2, 0, 1, 0, 1, 2]);
    app.set_m_to(10);
    app.set_f_to(1000);
    app.set_vg_to(0.08);
    // app.set_fv1(1);
    find_max_volt_from_fv1(app);
    app.set_vg_to(0.8);

    app.sa9();

    // const C: f32 = 6800.0 * 0.000_000_000_001;
    // const R: f32 = 100_000.0;
    // const F: f32 = 1000.;
    // const  TMP: f32 = 1. + (2. * PI * F * R * C) * (2. * PI * F * R * C);
    // let mkr1 = 1. / TMP.sqrt();

    const MRK1: f32 = 0.22789;
    app.ssa9();
    app.open_table(4);
    app.write_tabl4_call(1, 0, MRK1 * 100.);
    app.close_tabl();

    app.set_kia_to(KIA::INI);

    app.open_table(4);
    let kg = app.mem.kg() * 100.;
    app.write_tabl4_call(0, 0, kg);
    app.close_tabl();

    app.set_m_to(45);

    let kg = app.mem.kg() * 100.;
    app.ssa9();
    app.open_table(4);
    app.write_tabl4_call(0, 1, kg);
    app.final_table();
    app.close_tabl();

    println!("step4 time: {:?}", st.elapsed());
}
fn step5(app: &mut App) {
    let st = Instant::now();
    app.set_kia_to(KIA::DIGIT);
    app.set_sas([2, 2, 0, 1, 0, 1, 0, 1, 2]);
    app.set_m_to(20);
    app.set_f_to(1000);
    // app.set_fv1(1);
    app.set_vg_to(0.01);


    for sa6 in 0..4 {
        app.set_kia_to(KIA::DIGIT);
        app.set_sas([0, 0, 0, 0, 0, sa6 as i16 + 1, 0, 1, 2]);
        if sa6 == 3 {
            app.sa8();
            app.set_vg_to(0.1);
        }
        find_max_volt_from_fv1(app);
        app.find_volt_by_vg1(3.);
        find_max_volt_from_fv1(app);
        app.find_volt_by_vg1(3.);

        app.set_sas([0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let kg = app.mem.kg();
        app.set_kia_to(KIA::INI);
        app.fv1_less();
        app.fv1_more();
        app.waiter_1sec_while(|| kg != app.mem.kg());

        let kg = app.mem.kg() * 100.;
        app.write_table5(0, sa6, kg);
    }


    // let f = app.mem.fv();
    // app.set_fv1_to(f-100_000.);
    app.set_kia_to(KIA::DIGIT);
    find_max_volt_from_fv1(app);
    app.set_m_to(90);

    let arr = [50., 90.9, 99., 99.7];
    for (row, mkr2) in arr.into_iter().enumerate() {
        let row = row as i32;
        app.set_kia_to(KIA::DIGIT);
        app.set_sas([0, 0, 0, 0, 0, row as i16 + 1, 0, 1, 2]);
        if row == 3 {
            app.sa8();
            app.set_vg_to(0.1);
            find_max_volt_from_fv1(app);
        }
        app.find_volt_by_vg1(3.);

        app.set_sas([0, 0, 0, 0, 0, 0, 0, 0, 1]);
        let kg = app.mem.kg();
        app.set_kia_to(KIA::INI);
        app.fv1_less();
        app.fv1_more();
        app.waiter_1sec_while(|| kg != app.mem.kg());

        let kg = app.mem.kg() * 100.;
        app.write_table5(1, row, kg);

        app.write_table5(2, row, mkr2);
    }

    app.open_table(5);
    app.final_table();
    app.close_tabl();

    println!("step5 time: {:?}", st.elapsed());
}
fn step6(app: &mut App) {
    let st = Instant::now();
    app.set_sas([3, 0, 2, 0, 1, 0, 0, 0, 1]);
    app.set_m_to(30);
    app.set_f_to(1000);
    app.set_vg_to(0.01);
    app.set_kia_to(KIA::DIGIT);
    // app.set_fv1(1);
    app.sleep(100);
    find_max_volt_from_fv1(app);
    let f_max = app.mem.fv() - 50_000.;
    app.r8_to_04v();

    for sa5 in 0..2 {
        app.set_sas([0, 0, 0, 0, sa5 as i16 + 1, 0, 0, 0, 0]);
        let mut u = 1. / 1000.;

        for n in 0..5 {
            u *= 2.;
            app.set_vg_to(u);

            let vm = app.mem.vm();
            app.set_sas([0, 0, 0, 0, 0, 0, 0, 0, 2]);
            while vm == app.mem.vm() {
                app.sleep(1);
            }

            app.set_fv1_to(f_max);
            find_max_volt_from_fv1(app);
            app.fv1_more();
            find_max_volt_from_fv1(app);


            let u_vh = app.mem.vm() * 1000.;
            app.ssa9();
            app.write_table6(sa5, n, u_vh);

            let v = app.mem.vm();
            app.set_sas([0, 0, 0, 0, 0, 0, 0, 0, 1]);
            app.sleep(100);
            while v == app.mem.vm() {}

            let u_vyhf = app.mem.vm() * 1000.;
            app.ssa9();
            app.write_table6(sa5 + 2, n, u_vyhf);

            let kd = u_vyhf / (u_vh * 0.3 * 50.);
            app.write_table6(sa5 + 4, n, kd);
        }
    }

    app.open_table(6);
    app.final_table();
    app.close_tabl();

    println!("step6 time: {:?}", st.elapsed());
}
fn step7(app: &mut App) {
    let st = Instant::now();
    app.set_sas([3, 0, 2, 0, 1, 0, 0, 0, 1]);
    app.set_f_to(1000);
    app.set_kia_to(KIA::DIGIT);
    app.set_vg_to(0.008);
    app.set_fv1(1);
    while app.mem.fv()==300_000. {
        find_max_volt_from_fv1(app);
    }
    app.r8_to_04v();
    app.set_m_to(30);

    for (n, i) in [40, 60].into_iter().enumerate() {
        let kg = app.mem.kg();
        app.set_kia_to(KIA::INI);
        app.set_m_to(i);
        while kg == app.mem.kg() {}

        app.ssa9();
        app.sleep(100);

        let kg = app.mem.kg() * 100.;
        app.write_table7(n as i32, kg, i as f32 / 4.);
    }

    app.open_table(7);
    app.final_table();
    app.close_tabl();

    println!("step7 time: {:?}", st.elapsed());
}
fn step8(app: &mut App) {
    let st = Instant::now();
    app.set_sas([3, 0, 2, 0, 1, 0, 0, 0, 2]);
    app.set_f_to(1000);
    app.set_kia_to(KIA::DIGIT);
    app.set_vg_to(0.008);
    app.set_m_to(40);
    // app.set_fv1(1);
    find_max_volt_from_fv1(app);
    app.r8_to_04v();


    let vm = app.mem.vm();
    app.ssa9();
    app.write_table8(0, 1, vm * 1000.);

    app.sa9();
    let kg = app.mem.kg();
    app.set_kia_to(KIA::INI);
    while kg == app.mem.kg() {}

    let kg = app.mem.kg() * 100.;
    app.ssa9();
    app.write_table8(1, 1, kg);

    app.set_sas([0, 0, 1, 0, 0, 0, 0, 0, 2]);
    app.set_kia_to(KIA::DIGIT);
    app.set_vg_to(0.0001);
    app.find_volt_by_vg1((vm*10000.).round()/10000.);

    let vm = app.mem.vm() * 1000.;
    app.ssa9();
    app.write_table8(0, 0, vm);

    app.sa9();
    let kg = app.mem.kg();
    app.set_kia_to(KIA::INI);
    while kg == app.mem.kg() {}

    let kg = app.mem.kg() * 100.;
    app.ssa9();
    app.write_table8(1, 0, kg);

    app.open_table(8);
    app.final_table();
    app.close_tabl();

    println!("step8 time: {:?}", st.elapsed());
}
fn step9(app: &mut App) {
    let st = Instant::now();
    app.set_sas([1, 0, 0, 0, 0, 0, 0, 0, 1]);
    app.set_f_to(1000);
    app.set_m_to(40);
    app.set_kia_to(KIA::DIGIT);
    app.set_vg_to(0.005);
    app.set_fv1_to(465000.);

    let mut u = 0.005;
    for i in 0..5 {
        app.set_kia_to(KIA::DIGIT);
        app.set_vg_to(u);
        app.ssa9();

        let vm = app.mem.vm();
        app.write_table9(0, i, vm * 1000.);

        let kd = vm / 0.4 / u;
        app.write_table9(1, i, kd);

        let kg = app.mem.kg();
        app.set_kia_to(KIA::INI);
        while kg == app.mem.kg() {}

        let kg = app.mem.kg() * 100.;
        app.write_table9(2, i, kg);

        u *= 2.;
    }

    app.open_table(9);
    app.final_table();
    app.close_tabl();

    println!("step9 time: {:?}", st.elapsed());
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
            use std::process::exit;
            println!("Проверка компа");
            if !license().unwrap() {
                println!("эта прога не зафиксирована для этого компьютера");
                exit(3);
            }

            use std::{thread, time::Duration};
            use crossterm::event::{self, Event, KeyCode};
            thread::spawn(move || {
                loop {
                    if event::poll(Duration::from_millis(100)).unwrap() {
                        if let Event::Key(key_event) = event::read().unwrap() {
                            if key_event.code == KeyCode::Esc {
                                exit(0);
                            }
                        }
                    }
                }
            });
        }

        println!("Правила:");
        println!("\t*окно лабы не должно быть заграждено ЛЮБЫМ другим окном");
        println!("\t*желательно не трогай мышку во время запуска моей проги");
        println!("\t*Если прога вдруг остановиться с измерителем неленейных\n\t искажений подвигайте частоту на +-1кГц");
        println!("\t*Если прога вдруг остановиться, убедитесь что в этот\n\t момент курсор не находится над кнопкой\n\t увеличения Амплитуды генератора, т.к. вполне возможно\n\t он увеличивает амплитуду генератора с периудом ~1с.\n\t В этом cлучае можно помочь ему понажимав на кнопку.");
        println!("\n\"предложения и багрепорты приветствуются\"");
        println!("Если прога застряла или зациклилась на одном месте нажмите ESC, чтоб экстренно завершить прогамму");
        let mut arr = ["лох", "гей", "пылесос", "осознал куда поступил"];
        arr.shuffle(&mut rand::rng());

        // println!("{arr:?}");
        println!("нажмите Enter если ты {})", arr[0]);
        #[cfg(not(debug_assertions))]
        press_enter_for_exit();
    }

    let st = Instant::now();

    let mut app = App::new();

    println!("Приступаю к выполнению");

    app.setup_maket();

    step1(&mut app);
    step2(&mut app);
    step3(&mut app);
    // println!("Если прога вдруг остановиться с измерителем неленейных искажений подвигайте частоту на +-1кГц");
    step4(&mut app);
    step5(&mut app);
    step6(&mut app);
    step7(&mut app);
    step8(&mut app);
    step9(&mut app);

    println!("\nTotal time: {:.3}m", st.elapsed().as_secs_f32() / 60.);
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();
}
