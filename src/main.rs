
use rand::Rng;
use serde::Serialize;
use std::{fs::File, io::Write};

#[derive(Serialize)]
struct Detection {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Serialize)]
struct Frame {
    frame_id: u32,
    timestamp: String,
    detections: Vec<Detection>,
}

fn main() -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut frames = Vec::new();
    let total_frames = 1000;
    let num_objects = 50;

    // Initial positions
    let mut positions: Vec<(f32, f32)> = (0..num_objects)
        .map(|_| (rng.gen_range(0.1..0.9), rng.gen_range(0.1..0.9)))
        .collect();

    // Disappearance map: Vec<Vec<bool>> where map[obj_id][frame_id] = true => present
    let mut present_map = vec![vec![true; total_frames]; num_objects];
    for obj_id in 0..num_objects {
        let mut f = 0;
        while f < total_frames {
            if rng.gen_bool(0.05) { // 5% chance to disappear
                let miss_len = rng.gen_range(1..=2);
                for i in 0..miss_len {
                    if f + i < total_frames {
                        present_map[obj_id][f + i] = false;
                    }
                }
                f += miss_len;
            } else {
                f += 1;
            }
        }
    }

    for frame_id in 0..total_frames {
        let timestamp = format!("2025-03-24T18:{:02}:{:02}.000000", frame_id / 60, frame_id % 60);
        let mut detections = Vec::new();

        for (obj_id, pos) in positions.iter_mut().enumerate() {
            if present_map[obj_id][frame_id] {
                let jitter_x = rng.gen_range(-0.01..0.01);
                let jitter_y = rng.gen_range(-0.01..0.01);
                pos.0 = (pos.0 + jitter_x).clamp(0.0, 1.0);
                pos.1 = (pos.1 + jitter_y).clamp(0.0, 1.0);

                detections.push(Detection {
                    x: pos.0,
                    y: pos.1,
                    width: 0.05,
                    height: 0.05,
                });
            }
        }


        frames.push(Frame {
            frame_id: frame_id as u32,
            timestamp,
            detections,
        });
    }

    let mut file = File::create("input_data.json")?;
    let json = serde_json::to_string_pretty(&frames)?;
    file.write_all(json.as_bytes())?;
    println!("✅ Generated input_data.json with {} frames.", total_frames);
    Ok(())
}

