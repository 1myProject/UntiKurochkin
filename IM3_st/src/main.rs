use std::arch::is_loongarch_feature_detected;
use crate::app::{App, KIA};
use crate::memory_viewer::Meme;
use crate::step_helper::{find_max_volt_from_fv1};
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
    println!("step1 time: {:?}", st.elapsed());
}
fn step2(app: &mut App) {

    let st = Instant::now();
    println!("step2 time: {:?}", st.elapsed());
}
fn step3(app: &mut App) {
    let st = Instant::now();
    println!("step3 time: {:?}", st.elapsed());
}
fn step4_1(app: &mut App)  {
    let st = Instant::now();

    println!("step4_1 time: {:?}", st.elapsed());

}
fn step4_2(app: &mut App) {
    let st = Instant::now();
    println!("step4_2 time: {:?}", st.elapsed());
}
fn step4_3_to_5(app: &mut App) {
    let st = Instant::now();
    println!("step4_3_to_5 time: {:?}", st.elapsed());
}
fn step4_6(app: &mut App, i_start: f32) {
    let st = Instant::now();
    println!("step4_6 time: {:?}", st.elapsed());
}
fn step4_7(app: &mut App) {
    let st = Instant::now();
    println!("step4_7 time: {:?}", st.elapsed());
}
fn step5(app: &mut App) {
    let st = Instant::now();
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
        println!("\nтекущая версия программы для лабы по ИМ от 12 января\n");
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
    }
    #[cfg(debug_assertions)]
    {
        // step1(&mut app);
        // step2(&mut app);
        // step3(&mut app);
        // let i_st = step4_1(&mut app);
        // step4_2(&mut app);
        // step4_3_to_5(&mut app);
        // step4_6(&mut app, i_st);
        // step4_7(&mut app);
        // step5(&mut app);
    }
    println!("\nTotal time: {:.3}m", st.elapsed().as_secs_f32() / 60.);
    println!("разрабу на чай (кофе не пью): Белинвест 5578 8433 7104 1785");
    #[cfg(not(debug_assertions))]
    press_enter_for_exit();


    let mut vg = app.mem.vg();
    let mut i=179;
    'a: while i>164 {
        loop {
            app.click(i, 581);
            app.sleep(150);
            if vg == app.mem.vg(){
                break
            }
            let dif = app.mem.vg()-vg;
            if dif.abs() < 0.001 {
                break 'a
            }
            vg = app.mem.vg();
        }
        i-=1;
    }
    println!("{}",i)
}
