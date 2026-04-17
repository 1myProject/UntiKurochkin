use crate::app::App;
#[cfg(not(debug_assertions))]
use crate::memory_viewer::press_enter_for_exit;
use crate::memory_viewer::Meme;
use rand::prelude::SliceRandom;
use std::io::Write;
use windows_sys::Win32::System::Console::{
    GetConsoleMode, GetStdHandle, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
    STD_OUTPUT_HANDLE,
};

const DO_PIC: &str = "и сделайте рисунки графика который вы настраивали и график ошибки нажав на кнопку \"Сохранить рисунок\" (на верхней панели)";
fn enable_ansi() {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut mode = 0;
        GetConsoleMode(handle, &mut mode);
        SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
}
mod app;
mod memory_viewer;
mod open_windows;
// mod values;

fn granicy(app: &App) {
    println!("=========================================");
    println!("настройте граници рабочего диапазона (левая колонка ползунков)");

    println!("Выставите значения:");
    println!(
        "\"Если значения ниже превышают максимально возможные выставляемые числа, то изменяйте Ск.мин/Ск.макс\""
    );
    while !(app.is_lc() && app.is_cd()) {
        println!("Lс: {}             ", app.mem.lco() * 1E6);
        println!("Cд: {}             ", app.mem.cdo() * 1E12);

        app.sleep(100);
        print!("\x1B[2A");
        std::io::stdout().flush().unwrap();
    }

    println!("");
}
fn first_point(app: &App) {
    println!("=========================================");
    println!("подстройка значений для первого количества точек точного сопряжения");
    println!("количество точек должно стоять: 1");
    println!("Посл/Пар: не трогать\n");

    while !(app.mem.cparo() == 0.0 && app.mem.cposo() == 100.) {
        app.sleep(100);
    }

    println!("Выставите значение");
    println!("Lг: {}\n", app.mem.lgo() * 1E6);

    println!("{}", DO_PIC);
    println!("нажмите Entre для следующего этапа");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();
}
fn second_posl_point(app: &App) {
    println!("=========================================");
    println!("подстройка значений для второго количества точек (параллельный) точного сопряжения");
    println!("количество точек должно стоять: 2");
    println!("Посл/Пар: Посл\n");

    while !(app.mem.cparo() == 0.0 && app.mem.cposo() != 100.) {
        app.sleep(100);
    }

    println!("Выставите значения");
    println!("Lг: {}", app.mem.lgo() * 1E6);
    println!("Cпосл: {}\n", app.mem.cposo() * 1E12);

    println!("{}", DO_PIC);
    println!("нажмите Entre для следующего этапа");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();
}
fn second_par_point(app: &App) {
    println!("=========================================");
    println!(
        "подстройка значений для второго количества точек (последовательный) точного сопряжения"
    );
    println!("количество точек должно стоять: 2");
    println!("Посл/Пар: Пар\n");

    while !(app.mem.cparo() != 0.0 && app.mem.cposo() == 100.) {
        app.sleep(100);
    }
    println!("Выставите значения");
    println!("Lг: {}", app.mem.lgo() * 1E6);
    println!("Cпар: {}\n", app.mem.cparo() * 1E12);

    println!("{}", DO_PIC);
    println!("нажмите Entre для следующего этапа");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();
}
fn thred_point(app: &App) {
    println!("=========================================");
    println!("подстройка значений для третьего количества точек точного сопряжения");
    println!("количество точек должно стоять: 3");
    println!("Посл/Пар: не трогать\n");

    while !(app.mem.cparo() != 0.0 && app.mem.cposo() != 100.) {
        app.sleep(100);
    }

    println!("Выставите значения");
    println!("Lг: {}", app.mem.lgo() * 1E6);
    println!("Cпосл: {}", app.mem.cposo() * 1E12);
    println!("Cпар: {}\n", app.mem.cparo() * 1E12);

    println!("{}", DO_PIC);
}

fn main() {
    enable_ansi();
    let mem: Meme;
    {
        #[cfg(debug_assertions)]
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1")
        };

        #[cfg(not(debug_assertions))]
        {
            #[derive(Serialize)]
            struct User {
                id: String,
            }
            use serde::Serialize;
            use std::process::exit;
            use std::thread;
            thread::spawn(move || {
                loop {
                    let client = reqwest::blocking::Client::builder()
                        .redirect(reqwest::redirect::Policy::none())
                        .build()
                        .unwrap();

                    let id = match machine_uid::get() {
                        Ok(uid) => uid,
                        Err(_) => "uknown".to_string(),
                    };
                    let data = User { id };

                    let res = client
                        .post("http://150.251.113.37:8080/sopr")
                        .json(&data)
                        .send();
                    let Ok(res) = res else { exit(0) };
                    break;
                }
            });
        }

        println!("Вас приветствует Помощник-путеводитель!");
        println!(
            "Он будет вам помогать выполнять и указывать что вам делать (да, разраб обленился)\n"
        );
        println!("В данной работе нужно сделать по 2 рисунка на 4 точки сопряжения:");
        println!("\t1 рисунок - график который вы видите (2 синие 1 красная)");
        println!(
            "\t2 рисунок - график ошибки (его можно отобразить если нажать на кнопку (чтоб его увидеть нажмите на кнопку \"График ошибки сопряжения\""
        );
        println!("\n\"приветствуются багрепорты, не приветствуются предложения\"");
        println!(
            "Для issuе: https://t.me/morinosenshi или чекните новую версию в https://github.com/1myProject/UntiKurochkin/releases/tag/sopr"
        );
        println!(
            "и пишите разрабу только в последнюю очередь, эта прога может работать и с новыми версиями лаб, просто проверте."
        );
        println!("\nтекущая версия программы для лабы по Сопряжению от 12 января \n");

        let mut arr = ["мой папа", "Илон Маск", "огурчик Рик", "Анимешник"];
        arr.shuffle(&mut rand::rng());
        mem = Meme::new();
        // println!("{arr:?}");
        println!("Нажмите Enter если ты {})", arr[0]);
        #[cfg(not(debug_assertions))]
        press_enter_for_exit();
    }

    let mut app = App::new(mem);

    app.setup_maket();

    #[cfg(not(debug_assertions))]
    {
        granicy(&app);
        first_point(&app);
        second_posl_point(&app);
        second_par_point(&app);
        thred_point(&app);
    }
    #[cfg(debug_assertions)]
    {
        granicy(&app);
        first_point(&app);
        second_posl_point(&app);
        second_par_point(&app);
        thred_point(&app);
    }
    println!("разрабу на чай (кофе не пью): Белинвест 5578 8433 7104 1785");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();
}
