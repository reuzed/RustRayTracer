// Write result of rendering to image or video.
// Pixels in output image -> rays from camera -> march/trace intersect -> lighting -> colour
// For video output render an array of frames one by one

// Video: https://docs.rs/opencv/0.74.2/opencv/prelude/trait.VideoWriterTrait.html#method.write

// To produce a video, render frames to a folder then run ffmpeg

pub struct VideoWriter {}

use std::process::Command;

// call command line ffmpeg from rust
pub fn make_video() {
    let status = Command::new("ffmpeg")
        .args([
            "-framerate",
            "30",
            "-pattern_type",
            "glob",
            "-i",
            "tmp/image_*.png",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-y", // Overwrite output without asking
            "output.mp4",
        ])
        .status()
        .expect("Failed to execute ffmpeg");

    if status.success() {
        println!("Video created successfully!");
    } else {
        eprintln!("ffmpeg failed with: {}", status);
    }
}
