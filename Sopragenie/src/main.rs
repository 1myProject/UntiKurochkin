use crate::app::App;
use crate::memory_viewer::{Meme, press_enter_for_exit};
#[cfg(not(debug_assertions))]
use memory_viewer::press_enter_for_exit;
use rand::prelude::SliceRandom;
use std::time::Instant;

mod app;
mod memory_viewer;
mod open_windows;
mod values;

fn granicy(app: &App){
    println!("=========================================");
    println!("настройте граници рабочего диапазона (лувая колонка ползунков)");
    if app.mem.cdo() > 200.0*1E-12{
        println!("Сд слишком большое.\nУвеличивайте Ск.мин и уменьшайте Ск.макс пока значение ниже не будет меньше 200");
        while app.mem.cdo() > 200.0*1E-12 {
            print!("\r{}", app.mem.cdo()*1E12);
            app.sleep(100);
        }
        println!("");
    }
    println!("Выстовите значения:");
    println!("Lс: {}", app.mem.lco() * 1E9);
    println!("Cд: {}", app.mem.cdo() * 1E12);

    while app.mem.lc() != app.mem.lco() || app.mem.cd() != app.mem.cdo(){
        app.sleep(100);
    }
    println!("");
}
fn first_point(app: &App){
    println!("=========================================");
    println!("подстройка значений для первого количества точек точного сопряжения");
    println!("количество точек должно стоять: 1");
    println!("Посл/Пар: не трогать\n");
    println!("Выстовите значение");
    println!("Lг: {}", app.mem.lgo()*10E9);

    while app.mem.lgo() != app.mem.lg() {
        app.sleep(100);
    }

    println!("теперь сделайте рисунки нажав на кнопку \"Сохранить рисунок\" (на верхней понели)");
    println!("нажмите Entre для следующего этапа");
    press_enter_for_exit();
}
fn second_posl_point(app: &App){
    println!("=========================================");
    println!("подстройка значений для первого количества точек точного сопряжения");
    println!("количество точек должно стоять: 2");
    println!("Посл/Пар: Посл\n");

    while !(app.mem.cparo()!=0.0 && app.mem.cposo()==100.){
        app.sleep(100);
    }

    println!("Выстовите значение");
    println!("Lг: {}", app.mem.lgo()*10E9);
    println!("Cпосл: {}", app.mem.cposo()*10E12);

    while app.mem.lgo() != app.mem.lg() || app.mem.cposo() != app.mem.cpos() {
        app.sleep(100);
    }

    println!("теперь сделайте рисунки нажав на кнопку \"Сохранить рисунок\" (на верхней понели)");
    println!("нажмите Entre для следующего этапа");
    press_enter_for_exit();
}
fn second_par_point(app: &App){
    println!("=========================================");
    println!("подстройка значений для первого количества точек точного сопряжения");
    println!("количество точек должно стоять: 2");
    println!("Посл/Пар: Пар\n");
    
    while !(app.mem.cparo()==0.0 && app.mem.cposo()!=100.){
        app.sleep(100);
    }
    println!("Выстовите значение");
    println!("Lг: {}", app.mem.lgo()*10E9);
    println!("Cпар: {}", app.mem.cparo()*10E12);

    while app.mem.lgo() != app.mem.lg()  || app.mem.cparo() != app.mem.cpar() {
        app.sleep(100);
    }

    println!("теперь сделайте рисунки нажав на кнопку \"Сохранить рисунок\" (на верхней понели)");
    println!("нажмите Entre для следующего этапа");
    press_enter_for_exit();
}
fn thred_point(app: &App){
    println!("=========================================");
    println!("подстройка значений для первого количества точек точного сопряжения");
    println!("количество точек должно стоять: 3");
    println!("Посл/Пар: не трогать\n");
    
    while !(app.mem.cparo()!=0.0 && app.mem.cposo()!=100.){
        app.sleep(100);
    }

    println!("Выстовите значение");
    println!("Lг: {}", app.mem.lgo()*10E9);
    println!("Cпосл: {}", app.mem.cposo()*10E12);
    println!("Cпар: {}", app.mem.cparo()*10E12);

    while app.mem.lgo() != app.mem.lg() || app.mem.cposo() != app.mem.cpos() || app.mem.cparo() != app.mem.cpar() {
        app.sleep(100);
    }

    println!("теперь сделайте рисунки нажав на кнопку \"Сохранить рисунок\" (на верхней понели)");
    println!("нажмите Entre для следующего этапа");
    press_enter_for_exit();
}

fn main() {
    let mem: Meme;
    {
        #[cfg(debug_assertions)]
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1")
        };

        // #[cfg(not(debug_assertions))]
        // {
        //     use rdev::{listen, EventType, Key};
        //     use std::process::exit;
        //     use std::{thread};
        //     thread::spawn(move || {
        //         listen(move |event| {
        //             if event.event_type == EventType::KeyPress(Key::Escape){
        //                 exit(0);
        //             }
        //         })
        //     });
        // }

        println!("Правила:");
        println!("\t*окно лабы не должно быть заграждено ЛЮБЫМ другим окном");
        println!("\t*желательно не трогай мышку во время работы моей проги");
        println!("\n\"приветствуются багрепорты, не приветствуются предложения\"");
        println!(
            "Если прога застряла или зациклилась на одном месте нажмите ESC, чтоб экстренно завершить прогамму"
        );
        println!(
            "Для issuе: https://t.me/morinosenshi или чекните новую версию в https://github.com/1myProject/UntiKurochkin/releases/tag/sopr"
        );
        println!(
            "и пишите разрабу только в последнюю очередь, эта прога может работать и с новыми версиями лаб, просто проверте."
        );
        println!("\nтекущая версия программы для лабы по Сопряжению от 19 февраля\n");
        let mut arr = ["мой папа", "Илон Маск", "огурчик Рик", "Анимешник"];
        arr.shuffle(&mut rand::rng());
        mem = Meme::new();
        // println!("{arr:?}");
        println!("нажмите Enter если ты {})", arr[0]);
        #[cfg(not(debug_assertions))]
        press_enter_for_exit();
    }

    let mut app = App::new(mem);

    // println!("Приступаю к выполнению");

    // let st = Instant::now();

    app.setup_maket();

    #[cfg(not(debug_assertions))]
    {
    }
    #[cfg(debug_assertions)]
    {
        granicy(&app);
        first_point(&app);
        second_posl_point(&app);
        second_par_point(&app);
        thred_point(&app);

        // app.setting_diapazon();
        // app.set_lg();

        // app.points_of_sopr_n(2);
        // app.set_lg();
        // app.set_cposl();

        // app.points_of_sopr_n(3);
        // app.set_lg();
        // app.set_cpar();

        // app.points_of_sopr_n(4);
        // app.set_lg();
        // app.set_cposl();
        // app.set_cpar();
    }
    // println!("\nTotal time: {:.3}m", st.elapsed().as_secs_f32() / 60.);
    println!("разрабу на чай (кофе не пью): Белинвест 5578 8433 7104 1785");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();

}
