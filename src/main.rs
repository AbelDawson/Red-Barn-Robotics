use crop_tracking_system::tracker::Tracker;
use crop_tracking_system::detection;
use crop_tracking_system::visualize;

use std::{fs::File, path::PathBuf};
use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    output: PathBuf,
    #[arg(long)]
    vis_dir: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct FrameInput {
    frame_id: u32,
    timestamp: String,
    detections: Vec<detection::Detection>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();


    println!("Starting reding in frames");
    let input_file = File::open(args.input)?;
    let frames: Vec<FrameInput> = serde_json::from_reader(input_file)?;

    println!("Starting! {:?}", frames[0]);

    let mut tracker = Tracker::new(3, 0.3);
    let mut output_frames = Vec::new();

    for frame in frames {
        let result = tracker.update(frame.frame_id, &frame.timestamp, &frame.detections);

        if let Some(vis_path) = &args.vis_dir {
            std::fs::create_dir_all(vis_path)?;
            visualize::render_frame(frame.frame_id, &result.tracked_objects, vis_path)?;
        }

        output_frames.push(result);
    }

    let out_file = File::create(args.output)?;
    serde_json::to_writer_pretty(out_file, &output_frames)?;

    Ok(())
}
