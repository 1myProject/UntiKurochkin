use crate::app::App;
use crate::memory_viewer::Meme;
#[cfg(not(debug_assertions))]
use memory_viewer::press_enter_for_exit;
use rand::prelude::SliceRandom;
use std::time::Instant;

mod app;
mod memory_viewer;
mod open_windows;
mod values;

fn step1(app: &mut App) {
}
fn step2(app: &mut App) {
}
fn step3(app: &mut App) {
}
fn step4(app: &mut App) {
}
fn step5(app: &mut App) {
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
            use rdev::{listen, EventType, Key};
            use std::process::exit;
            use std::{thread};
            thread::spawn(move || {
                listen(move |event| {
                    if event.event_type == EventType::KeyPress(Key::Escape){
                        exit(0);
                    }
                })
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

    println!("Приступаю к выполнению");

    let st = Instant::now();

    app.setup_maket();

    #[cfg(not(debug_assertions))]
    {
    }
    #[cfg(debug_assertions)]
    {
        // step1(&mut app);
        // step2(&mut app);
        // step3(&mut app);
        // step4(&mut app);
    }
    println!("\nTotal time: {:.3}m", st.elapsed().as_secs_f32() / 60.);
    println!("разрабу на чай (кофе не пью): Белинвест 5578 8433 7104 1785");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();

}
