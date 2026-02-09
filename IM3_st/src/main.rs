use crate::app::{App, KIA};
use crate::memory_viewer::Meme;
use crate::step_helper::{find_max_volt_from_fv1, find_volt_from_fv1_plus};
#[cfg(not(debug_assertions))]
use memory_viewer::press_enter_for_exit;
use rand::prelude::SliceRandom;
use std::f64::consts::PI;
use std::time::Instant;

mod app;
mod memory_viewer;
mod open_windows;
mod step_helper;

fn step1(app: &mut App) -> [f64; 4] {
    let st = Instant::now();
    app.set_sas([1, 2, 0, 1, 0, 0, 0]);
    app.set_fv1_to(500_000.);
    app.set_vg_to(0.001);
    app.to_nepr();
    app.set_m_to(0);

    let mut dfs_07 = [0.0; 4];

    for i in 0..4 {
        find_max_volt_from_fv1(app);

        let k0 = app.mem.vm();
        app.write_table1(1, 4, k0 * 1000.);
        let fv = app.mem.fv();
        app.write_table1(2, 0, fv / 1000.);

        find_volt_from_fv1_plus(app, k0 * 0.707, true);
        let fv_07_1 = app.mem.fv();
        app.write_table1(1, 0, fv_07_1 / 1000.);

        find_volt_from_fv1_plus(app, k0 * 0.1, true);
        let fv_01_1 = app.mem.fv();
        app.write_table1(0, 0, fv_01_1 / 1000.);

        app.set_fv1_to(fv); // return to center

        find_volt_from_fv1_plus(app, k0 * 0.707, false);
        let fv_07_2 = app.mem.fv();
        app.write_table1(1, 1, fv_07_2 / 1000.);

        find_volt_from_fv1_plus(app, k0 * 0.1, false);
        let fv_01_2 = app.mem.fv();
        app.write_table1(0, 1, fv_01_2 / 1000.);

        let df_07 = fv_07_2 - fv_07_1;
        dfs_07[i] = df_07;
        app.write_table1(1, 2, df_07 / 1000.);

        let df_01 = fv_01_2 - fv_01_1;
        app.write_table1(0, 2, df_01 / 1000.);

        let k_pr = df_01 / df_07;
        app.write_table1(1, 3, k_pr / 1000.);
        app.sa_num(1);
    }
    println!("step1 time: {:?}", st.elapsed());
    dfs_07
}
fn step2(app: &mut App, dfs_07: [f64; 4]) {
    let st = Instant::now();
    app.set_sas([1, 2, 0, 1, 0, 0, 0]);
    app.to_impl();
    app.fi_to(5);
    app.q_to(2.5);

    app.set_kia_to(KIA::OSC);
    for (n, &df) in dfs_07.iter().enumerate() {
        let n = n as i32;
        find_max_volt_from_fv1(app);

        let t = 2.2 / PI * 1_000_000.0 / df;
        app.write_table2(n, t);

        app.sa_num(1);
    }

    println!("step2 time: {:?}", st.elapsed());
}
fn step3(app: &mut App) {
    let doing = |app: &mut App, row_mod: i32| {
        for sa3 in 0..3 {
            let v = [0.0; 2];
            for sa2 in 0..2 {
                app.set_sas([0, 2 - sa2 as i16, sa3 as i16 + 1, 0, 0, 0, 0]);

                let vm = app.mem.vm();
                app.write_table3(sa3, row_mod * 3 + sa2, vm * 1000.);
            }

            let r = 0.001 * 5100. / (v[0] / v[1] - 1.);
            app.write_table3(sa3, row_mod * 3 + 2, r);
        }
    };

    let st = Instant::now();
    app.set_sas([4, 2, 1, 1, 0, 0, 0]);
    app.to_nepr();
    app.set_vg_to(0.002);
    app.set_m_to(0);
    app.set_kia_to(KIA::DIGIT);
    find_max_volt_from_fv1(app);

    doing(app, 0);

    app.to_impl();
    app.fi_to(5);
    app.q_to(5.);

    doing(app, 1);

    println!("step3 time: {:?}", st.elapsed());
}
fn step4(app: &mut App) {
    let st = Instant::now();
    app.set_sas([5, 2, 1, 1, 0, 0, 0]);
    app.to_impl();
    app.fi_to(1);
    app.q_to(2.);
    app.set_vg_to(0.001);
    find_max_volt_from_fv1(app);

    app.set_kia_to(KIA::OSC);

    let tust = app.mem.tust2() * 1_000_000.0 * 2.3;
    let tdspad = app.mem.tdspad() * 1_000_000.0 * 2.3;

    app.write_table4(0, tust, tdspad);

    app.sa_num(3);
    app.write_table4(1, 500., 500.);

    app.sa_num(3);
    app.write_table4(2, tust, tdspad);

    println!("step4_1 time: {:?}", st.elapsed());
}
fn step5(app: &mut App) {
    let st = Instant::now();
    app.set_sas([0, 0, 0, 0, 3, 0, 1]);
    app.to_nepr();
    app.set_vg_to(0.001);

    for sa2 in 0..4 {
        app.set_sas([0, 0, 0, 0, 0, sa2 as i16 + 1, 0]);
        find_max_volt_from_fv1(app);

        let k0 = app.mem.vm();
        app.write_table5(1, 4, k0 * 1000.);
        let fv = app.mem.fv();
        app.write_table5(2, 0, fv / 1000.);

        find_volt_from_fv1_plus(app, k0 * 0.707, true);
        let fv_07_1 = app.mem.fv();
        app.write_table5(1, 0, fv_07_1 / 1000.);

        find_volt_from_fv1_plus(app, k0 * 0.1, true);
        let fv_01_1 = app.mem.fv();
        app.write_table5(0, 0, fv_01_1 / 1000.);

        app.set_fv1_to(fv); // return to center

        find_volt_from_fv1_plus(app, k0 * 0.707, false);
        let fv_07_2 = app.mem.fv();
        app.write_table5(1, 1, fv_07_2 / 1000.);

        find_volt_from_fv1_plus(app, k0 * 0.1, false);
        let fv_01_2 = app.mem.fv();
        app.write_table5(0, 1, fv_01_2 / 1000.);

        let df_07 = fv_07_2 - fv_07_1;
        app.write_table5(1, 2, df_07 / 1000.);

        let df_01 = fv_01_2 - fv_01_1;
        app.write_table5(0, 2, df_01 / 1000.);

        let k_pr = df_01 / df_07;
        app.write_table5(1, 3, k_pr / 1000.);
    }

    println!("step4_2 time: {:?}", st.elapsed());
}
fn step6(app: &mut App) {
    let st = Instant::now();
    println!("step4_3_to_5 time: {:?}", st.elapsed());
}
fn step7(app: &mut App) {
    let st = Instant::now();
    println!("step4_6 time: {:?}", st.elapsed());
}
fn main() {
    let mem: Meme;
    {
        #[cfg(debug_assertions)]
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1")
        };

        #[cfg(not(debug_assertions))]
        {
            use crossterm::event::{self, Event, KeyCode};
            use std::process::exit;
            use std::{thread, time::Duration};
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
        println!("\t*желательно не трогай мышку во время работы моей проги");
        println!("\n\"приветствуются багрепорты, не приветствуются предложения\"");
        println!(
            "Если прога застряла или зациклилась на одном месте нажмите ESC, чтоб экстренно завершить прогамму"
        );
        println!(
            "Для issuе: https://t.me/morinosenshi или чекните новую версию в https://github.com/1myProject/UntiKurochkin/releases/tag/im"
        );
        println!(
            "и пишите разрабу только в последнюю очередь, эта прога может работать и с новыми версиями лаб, просто проверте."
        );
        println!("\nтекущая версия программы для лабы по ИМ от 12 января\n");
        let mut arr = ["мой папа", "Илон Маск", "огурчик Рик", "Анимешник"];
        arr.shuffle(&mut rand::rng());
        mem = Meme::new();
        // println!("{arr:?}");
        println!("нажмите Enter если ты {})", arr[0]);
        #[cfg(not(debug_assertions))]
        press_enter_for_exit();
    }

    let mut app = App::new(mem);

    println!("Приступаю к выполнению");

    let st = Instant::now();

    app.setup_maket();

    #[cfg(not(debug_assertions))]
    {}
    #[cfg(debug_assertions)]
    {
        let dfs = step1(&mut app);
        step2(&mut app, dfs);
        step3(&mut app);
        step4(&mut app);
        app.set_to_maket2();
        step5(&mut app);
        step6(&mut app);
        step7(&mut app);
    }
    println!("\nTotal time: {:.3}m", st.elapsed().as_secs_f32() / 60.);
    println!("разрабу на чай (кофе не пью): Белинвест 5578 8433 7104 1785");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();

    #[cfg(debug_assertions)]
    {
        let mut vg = app.mem.vg();
        let mut i = 179;
        'a: while i > 164 {
            loop {
                app.click(i, 581);
                app.sleep(150);
                if vg == app.mem.vg() {
                    break;
                }
                let dif = app.mem.vg() - vg;
                if dif.abs() < 0.001 {
                    break 'a;
                }
                vg = app.mem.vg();
            }
            i -= 1;
        }
        println!("{}", i)
    }
}
