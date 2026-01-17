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