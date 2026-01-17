pub struct Screen {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
}

impl Screen {
    pub fn new(image_width: u32, aspect_ratio: f64) -> Screen {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        Screen { aspect_ratio, image_width, image_height }
    }
}
