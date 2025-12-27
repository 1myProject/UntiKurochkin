use crate::app::{App, KIA};
use crate::memory_viewer::press_enter_for_exit;
use crate::step_helper::find_max_volt_from_fv1;
use std::time::Instant;

mod app;
mod memory_viewer;
mod open_windows;
mod step_helper;

fn step1(app: &mut App) {
    let st = Instant::now();
    app.set_kia_to(KIA::DIGIT_AC);
    app.set_sas([1,1,1]);
    app.set_f_to(1000);
    app.set_fd_to(0.);
    app.set_fv1_to(4_000_000.);


    for sa2 in 0..4 {
        app.set_vg_to(1.);
        find_max_volt_from_fv1(app);
        let mut vg = 0.002;

        for col in 0..9{
            app.set_vg_to(vg);
            vg*=2.;
            app.write_table1(col, sa2);
        }

        app.set_vg_to(1.);
        app.write_table1(9, sa2);
        app.sa2()
    }

    app.open_table(1);
    app.final_table();
    app.close_tabl();

    println!("step1 time: {:?}", st.elapsed());
}
fn step2(app: &mut App) {
    let band = app.mem.band();
    let st = Instant::now();
    app.set_kia_to(KIA::DIGIT_DC);
    // app.set_sas([2,0,0]);
    let t = match app.mem.sa()[0] {
        2=>2,
        4=>4,
        _=>panic!("errr")
    };
    app.set_f_to(1000);
    app.set_fd_to(0.);
    app.set_vg_to(0.01);


    app.set_kia_to(KIA::AFC);
    app.open_table(t);
    app.write_tabl2_4_call(5, 2, band/1000.);
    app.close_tabl();
    app.set_kia_to(KIA::DIGIT_DC);

    app.write_table2_4(t, 5);

    let fv = app.mem.fv();
    let mut v = 0f32;
    for d in 1..=5 {
        let f = fv + band * (d as f32 * 0.1);
        app.set_fv1_to(f);

        if d == 1 {
            v = app.mem.vm();
        }

        app.write_table2_4(t, d+5);
    }
    for d in 1..=5 {
        let f = fv - band * (d as f32 * 0.1);
        app.set_fv1_to(f);

        if d == 1 {
            v -= app.mem.vm();
        }

        app.write_table2_4(t, 5-d)
    }

    app.set_kia_to(KIA::AFC);
    app.open_table(t);

    let s = v/ band*5e6;
    app.write_tabl2_4_call(5, 3, s);

    // press_enter_for_exit();
    app.final_table();
    app.close_tabl();
    app.set_kia_to(KIA::DIGIT_DC);

    println!("step2 time: {:?}", st.elapsed());
}
fn step3(app: &mut App) {
    let st = Instant::now();
    app.set_kia_to(KIA::DIGIT_AC);
    // app.set_sas([t as i16,0,0]);
    let t = match app.mem.sa()[0] {
        2=>2,
        4=>4,
        _=>panic!("errr")
    };
    app.set_f_to(1000);
    app.set_fd_to(0.);
    app.set_vg_to(0.01);

    let arr = [10,50,100,150,200,250,300,350,400,450,500,];

    for (n, &i) in arr.iter().enumerate() {
        app.set_kia_to(KIA::DIGIT_AC);
        app.set_fd_to((i*1000) as f32);
        app.set_kia_to(KIA::INI);

        app.write_table3_5(t+1 , n as i32);
    }

    app.open_table(t+1);
    app.final_table();
    app.close_tabl();

    println!("step3 time: {:?}", st.elapsed());
}

fn main() {
    // {
    //     #[cfg(debug_assertions)]
    //     unsafe {
    //         std::env::set_var("RUST_BACKTRACE", "1")
    //     };
    //
    //     #[cfg(not(debug_assertions))]
    //     {
    //         use crate::license::license;
    //         use std::process::exit;
    //         println!("Проверка компа");
    //         if !license().unwrap() {
    //             println!("эта прога не зафиксирована для этого компьютера");
    //             exit(3);
    //         }
    //
    //         use std::{thread, time::Duration};
    //         use crossterm::event::{self, Event, KeyCode};
    //         thread::spawn(move || {
    //             loop {
    //                 if event::poll(Duration::from_millis(100)).unwrap() {
    //                     if let Event::Key(key_event) = event::read().unwrap() {
    //                         if key_event.code == KeyCode::Esc {
    //                             exit(0);
    //                         }
    //                     }
    //                 }
    //             }
    //         });
    //     }
    //
    //     println!("Правила:");
    //     println!("\t*окно лабы не должно быть заграждено ЛЮБЫМ другим окном");
    //     println!("\t*желательно не трогай мышку во время запуска моей проги");
    //     println!("\t*Если прога вдруг остановиться с измерителем неленейных\n\t искажений подвигайте частоту на +-1кГц");
    //     println!("\t*Если прога вдруг остановиться, убедитесь что в этот\n\t момент курсор не находится над кнопкой\n\t увеличения Амплитуды генератора, т.к. вполне возможно\n\t он увеличивает амплитуду генератора с периудом ~1с.\n\t В этом cлучае можно помочь ему понажимав на кнопку.");
    //     println!("\n\"предложения и багрепорты приветствуются\"");
    //     println!("Если прога застряла или зациклилась на одном месте нажмите ESC, чтоб экстренно завершить прогамму");
    //     let mut arr = ["лох", "гей", "пылесос", "осознал куда поступил"];
    //     arr.shuffle(&mut rand::rng());
    //
    //     // println!("{arr:?}");
    //     println!("нажмите Enter если ты {})", arr[0]);
    //     #[cfg(not(debug_assertions))]
    //     press_enter_for_exit();
    // }

    let st = Instant::now();

    let mut app = App::new();

    println!("Приступаю к выполнению");

    app.form();
    app.setup_maket();
    step1(&mut app);

    app.sa2();
    for sa1 in [2, 4]{
        app.set_kia_to(KIA::AFC);
        app.set_sas([sa1, 0, 0]);
        press_enter_for_exit();

        let f = app.mem.fv();
        step2(&mut app);
        app.set_fv1_to(f);
        step3(&mut app);

        app.sa2();
        app.sa2();
    }


    println!("\nTotal time: {:.3}m", st.elapsed().as_secs_f32() / 60.);
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();
}
