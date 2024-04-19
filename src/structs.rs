use macroquad::color::Color;
pub struct Velocity {
    x: f32,
    y: f32,
}

pub struct VisionSensor {
    x: f32,
    y: f32,
    angle: f32,
    range: f32,
}

impl VisionSensor {
    fn sees_hider(&self, hider: &Hider) -> bool {
        let distance = ((self.x - hider.x).powi(2) + (self.y - hider.y).powi(2)).sqrt();
        if distance < self.range + 9.0 {
            let angle = (hider.y - self.y).atan2(hider.x - self.x);
            let angle_diff = angle - self.angle;
            if angle_diff.abs() < degree_to_radian(30.0) {
                return true;
            }
        }
        false
    }

    fn blocked_by_obs(&self, obs: &Obstacle) -> (f32, f32) {
        // this function return how much sensor can see, after being blocked by obstacle
        let x1 = self.x;
        let y1 = self.y;
        let x2 = self.x + self.range * self.angle.cos();
        let y2 = self.y + self.range * self.angle.sin();

        let x3 = obs.x;
        let y3 = obs.y;
        let x4 = obs.x + obs.length;
        let y4 = obs.y;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denominator == 0.0 {
            return (0.0, 0.0);
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

        if t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0 {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            return (x, y);
        }
        (0.0, 0.0)
    }
}
pub struct Seeker {
    x: f32,
    y: f32,
    color: Color,
    num_vision_sensors: u32,
    vision_sensors: Vec<VisionSensor>,
    velocity: Velocity,
}
pub struct Hider {
    x: f32,
    y: f32,
    color: Color,
    caught: bool,
    velocity: Velocity,
}

pub struct Obstacle {
    x: f32,
    y: f32,
    length: f32,
    color: Color,
}

fn degree_to_radian(degree: f32) -> f32 {
    degree * std::f32::consts::PI / 180.0
}
