struct PpmWriter {
    image_width: i32,
    image_height: i32,
}

impl PpmWriter {
    pub fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.image_width, self.image_height)
    }
}