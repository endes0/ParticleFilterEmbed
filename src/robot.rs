use crate::utils;

pub struct Robot {
    pub x: f32,
    pub y: f32,
    orientation: f32,

    forward_noise: f32,
    turn_noise: f32,
    sense_noise: f32,

    world_size: f32,
}

impl Robot {
    pub fn new(world_size: f32) -> Self {
        Robot {
            x: utils::random() * world_size,
            y: utils::random() * world_size,
            orientation: 0.0,
            forward_noise: 0.0,
            turn_noise: 0.0,
            sense_noise: 0.0,
            world_size,
        }
    }

    pub fn set_noise(&mut self, new_f_noise: f32, new_t_noise: f32, new_s_noise: f32) {
        self.forward_noise = new_f_noise;
        self.turn_noise = new_t_noise;
        self.sense_noise = new_s_noise;
    }

    pub fn sense(&self, landmarks: &Vec<Vec<f32>>) -> Vec<f32> {
        let mut z: Vec<f32> = Vec::new();
        for i in 0..landmarks.len() {
            let dist =
                ((self.x - landmarks[i][0]).powi(2) + (self.y - landmarks[i][1]).powi(2)).sqrt();
            z.push(dist + utils::random_gaussian(0.0, self.sense_noise));
        }
        z
    }

    pub fn movee(&self, turn: f32, forward: f32) -> Robot {
        let mut result = self.clone();
        let mut orientation =
            self.orientation + turn + utils::random_gaussian(0.0, self.turn_noise);
        orientation = orientation % (2.0 * std::f32::consts::PI);

        let dist = forward + utils::random_gaussian(0.0, self.forward_noise);

        result.x = (self.x + orientation.cos() * dist) % self.world_size;
        result.y = (self.y + orientation.sin() * dist) % self.world_size;
        result.orientation = orientation;

        //print!("{} {} {}", result.x, result.y, result.orientation);

        result
    }

    pub fn measure_prob(&self, landmarks: &Vec<Vec<f32>>, measurement: &Vec<f32>) -> f32 {
        let mut prob = 1.0;
        for i in 0..landmarks.len() {
            let dist = utils::distance(self.x, self.y, landmarks[i][0], landmarks[i][1]);
            prob *= utils::gaussian(0.0, self.sense_noise, dist - measurement[i]);
        }
        prob
    }
}

impl Copy for Robot {}

impl Clone for Robot {
    fn clone(&self) -> Robot {
        *self
    }
}
