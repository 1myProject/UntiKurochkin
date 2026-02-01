use std::arch::is_loongarch_feature_detected;
use crate::app::{App, KIA};
use crate::memory_viewer::Meme;
use crate::step_helper::{find_max_volt_from_fv1, find_volt_from_fv1_plus};
use rand::prelude::SliceRandom;
use std::time::Instant;
#[cfg(not(debug_assertions))]
use memory_viewer::press_enter_for_exit;

mod app;
mod memory_viewer;
mod open_windows;
mod step_helper;

fn step1(app: &mut App) {
    let st = Instant::now();
    app.set_sas([1,2,2,0,2,1,0,1]);
    app.set_fv1_to(465000.);
    app.set_vg_to(0.0002);
    app.r8_to(0.0001);
    find_max_volt_from_fv1(app);

    app.write_table1(0);

    let mut i = 0.;
    for c in 1..=9 {
        i += 0.5;
        app.r8_to(i /1000.);
        find_max_volt_from_fv1(app);
        app.write_table1(c);
    }

    println!("step1 time: {:?}", st.elapsed());
}
fn step2(app: &mut App) {
    let doing = move |app: &mut App, row: i32| {
        let i1 = app.mem.i8()*1000.;
        app.write_table2(0, row, i1);

        let u = app.mem.vg()*1000.;
        app.write_table2(1, row, u);

        let f0 = app.mem.fv();
        app.write_table2(2, row, f0/1000.);
        find_volt_from_fv1_plus(app, 0.5*0.707, true);
        let f1 = app.mem.fv();
        app.set_fv1_to(f0);
        find_volt_from_fv1_plus(app, 0.5*0.707, false);
        let f2 = app.mem.fv();

        let p707 = f2-f1;
        app.write_table2(3, row, p707/1000.);

        app.set_f_to(1000);
        app.set_m_to(60);
        find_max_volt_from_fv1(app);

        app.sa8();
        app.set_kia_to(KIA::INI);
        let kg = app.mem.kg();
        app.write_table2(4, row, kg);
        app.sa8();
        app.set_kia_to(KIA::DIGIT);
    };

    let st = Instant::now();
    app.set_sas([1,2,2,0,2,1,0,1]);
    app.find_max_uk_by_r8();
    app.set_vg_to(0.001);
    app.set_fv1(1);
    find_max_volt_from_fv1(app);
    app.find_volt_by_vg1(0.5);
    find_max_volt_from_fv1(app);

    doing(app, 1);

    let vg = app.mem.vg() * 50.;
    app.set_vg_to(vg);
    app.find_05volt_by_r8_revers(false);
    find_max_volt_from_fv1(app);

    doing(app, 0);

    println!("step2 time: {:?}", st.elapsed());
}
fn step3(app: &mut App) {
    let st = Instant::now();
    app.set_sas([1,2,2,0,2,1,0,1]);
    app.find_05volt_by_r8_revers(true);
    find_max_volt_from_fv1(app);

    let i1 = app.mem.i8()*1000.;
    app.write_table3(0, i1);

    let u = app.mem.vg()*1000.;
    app.write_table3(1, u);

    let f0 = app.mem.fv();
    app.write_table3(2, f0/1000.);
    find_volt_from_fv1_plus(app, 0.5*0.707, true);
    let f1 = app.mem.fv();
    app.set_fv1_to(f0);
    find_volt_from_fv1_plus(app, 0.5*0.707, false);
    let f2 = app.mem.fv();

    let p707 = f2-f1;
    app.write_table3(3, p707/1000.);

    app.set_f_to(1000);
    app.set_m_to(60);
    find_max_volt_from_fv1(app);

    app.sa8();
    app.set_kia_to(KIA::INI);
    let kg = app.mem.kg();
    app.write_table3(4, kg);
    app.sa8();
    app.set_kia_to(KIA::DIGIT);
    println!("step3 time: {:?}", st.elapsed());
}
fn step4_1(app: &mut App) -> f32 {
    let st = Instant::now();
    app.set_sas([1, 1, 1, 1, 2, 1, 0, 1]);
    app.set_m_to(30);
    app.set_vg_to(0.001);
    find_max_volt_from_fv1(app);
    app.set_vg_to(0.000_01);

    let i_start = app.mem.i8();

    app.sa3();
    app.sa2();
    app.r8_to(i_start);
    find_max_volt_from_fv1(app);

    let f0 = app.mem.fv()/1000.;
    app.write_table4_1(3, 3, f0);
    app.write_table4_1(3, 4, i_start);

    for (n, u) in [0.01, 0.1, 1., 2., 5., 10., 20., 50., 100.].into_iter().enumerate() {
        let n = n as i32;
        let u= u / 1000.;
        app.set_kia_to(KIA::DIGIT);
        app.set_sas([0, 0, 0, 0, 0, 0, 0, 1]);
        app.set_vg_to(u);
        // find_max_volt_from_fv1(app);

        let vm = app.mem.vm()*1000.;
        app.write_table4_1(n, 0, vm);

        app.sa8();
        let vm = app.mem.vm()*1000.;
        app.write_table4_1(n, 1, vm);

        app.set_kia_to(KIA::INI);
        let kg = app.mem.kg();
        app.write_table4_1(n, 2, kg);
    }

    app.set_kia_to(KIA::DIGIT);
    app.set_sas([0, 0, 0, 0, 0, 0, 0, 2]);
    app.find_volt_by_vg1(0.12);

    let vg = app.mem.vg()*1000.;
    app.write_table4_1(3, 5, vg);

    app.set_kia_to(KIA::INI);
    let kg = app.mem.kg();
    app.write_table4_1(3, 6, kg);
    app.set_kia_to(KIA::DIGIT);

    println!("step4_1 time: {:?}", st.elapsed());
    i_start
}
fn step4_2(app: &mut App) {
    let st = Instant::now();
    app.set_sas([1, 1, 1, 1, 2, 1, 0, 2]);
    app.set_m_to(30);
    app.set_vg_to(0.001);
    find_max_volt_from_fv1(app);

    for (n, u) in [0.01, 0.1, 1., 2., 5., 10., 20., 50., 100.].into_iter().enumerate() {
        let n = n as i32;
        let u= u / 1000.;
        app.set_kia_to(KIA::DIGIT);
        app.set_sas([0, 0, 0, 0, 0, 0, 0, 1]);
        app.set_vg_to(u);
        // find_max_volt_from_fv1(app);

        let vm = app.mem.vm()*1000.;
        app.write_table4_2(n, 0, vm);

        let i = app.mem.i8()*1000.;
        app.write_table4_2(n, 3, vm);

        app.sa8();
        let vm = app.mem.vm()*1000.;
        app.write_table4_2(n, 1, vm);

        app.set_kia_to(KIA::INI);
        let kg = app.mem.kg();
        app.write_table4_2(n, 2, kg);
    }

    app.set_kia_to(KIA::DIGIT);
    app.set_sas([0, 0, 0, 0, 0, 0, 0, 2]);
    app.find_volt_by_vg1(0.12);

    let vg = app.mem.vg()*1000.;
    app.write_table4_2(3, 5, vg);

    app.set_kia_to(KIA::INI);
    let kg = app.mem.kg();
    app.write_table4_2(3, 6, kg);
    app.set_kia_to(KIA::DIGIT);

    println!("step4_2 time: {:?}", st.elapsed());
}
fn step4_3_to_5(app: &mut App) {
    let doing = |app: &mut App, table:u32| {
        let f = app.mem.fv()/1000.;
        app.write_table4_3_5(table, 4, 2, f);

        for (n, u) in [0.01, 0.1, 1., 2., 5., 10., 20., 50., 100.].into_iter().enumerate() {
            let n = n as i32;
            let u = u / 1000.;
            app.set_vg_to(u);

            let vm = app.mem.vm()*1000.;
            app.write_table4_3_5(table, n, 0, vm);

            let i = app.mem.i8()*1000.;
            app.write_table4_3_5(table, n, 1, i);
        }
    };

    let st = Instant::now();
    app.set_sas([1, 2, 1, 1, 2, 1, 0, 2]);
    app.set_f_to(1000);
    app.set_m_to(30);
    app.set_vg_to(0.001);
    find_max_volt_from_fv1(app);

    doing(app, 3);

    app.set_sas([1, 1, 1, 1, 2, 2, 0, 2]);
    app.set_vg_to(0.001);
    find_max_volt_from_fv1(app);

    doing(app, 4);

    app.set_sas([1, 2, 1, 1, 2, 2, 0, 2]);
    app.set_vg_to(0.001);
    find_max_volt_from_fv1(app);

    doing(app, 5);

    println!("step4_3_to_5 time: {:?}", st.elapsed());
}
fn step4_6(app: &mut App, i_start: f32) {
    let st = Instant::now();
    app.set_sas([2, 2, 2, 1, 2, 1, 0, 1]);
    app.set_f_to(1000);
    app.set_m_to(30);
    app.set_vg_to(0.001);
    app.r8_to(i_start);
    find_max_volt_from_fv1(app);

    app.set_vg_to(0.000_1);
    app.set_sas([1, 0,0,0,0,0,0,0]);
    let vm1 = app.mem.vm()*1000.;
    app.set_sas([2, 0,0,0,0,0,0,0]);
    let vm2 = app.mem.vm()*1000.;
    app.write_table4_6(4, 3, vm1);
    app.write_table4_6(4, 4, vm2);
    app.set_sas([0, 0, 0, 0, 0, 0, 0, 2]);

    let f = app.mem.fv()/1000.;
    app.write_table4_6(4, 2, f);

    for (n, u) in [0.01, 0.1, 1., 2., 5., 10., 20., 50., 100.].into_iter().enumerate() {
        let n = n as i32;
        let u = u / 1000.;
        app.set_vg_to(u);
        app.set_kia_to(KIA::DIGIT);

        let vm = app.mem.vm()*1000.;
        app.write_table4_6(n, 0, vm);

        app.set_kia_to(KIA::INI);
        let kg = app.mem.kg();
        app.write_table4_6(n, 1, kg);
    }

    println!("step4_6 time: {:?}", st.elapsed());
}
fn step4_7(app: &mut App) {
    let st = Instant::now();
    app.set_sas([2, 2, 1, 1, 2, 2, 0, 1]);
    app.set_f_to(1000);
    app.set_m_to(30);
    app.set_vg_to(0.001);
    find_max_volt_from_fv1(app);

    let f = app.mem.fv()/1000.;
    app.write_table4_7(4, 3, f);

    let mut min_u = 0.0;
    let mut max_u = 0.0;

    for (n, u) in [0.01, 0.1, 1., 2., 5., 10., 20., 50., 100.].into_iter().enumerate() {
        let n = n as i32;
        let u = u / 1000.;
        app.set_vg_to(u);
        app.set_kia_to(KIA::DIGIT);
        app.set_sas([0, 0, 0, 0, 0, 0, 0, 1]);

        let vm = app.mem.vm()*1000.;
        if n == 2{
            min_u = vm;
        }
        else if n == 8{
            max_u = vm;
        }
        app.write_table4_7(n, 0, vm);

        app.sa8();

        let vm = app.mem.vm()*1000.;
        app.write_table4_7(n, 1, vm);

        app.set_kia_to(KIA::INI);
        let kg = app.mem.kg();
        app.write_table4_7(n, 2, kg);
    }

    let k_reg = 20.*(min_u/max_u).log10()+40.;

    app.write_table4_7(4, 4, k_reg);

    println!("step4_7 time: {:?}", st.elapsed());
}
fn step5(app: &mut App) {
    let doing = |app: &mut App, sa4: i32| {
        for (n,u) in [1., 10., 100.].into_iter().map(|x| x/1000.).enumerate() {
            let n = n as i32;
            app.set_vg_to(u);
            app.set_kia_to(KIA::DIGIT);

            let vm = app.mem.vm()*1000.;
            app.write_table5(n, sa4*2, vm);

            app.set_kia_to(KIA::INI);
            let kg = app.mem.kg();
            app.write_table5(n, sa4*2+1, kg);
        }
    };

    let st = Instant::now();
    app.set_sas([1, 1, 1, 1, 2, 1, 0, 2]);
    app.set_f_to(400);
    app.set_m_to(30);
    app.set_vg_to(0.001);
    find_max_volt_from_fv1(app);

    doing(app, 0);

    app.sa4();
    doing(app, 1);

    println!("step5 time: {:?}", st.elapsed());
}

fn main() {
    {
        #[cfg(debug_assertions)]
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1")
        };

        #[cfg(not(debug_assertions))]
        {
            use std::process::exit;
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
        println!("\n\"приветствуются багрепорты, не приветствуются предложения\"");
        println!("Если прога застряла или зациклилась на одном месте нажмите ESC, чтоб экстренно завершить прогамму");
        println!("Для issuе: https://t.me/morinosenshi или чекните новую версию в https://github.com/1myProject/UntiKurochkin/releases/tag/aru");
        println!("\nтекущая версия программы для лабы по АРУ от 12 января\n");
        let mut arr = ["мой папа", "Илон Маск", "огурчик Рик", "Анимешник"];
        arr.shuffle(&mut rand::rng());

        // println!("{arr:?}");
        println!("нажмите Enter если ты {})", arr[0]);
        #[cfg(not(debug_assertions))]
        press_enter_for_exit();
    }

    let st = Instant::now();

    let mem = Meme::new();
    let mut app = App::new(mem);

    println!("Приступаю к выполнению");

    app.setup_maket();

    #[cfg(not(debug_assertions))]
    {
        step1(&mut app);
        step2(&mut app);
    }
    #[cfg(debug_assertions)]
    {
        step3(&mut app);
        let i_st = step4_1(&mut app);
        step4_2(&mut app);
        step4_3_to_5(&mut app);
        step4_6(&mut app, i_st);
        step4_7(&mut app);
        step5(&mut app);
    }
    println!("\nTotal time: {:.3}m", st.elapsed().as_secs_f32() / 60.);
    println!("разрабу на чай (кофе не пью): Белинвест 5578 8433 7104 1785");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();
}
