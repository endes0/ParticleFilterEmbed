use core::f32::consts::PI;
use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::Read;

pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

pub fn random() -> f32 {
    static DEV_RANDOM: OnceCell<File> = OnceCell::new();
    if DEV_RANDOM.get().is_none() {
        DEV_RANDOM.set(File::open("/dev/urandom").unwrap()).unwrap();
    }

    let mut buf = [0u8; 4];
    DEV_RANDOM.get().unwrap().read_exact(&mut buf).unwrap();
    let mut num = u32::from_le_bytes(buf);
    num >>= 8;
    num as f32 / 0x00ff_ffff as f32
}

pub fn gaussian(mean: f32, sigma: f32, x: f32) -> f32 {
    let variance = sigma.powi(2);
    let exp = (-((mean - x).powi(2)) / (2.0 * variance)).exp();
    exp / (2.0 * PI * variance).sqrt()
}

pub fn random_gaussian(mean: f32, sigma: f32) -> f32 {
    // Lambert Meertens algorithm
    let x2pi = random() * 2.0 * PI;
    let g2rad = (-2.0 * (1.0 - random()).ln()).sqrt();
    let x = x2pi.cos() * g2rad;
    x * sigma + mean
}
