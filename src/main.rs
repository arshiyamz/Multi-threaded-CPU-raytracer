use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const RENDER_WIDTH: i32 = 1280;
const RENDER_HEIGHT: i32 = 720;

const PROJECT_PATH: &str = env!("CARGO_MANIFEST_DIR");
const IMAGE_OUT_PATH: &str = "out/output.ppm";

mod math;
mod utils;

use math::vect::Vect;
use utils::color::Color;

fn main() -> std::io::Result<()>
{
    let mut out_path = PathBuf::from(PROJECT_PATH);
    out_path.push(IMAGE_OUT_PATH);
    let mut file = File::create(out_path)?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", RENDER_WIDTH, RENDER_HEIGHT).as_bytes())?;
    file.write_all(b"255\n")?;

    for height_iterator in 0..RENDER_HEIGHT
    {
        for width_iterator in 0..RENDER_WIDTH
        {
            let c = Color::new
            (
                (255f32 * (width_iterator as f32 / (RENDER_WIDTH - 1) as f32)) as u8,
                (255f32 * (height_iterator as f32 / (RENDER_HEIGHT - 1) as f32)) as u8,
                64u8
            );

            file.write_all(format!("{}", c).as_bytes())?;
        }
    }

    Ok(())
}
