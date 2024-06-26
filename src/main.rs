use macroquad::rand::gen_range;
use macroquad::prelude::*;
mod network;
use ndarray::array;

/* ---------Enums------------ */
enum GameStatus {
    Paused,
    Running,
}

/* ---------Structs --------- */
struct Velocity {
    x: f32,
    y: f32,
}

struct VisionSensor {
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
    vision_sensors: Vec<VisionSensor>,
    velocity: Velocity,
}
struct Hider {
    x: f32,
    y: f32,
    color: Color,
    velocity: Velocity,
    network: network::NeuralNetwork,
    born_time: f64,
    caught: bool,
}

struct Obstacle {
    x: f32,
    y: f32,
    length: f32,
    color: Color,
}


/* ---------Implementations --------- */
impl VisionSensor {
    fn sees_hider(&self, hider: &Hider, obstable: &Obstacle) -> bool {
        let blocked = self.blocked_by_obs(obstable);
        let start_x = self.x;
        let start_y = self.y;
        let mut end_x = self.x + self.range * self.angle.cos();
        let mut end_y = self.y + self.range * self.angle.sin();
        if blocked.0 != 0.0 || blocked.1 != 0.0 {
            end_x = blocked.0;
            end_y = blocked.1;
        }
        let t = ((hider.x - start_x) * (end_x - start_x) + (hider.y - start_y) * (end_y - start_y)) / (self.range * self.range);
        let t = t.max(0.0).min(1.0);
        let closest_x = start_x + t * (end_x - start_x);
        let closest_y = start_y + t * (end_y - start_y);
        let distance = ((hider.x - closest_x).powi(2) + (hider.y - closest_y).powi(2)).sqrt();
        return distance < 10.0;
    }

    fn blocked_by_obs(&self, obs: &Obstacle) -> (f32, f32) {
        let sensor_end_x = self.x + self.range * self.angle.cos();
        let sensor_end_y = self.y + self.range * self.angle.sin();
        let p1 = (self.x, self.y);
        let p2 = (sensor_end_x, sensor_end_y);
        let p3 = (obs.x, obs.y);
        let p4 = (obs.x + obs.length, obs.y);
        let intersection = line_intersection(p1, p2, p3, p4);
        match intersection {
            Some((x, y)) => (x, y),
            None => (0.0, 0.0),
        }
    }
}

impl Seeker {
    fn new(x: f32, y: f32, color: Color, num_vision_sensors: u32, velocity: Velocity) -> Self {
        let step_angle = 90.0 / (num_vision_sensors as f32 - 1.0);
        let mut vision_sensors = Vec::new();
        for i in 0..num_vision_sensors {
            vision_sensors.push(VisionSensor {
                x,
                y,
                angle: degree_to_radian(-90.0 + step_angle * i as f32),
                range: 50.0,
            });
        }
        Self {
            x,
            y,
            color,
            num_vision_sensors,
            vision_sensors,
            velocity,
        }
    }
}

impl Hider {
    fn new(x: f32, y: f32, color: Color, velocity: Velocity, network: Option<network::NeuralNetwork>, new: bool) -> Self {

        let mut network = match network {
            Some(network) => network,
            None => network::NeuralNetwork::new(6, 4, 4),
        };

        if !new {
            network.mutate(0.3);
        }


        Self {
            x,
            y,
            color,
            velocity,
            network,
            born_time: get_time(),
            caught: false,
        }
    }
}

impl Obstacle {
    fn new(x: f32, y: f32, length: f32, color: Color) -> Self {
        Self {
            x,
            y,
            length,
            color,
        }
    }
}

/* ---------Functions --------- */

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


    if is_key_pressed(KeyCode::D) {
        direction_x = degree_to_radian(0.0).cos();
        direction_y = degree_to_radian(0.0).sin();
    } else if is_key_pressed(KeyCode::A) {
        direction_x = degree_to_radian(180.0).cos();
        direction_y = degree_to_radian(180.0).sin();
    } else if is_key_pressed(KeyCode::W) {
        direction_y = degree_to_radian(-90.0).sin();
        direction_x = degree_to_radian(-90.0).cos();
    } else if is_key_pressed(KeyCode::S) {
        direction_y = degree_to_radian(90.0).sin();
        direction_x = degree_to_radian(90.0).cos();
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

fn move_hider(hider: &mut Hider, time: f32, width: f32, height: f32, radius: f32, direction: usize, seeker: &Seeker, obstacle: &Obstacle) {
    if hider.caught {
        return;
    }

    let magnitude = (hider.velocity.x.powi(2) + hider.velocity.y.powi(2)).sqrt();
    let mut direction_x = hider.velocity.x / magnitude;
    let mut direction_y = hider.velocity.y / magnitude;


    if direction == 0 { // Move right
        direction_x = degree_to_radian(0.0).cos();
        direction_y = degree_to_radian(0.0).sin();
    } else if direction == 1 { // Move left
        direction_x = degree_to_radian(180.0).cos();
        direction_y = degree_to_radian(180.0).sin();
    } else if direction == 2 { // Move up
        direction_x = degree_to_radian(90.0).cos();
        direction_y = degree_to_radian(90.0).cos();
    } else if direction == 3 { // Move down
        direction_x = degree_to_radian(270.0).cos();
        direction_y = degree_to_radian(270.0).sin();
    }

    
    if hider.x > width - radius || hider.x < radius {
        direction_x = -direction_x;
    }
    if hider.y > height - radius || hider.y < radius {
        direction_y = -direction_y;
    }
    
    hider.velocity.x = direction_x * magnitude;
    hider.velocity.y = direction_y * magnitude;

    hider.x = hider.x + hider.velocity.x * time;
    hider.y = hider.y + hider.velocity.y * time;



}

fn line_intersection(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), p4: (f32, f32)) -> Option<(f32, f32)> {
    let s1_x = p2.0 - p1.0;
    let s1_y = p2.1 - p1.1;
    let s2_x = p4.0 - p3.0;
    let s2_y = p4.1 - p3.1;

    let s = (-s1_y * (p1.0 - p3.0) + s1_x * (p1.1 - p3.1)) / (-s2_x * s1_y + s1_x * s2_y);
    let t = ( s2_x * (p1.1 - p3.1) - s2_y * (p1.0 - p3.0)) / (-s2_x * s1_y + s1_x * s2_y);

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        // Collision detected
        let x = p1.0 + (t * s1_x);
        let y = p1.1 + (t * s1_y);
        return Some((x, y));
    }

    None
}

fn draw_frame(hiders: &Vec<Hider>, seeker: &Seeker, obstacle: &Obstacle, radius: f32) {
    draw_circle(seeker.x, seeker.y, radius, seeker.color);
    for sensor in seeker.vision_sensors.iter() {
        let (x, y) = sensor.blocked_by_obs(obstacle);
        draw_line(
            sensor.x,
            sensor.y,
            if x == 0.0 { sensor.x + sensor.range * sensor.angle.cos() } else { x },
            if y == 0.0 { sensor.y + sensor.range * sensor.angle.sin() } else { y },
            1.0,
            GREEN,
        );
    }
    for hider in hiders.iter() {
        draw_circle(hider.x, hider.y, radius, hider.color);
    }
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
    let mut game_status = GameStatus::Running;
    let radius = 10.0;
    let speed = 8.0;
    let width = screen_width();
    let height = screen_height();
    let fov = 90.0;
    let mut seeker = Seeker::new(100.0, 100.0, RED, 5, 
            Velocity { x: gen_range(-radius, radius), y: gen_range(-radius, radius) });
    let obstacle = Obstacle::new(150.0, 150.0, 100.0, YELLOW);
    let mut found = false;
    
    let mut hiders : Vec<Hider> = Vec::new();
    for _ in 0..10 {
        hiders.push(Hider::new(gen_range(50.0, width - 50.0), gen_range(50.0, height - 50.0), 
            BLUE, Velocity { x: gen_range(-radius, radius), y: gen_range(-radius, radius) }, None, true));
    }

    let mut caught_count = 0;

    loop {
        if let GameStatus::Running = game_status {
            clear_background(BLACK);
            let t = get_frame_time() as f32 * speed;
            move_seeker(&mut seeker, &obstacle, t, width, height, fov, radius);
            for hider in hiders.iter_mut() {
                let direction = hider.network.get_direction(array![[hider.x as f64, hider.y as f64, seeker.x as f64, seeker.y as f64, obstacle.x as f64, obstacle.y as f64]]);
                move_hider(hider, t, width, height, radius, direction, &seeker, &obstacle);
                found = seeker.vision_sensors.iter().any(|sensor| sensor.sees_hider(&hider, &obstacle));
                found = seeker.vision_sensors.iter().any(|sensor| sensor.sees_hider(&hider, &obstacle));
                if found {
                    if !hider.caught {
                        hider.velocity.x = 0.0;
                        hider.velocity.y = 0.0;
                        hider.caught = true;
                        caught_count += 1;
                    }
                }
            }
            draw_frame(&hiders, &seeker, &obstacle, radius);
            println!("Caught count: {}", caught_count);
            println!("Hiders count: {}", hiders.len());
            if caught_count == hiders.len() {
                //get the index of the hider that was caught with max time
                let current_time = get_time();
                let mut max_time = 0.0;
                let mut max_index = 0;
                for (index, hider) in hiders.iter().enumerate() {
                    if current_time - hider.born_time > max_time {
                        max_time = current_time - hider.born_time;
                        max_index = index;
                    }
                }
                let best_network = hiders[max_index].network.clone();
                for _ in 0..10 {
                    hiders.push(Hider::new(gen_range(50.0, width - 50.0), gen_range(50.0, height - 50.0), 
                    BLUE, Velocity { x: gen_range(-radius, radius), y: gen_range(-radius, radius) }, 
                    Some(best_network.clone()), false));
                }
                //remove all previous caught_count hiders
                hiders.drain(0..caught_count);

                
            }
        }
        next_frame().await
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