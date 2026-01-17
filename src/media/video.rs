// Write result of rendering to image or video.
// Pixels in output image -> rays from camera -> march/trace intersect -> lighting -> colour
// For video output render an array of frames one by one

// Video: https://docs.rs/opencv/0.74.2/opencv/prelude/trait.VideoWriterTrait.html#method.write

// To produce a video, render frames to a folder then run ffmpeg

pub struct VideoWriter {
    
}
