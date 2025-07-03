use crate::tracker::TrackedObject;
use image::{ImageBuffer, Rgba};
use std::path::Path;
use ab_glyph::{FontArc, PxScale};
use imageproc::drawing::draw_text_mut;

pub fn render_frame(
    frame_id: u32,
    objects: &[TrackedObject],
    output_dir: &Path,
) -> anyhow::Result<()> {
    let width = 800u32;
    let height = 800u32;
    let mut img = ImageBuffer::from_pixel(width, height, Rgba([30u8, 30u8, 30u8, 255u8]));

    let font_data = include_bytes!("../assets/DejaVuSans.ttf");
    let font = FontArc::try_from_slice(font_data.as_ref())?;
    let scale = PxScale::from(18.0); // Or PxScale::uniform(18.0)

    for obj in objects {
        let x1 = ((obj.x - obj.width / 2.0) * width as f32).round() as i32;
        let y1 = ((obj.y - obj.height / 2.0) * height as f32).round() as i32;
        let x2 = ((obj.x + obj.width / 2.0) * width as f32).round() as i32;
        let y2 = ((obj.y + obj.height / 2.0) * height as f32).round() as i32;

        for x in x1..x2 {
            for y in y1..y2 {
                if x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
                    img.put_pixel(x as u32, y as u32, Rgba([200u8, 0u8, 0u8, 255u8]));
                }
            }
        }

        draw_text_mut(
            &mut img,
            Rgba([255u8, 255u8, 0u8, 255u8]),
            x1.max(0),
            y1.saturating_sub(20).max(0),
            scale,
            &font,
            &format!("ID {}", obj.id),
        );
    }

    let out_path = output_dir.join(format!("frame_{:05}.png", frame_id));
    img.save(&out_path)?;
    Ok(())
}

