use std::env;
use std::process;

mod robot;
mod utils;

fn move_particles(particles: &Vec<robot::Robot>, turn: f32, forward: f32) -> Vec<robot::Robot> {
    //coz::scope!("move_particles");
    let mut result: Vec<robot::Robot> = Vec::new();
    for i in 0..particles.len() {
        result.push(particles[i].movee(turn, forward));
    }
    result
}

fn measurement_prob(
    particles: &Vec<robot::Robot>,
    landmarks: &Vec<Vec<f32>>,
    measurement: &Vec<f32>,
) -> Vec<f32> {
    //coz::scope!("measurement_prob");
    let mut weights: Vec<f32> = Vec::new();
    for i in 0..particles.len() {
        weights.push(particles[i].measure_prob(landmarks, measurement));
    }

    // normalize weights
    let sum: f32 = weights.iter().sum();
    for i in 0..weights.len() {
        weights[i] /= sum;
    }

    weights
}

fn resample(particles: &Vec<robot::Robot>, weights: &Vec<f32>) -> Vec<robot::Robot> {
    //coz::scope!("resample");
    let mut result: Vec<robot::Robot> = Vec::new();
    let mut index = 0;
    let mut beta = 0.0;
    let mw = weights.iter().fold(0.0, |acc: f32, x| acc.max(*x));
    for _ in 0..particles.len() {
        beta += utils::random() * 2.0 * mw;
        while beta > weights[index] {
            beta -= weights[index];
            index = (index + 1) % particles.len();
        }
        result.push(particles[index]);
    }
    result
}

fn particle_filter(
    particles: &Vec<robot::Robot>,
    my_robot: &mut robot::Robot,
    landmarks: &Vec<Vec<f32>>,
) -> Vec<robot::Robot> {
    //coz::scope!("particle_filter");
    // move robot and sense environment
    let my_new_robot = my_robot.movee(0.1, 5.0);
    *my_robot = my_new_robot;
    let z = my_new_robot.sense(landmarks);

    //print!("{} {} ", my_robot.x, my_robot.y);

    // move particles
    let mut particles = move_particles(&particles, 0.1, 5.0);

    // calculate weights
    let weights = measurement_prob(&particles, &landmarks, &z);

    // resample
    particles = resample(&particles, &weights);

    particles
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run --release -- 1000 100");
        process::exit(1);
    }

    let world_size = 100.0;
    let landmarks: Vec<Vec<f32>> = vec![
        vec![20.0, 20.0],
        vec![80.0, 80.0],
        vec![20.0, 80.0],
        vec![80.0, 20.0],
    ];
    let n = args[1].parse::<usize>().unwrap();
    let iterations = args[2].parse::<usize>().unwrap();

    let mut my_robot = robot::Robot::new(world_size);
    let mut my_robot_histo: Vec<Vec<f32>> = vec![vec![my_robot.x, my_robot.y]];
    let mut particles_histo: Vec<Vec<robot::Robot>> = Vec::new();

    let mut first_particles: Vec<robot::Robot> = Vec::new();
    for _ in 0..n {
        let mut particle = robot::Robot::new(world_size);
        particle.set_noise(0.05, 0.05, 5.0);
        first_particles.push(particle);
    }

    particles_histo.push(first_particles);

    for _ in 0..iterations {
        let particles =
            particle_filter(&particles_histo.last().unwrap(), &mut my_robot, &landmarks);
        particles_histo.push(particles);
        my_robot_histo.push(vec![my_robot.x, my_robot.y]);
        coz::progress!("iter filtro");
    }

    if args.len() > 3 && args[3] == "p" {
        for i in 0..particles_histo.len() {
            println!("I{}", i);
            for j in 0..particles_histo[i].len() {
                println!(
                    "P{};{};{}",
                    j, particles_histo[i][j].x, particles_histo[i][j].y
                );
            }
            println!("R;{};{}", my_robot_histo[i][0], my_robot_histo[i][1]);
        }
    }
}
