use core::time;

use macroquad::rand::gen_range;
use macroquad::{prelude::*};


struct Velocity {
    x: f32,
    y: f32,
}

struct Vision_Sensor {
    x: f32,
    y: f32,
    angle: f32,
    range: f32,
}

struct Seeker {
    x: f32,
    y: f32,
    color: Color,
    num_vision_sensors: u32,
    vision_sensors: Vec<Vision_Sensor>,
    velocity: Velocity,
}
struct Hider {
    x: f32,
    y: f32,
    color: Color,
    caught: bool,
    velocity: Velocity,
}

fn degree_to_radian(degree: f32) -> f32 {
    degree * std::f32::consts::PI / 180.0
}

fn move_seeker(seeker: &mut Seeker, time: f32, width: f32, height: f32) {
    let magnitude = (seeker.velocity.x.powi(2) + seeker.velocity.y.powi(2)).sqrt();
    let mut direction_x = seeker.velocity.x / magnitude;
    let mut direction_y = seeker.velocity.y / magnitude;


    if gen_range(0, 100) < 10 {
        direction_x = gen_range(-1.0, 1.0);
        direction_y = gen_range(-1.0, 1.0);
    }

    seeker.x = seeker.x + seeker.velocity.x * time * direction_x;
    seeker.y = seeker.y + seeker.velocity.y * time * direction_y;

    if seeker.x > width - 10.0 || seeker.x < 10.0 {
        seeker.velocity.x = -seeker.velocity.x;
    }
    if seeker.y > height - 10.0 || seeker.y < 10.0 {
        seeker.velocity.y = -seeker.velocity.y;
    }
}

fn move_hider(hider: &mut Hider, time: f32, width: f32, height: f32) {
    let magnitude = (hider.velocity.x.powi(2) + hider.velocity.y.powi(2)).sqrt();
    let mut direction_x = hider.velocity.x / magnitude;
    let mut direction_y = hider.velocity.y / magnitude;

    if gen_range(0, 100) < 10 {
        direction_x = gen_range(-1.0, 1.0);
        direction_y = gen_range(-1.0, 1.0);
    }

    hider.x = hider.x + hider.velocity.x * time * direction_x;
    hider.y = hider.y + hider.velocity.y * time * direction_y;

    if hider.x > width - 10.0 || hider.x < 10.0 {
        hider.velocity.x = -hider.velocity.x;
    }
    if hider.y > height - 10.0 || hider.y < 10.0 {
        hider.velocity.y = -hider.velocity.y;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let radius = 10.0;
    let speed = 8.0;
    let width = screen_width();
    let height = screen_height();

    let mut seeker = Seeker {
        x: gen_range(200.0, 600.0),
        y: gen_range(200.0, 400.0),
        color: RED,
        num_vision_sensors: 6,
        vision_sensors: Vec::new(),
        velocity: Velocity { x: gen_range(-10.0, 10.0), y: gen_range(-10.0, 10.0) },
    };

    let step_angle = 120.0 / (seeker.num_vision_sensors as f32 - 1.0) ;

    for i in 0..seeker.num_vision_sensors {
        seeker.vision_sensors.push(Vision_Sensor {
            x: seeker.x,
            y: seeker.y,
            angle: degree_to_radian(-60.0 + step_angle * i as f32),
            range: 50.0,
        });

    }
    println!("{:?}", seeker.vision_sensors[0].angle);
    println!("{:?}", seeker.vision_sensors[1].angle);
    println!("{:?}", seeker.vision_sensors[2].angle);

    let mut hider = Hider {
        x: gen_range(0.0, 800.0),
        y: gen_range(0.0, 600.0),
        color: BLUE,
        caught: false,
        velocity: Velocity { x: gen_range(-10.0, 10.0), y: gen_range(-10.0, 10.0) },
    };

    loop {
        clear_background(BLACK);
    
        let t = get_frame_time() as f32 * speed;
        move_seeker(&mut seeker, t, width, height);
        move_hider(&mut hider, t, width, height);

        for sensor in seeker.vision_sensors.iter_mut() {
            sensor.x = seeker.x;
            sensor.y = seeker.y;
        }

        draw_circle(seeker.x, seeker.y, radius, seeker.color);
        for sensor in seeker.vision_sensors.iter() {
            draw_line(
                sensor.x,
                sensor.y,
                sensor.x + sensor.range * sensor.angle.cos(),
                sensor.y + sensor.range * sensor.angle.sin(),
                2.0,
                seeker.color,
            );
        }
        draw_circle(hider.x, hider.y, radius, hider.color);
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Hide and Seek".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}
