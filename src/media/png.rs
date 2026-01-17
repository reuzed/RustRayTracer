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

