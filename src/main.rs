//use crate::touren_berechner;

use macroquad::prelude::*;
use macroquad::ui;



fn get_config() -> Conf {
    Conf { window_title: "touren tracker".to_string(), window_width: 1920, window_height: 1080, fullscreen: true, window_resizable: false, ..Default::default()}
}



#[macroquad::main(get_config)]
async fn main() {
    loop {
        clear_background(GRAY);


        if is_key_pressed(KeyCode::Escape) {
            break;
        }


        next_frame().await;
    }
}