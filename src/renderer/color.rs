use crate::renderer::util::clamp;
use crate::renderer::vector3d::Color;

pub fn write_color(pixel_color: &Color, samples: i32) -> String {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as u32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as u32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as u32;

    format!("{} {} {}\n", ir, ig, ib)
}