use crate::app::App;
#[cfg(not(debug_assertions))]
use crate::memory_viewer::press_enter_for_exit;
use crate::memory_viewer::Meme;
use rand::prelude::SliceRandom;
use std::io::Write;
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
        println!("Lс: {}", app.mem.lco() * 1E6);
        println!("Cд: {}", app.mem.cdo() * 1E12);

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

    println!("и сделайте рисунки нажав на кнопку \"Сохранить рисунок\" (на верхней панели)");
    println!("нажмите Entre для следующего этапа");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();
}
fn second_posl_point(app: &App) {
    println!("=========================================");
    println!("подстройка значений для первого количества точек точного сопряжения");
    println!("количество точек должно стоять: 2");
    println!("Посл/Пар: Посл\n");

    while !(app.mem.cparo() == 0.0 && app.mem.cposo() != 100.) {
        app.sleep(100);
    }

    println!("Выставите значения");
    println!("Lг: {}", app.mem.lgo() * 1E6);
    println!("Cпосл: {}\n", app.mem.cposo() * 1E12);

    println!("и сделайте рисунки нажав на кнопку \"Сохранить рисунок\" (на верхней панели)");
    println!("нажмите Entre для следующего этапа");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();
}
fn second_par_point(app: &App) {
    println!("=========================================");
    println!("подстройка значений для первого количества точек точного сопряжения");
    println!("количество точек должно стоять: 2");
    println!("Посл/Пар: Пар\n");

    while !(app.mem.cparo() != 0.0 && app.mem.cposo() == 100.) {
        app.sleep(100);
    }
    println!("Выставите значения");
    println!("Lг: {}", app.mem.lgo() * 1E6);
    println!("Cпар: {}\n", app.mem.cparo() * 1E12);

    println!("и сделайте рисунки нажав на кнопку \"Сохранить рисунок\" (на верхней панели)");
    println!("нажмите Entre для следующего этапа");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();
}
fn thred_point(app: &App) {
    println!("=========================================");
    println!("подстройка значений для первого количества точек точного сопряжения");
    println!("количество точек должно стоять: 3");
    println!("Посл/Пар: не трогать\n");

    while !(app.mem.cparo() != 0.0 && app.mem.cposo() != 100.) {
        app.sleep(100);
    }

    println!("Выставите значения");
    println!("Lг: {}", app.mem.lgo() * 1E6);
    println!("Cпосл: {}", app.mem.cposo() * 1E12);
    println!("Cпар: {}\n", app.mem.cparo() * 1E12);

    println!("и сделайте рисунки нажав на кнопку \"Сохранить рисунок\" (на верхней панели)");
}

fn main() {
    let mem: Meme;
    {
        #[cfg(debug_assertions)]
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1")
        };

        println!("Вас приветствует Помощник-путеводитель!");
        println!(
            "Он будет вам помогать выполнять и указывать что вам делать (да, разраб обленился)\n"
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
        println!("надеюсь, что ты {})", arr[0]);
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
