// Write result of rendering to image or video.
// Pixels in output image -> rays from camera -> march/trace intersect -> lighting -> colour
// For video output render an array of frames one by one

// Video: https://docs.rs/opencv/0.74.2/opencv/prelude/trait.VideoWriterTrait.html#method.write

use opencv::{
    core::{Mat, Size},
    videoio::{VideoWriter, VideoWriter_fourcc},
    prelude::*,
};

fn main() -> opencv::Result<()> {
    let frame_size = Size::new(640, 480);
    let fourcc = VideoWriter_fourcc('m', 'p', '4', 'v')?;
    
    let mut writer = VideoWriter::new("output.mp4", fourcc, 30.0, frame_size, true)?;

    // Write frames in a loop
    let frame: Mat = Mat::new();
    writer.write(&frame)?;

    // VideoWriter releases automatically when dropped
    Ok(())
}

use image::RgbImage;

fn main() {
    let (width, height) = (800, 600);
    let mut img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = 128;
            img.put_pixel(x, y, image::Rgb([r, g, b]));
        }
    }

    img.save("output.png").unwrap();
}

// Cargo.toml:
// eframe = "0.29"
// image = { version = "0.25", default-features = false, features = ["png"] }

use eframe::egui;

fn main() -> eframe::Result<()> {
    let (width, height) = (400, 300);
    
    eframe::run_native(
        "Raytracer",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(App::new(cc, width, height)))),
    )
}

struct App {
    texture: egui::TextureHandle,
    pixels: Vec<u8>, // RGB buffer
    width: usize,
    height: usize,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, width: usize, height: usize) -> Self {
        let pixels = vec![0u8; width * height * 3];
        let image = egui::ColorImage::from_rgb([width, height], &pixels);
        let texture = cc.egui_ctx.load_texture("render", image, egui::TextureOptions::NEAREST);
        Self { texture, pixels, width, height }
    }

    fn render(&mut self) {
        // Your raytracing goes here - fill self.pixels with RGB data
        for y in 0..self.height {
            for x in 0..self.width {
                let i = (y * self.width + x) * 3;
                self.pixels[i] = (x as f32 / self.width as f32 * 255.0) as u8;
                self.pixels[i + 1] = (y as f32 / self.height as f32 * 255.0) as u8;
                self.pixels[i + 2] = 128;
            }
        }
    }

    fn update_texture(&mut self) {
        let image = egui::ColorImage::from_rgb([self.width, self.height], &self.pixels);
        self.texture.set(image, egui::TextureOptions::NEAREST);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render();
        self.update_texture();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Raytracer");
            ui.image(&self.texture);
            
            // Add controls here
            if ui.button("Save PNG").clicked() {
                image::RgbImage::from_raw(self.width as u32, self.height as u32, self.pixels.clone())
                    .unwrap()
                    .save("output.png")
                    .unwrap();
            }
        });

        ctx.request_repaint(); // Continuous rendering
    }
}