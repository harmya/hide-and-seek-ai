use macroquad::rand::gen_range;
use macroquad::prelude::*;
mod structs;
use structs::*;

fn degree_to_radian(degree: f32) -> f32 {
    degree * std::f32::consts::PI / 180.0
}

fn move_seeker(seeker: &mut Seeker, obst: &Obstacle, time: f32, width: f32, height: f32, fov: f32, radius: f32) {

    if seeker.x > width - radius || seeker.x < radius {
        seeker.velocity.x = -seeker.velocity.x;
    }
    if seeker.y > height - radius || seeker.y < radius {
        seeker.velocity.y = -seeker.velocity.y;
    }

    if seeker.x > obst.x && seeker.x < obst.x + obst.length && seeker.y > obst.y - radius && seeker.y < obst.y + radius {
        seeker.velocity.x = -seeker.velocity.x;
        seeker.velocity.y = -seeker.velocity.y;
    }

    let magnitude = (seeker.velocity.x.powi(2) + seeker.velocity.y.powi(2)).sqrt();
    let mut direction_x = seeker.velocity.x / magnitude;
    let mut direction_y = seeker.velocity.y / magnitude;


     if gen_range(0, 100) < 2 {
         let angle = gen_range(0.0, 2.0 * std::f32::consts::PI);
         direction_x = angle.cos();
         direction_y = angle.sin();
     }

    seeker.velocity.x = direction_x * magnitude;
    seeker.velocity.y = direction_y * magnitude;

    seeker.x = seeker.x + seeker.velocity.x * time;
    seeker.y = seeker.y + seeker.velocity.y * time;

    let angle_of_velocity = seeker.velocity.y.atan2(seeker.velocity.x);
    let step_angle = fov / (seeker.num_vision_sensors as f32 - 1.0);

    let mut index :f32 = 0.0;
    for sensor in seeker.vision_sensors.iter_mut() {
        sensor.angle = angle_of_velocity + degree_to_radian(-fov / 2.0 + step_angle * index);
        sensor.x = seeker.x;
        sensor.y = seeker.y;
        index += 1.0;
    }
}

fn move_hider(hider: &mut Hider, time: f32, width: f32, height: f32, radius: f32) {
    let magnitude = (hider.velocity.x.powi(2) + hider.velocity.y.powi(2)).sqrt();
    let mut direction_x = hider.velocity.x / magnitude;
    let mut direction_y = hider.velocity.y / magnitude;

    if gen_range(0, 100) < 2 {
        let angle = gen_range(0.0, 2.0 * std::f32::consts::PI);
        direction_x = angle.cos();
        direction_y = angle.sin();
    }

    hider.velocity.x = direction_x * magnitude;
    hider.velocity.y = direction_y * magnitude;

    hider.x = hider.x + hider.velocity.x * time;
    hider.y = hider.y + hider.velocity.y * time;

    if hider.x > width - radius || hider.x < radius {
        hider.velocity.x = -hider.velocity.x;
    }
    if hider.y > height - radius || hider.y < radius {
        hider.velocity.y = -hider.velocity.y;
    }


}

fn draw_frame(hider: &Hider, seeker: &Seeker, obstacle: &Obstacle, radius: f32) {
    draw_circle(seeker.x, seeker.y, radius, seeker.color);
    for sensor in seeker.vision_sensors.iter() {
        let color = if sensor.sees_hider(hider) { GREEN } else { WHITE };
        if sensor.sees_hider(hider) {
            println!("Saw the hider!")
        }
        let (x, y) = sensor.blocked_by_obs(obstacle);
        draw_line(
            sensor.x,
            sensor.y,
            if x == 0.0 { sensor.x + sensor.range * sensor.angle.cos() } else { x },
            if y == 0.0 { sensor.y + sensor.range * sensor.angle.sin() } else { y },
            1.0,
            color,
        );
    }
    draw_circle(hider.x, hider.y, radius, hider.color);
    draw_line(
        obstacle.x,
        obstacle.y,
        obstacle.x + obstacle.length,
        obstacle.y,
        1.0,
        obstacle.color,
    );
}

/* ---------Main --------- */
#[macroquad::main(window_conf)]
async fn main() {
    let radius = 10.0;
    let speed = 8.0;
    let width = screen_width();
    let height = screen_height();
    let fov = 90.0;
    let mut seeker = Seeker {
        x: gen_range(radius, width),
        y: gen_range(radius, height),
        color: RED,
        num_vision_sensors: 6,
        vision_sensors: Vec::new(),
        velocity: Velocity { x: gen_range(-radius, radius), y: gen_range(-radius, radius) },
    };

    let step_angle = fov / (seeker.num_vision_sensors as f32 - 1.0);

    for i in 0..seeker.num_vision_sensors {
        seeker.vision_sensors.push(VisionSensor {
            x: seeker.x,
            y: seeker.y,
            angle: degree_to_radian(-fov / 2.0 + step_angle * i as f32),
            range: 50.0,
        });

    }

    let mut hider = Hider {
        x: gen_range(0.0, width),
        y: gen_range(0.0, height),
        color: BLUE,
        caught: false,
        velocity: Velocity { x: gen_range(-radius, radius), y: gen_range(-radius, radius) },
    };


    let obstacle = Obstacle {
        x: 100.0,
        y: 100.0,
        length: 100.0,
        color: WHITE,
    };

    loop {  
        clear_background(BLACK);
        let t = get_frame_time() as f32 * speed;
        move_seeker(&mut seeker, &obstacle, t, width, height, fov, radius);
        move_hider(&mut hider, t, width, height, radius);
        draw_frame(&hider, &seeker, &obstacle, radius);
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Hide and Seek".to_owned(),
        window_width: 400,
        window_height: 300,
        ..Default::default()
    }
}