// Write result of rendering to image or video.
// Pixels in output image -> rays from camera -> march/trace intersect -> lighting -> colour
// For video output render an array of frames one by one

// Video: https://docs.rs/opencv/0.74.2/opencv/prelude/trait.VideoWriterTrait.html#method.write

use opencv::{
    core::{Mat, Size},
    prelude::*,
    videoio::VideoWriter,
};

use opencv::core::ToInputArray;

struct Mp4VideoWriter {
    image_width: i32,
    image_height: i32,
    writer: VideoWriter,
}

impl Mp4VideoWriter {
    pub fn new(image_width: i32, aspect_ratio: f64) -> Mp4VideoWriter {
        let frame_size = Size::new(640, 480);
        let fourcc = VideoWriter::fourcc('m', 'p', '4', 'v').unwrap();

        let mut writer = VideoWriter::new("output.mp4", fourcc, 30.0, frame_size, true).unwrap();

        Mp4VideoWriter {
            image_width: image_width,
            image_height: (aspect_ratio * (image_width as f64)) as i32,
            writer: writer,
        }
    }

    pub fn write_frame(&mut self, frame: &impl ToInputArray) {
        self.writer.write(frame).expect("frame should write okay")
    }
}
