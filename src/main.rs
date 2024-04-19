use macroquad::rand::gen_range;
use macroquad::{prelude::*};



#[macroquad::main(window_conf)]
async fn main() {
    println!("Hello, world!");
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Hide and Seek".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}
