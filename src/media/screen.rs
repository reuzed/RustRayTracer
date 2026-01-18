#[derive(Clone)]
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

    pub fn u(&self, i: impl Into<f64>) -> f64 {
        // u coordinate on screen of a pixel
        i.into() / (self.image_width - 1) as f64
    }

    pub fn v(&self, j: impl Into<f64>) -> f64 {
        // v coordinate on screen of a pixel
        j.into() / (self.image_height - 1) as f64
    }

    pub fn uv_iter(&self) -> impl Iterator<Item = (f64, f64)> {
        (0..self.image_height).rev().flat_map(move |j| {
            (0..self.image_width).rev().map(move |i| {
                (self.u(i), self.v(j))    
            })
        })
    }
}