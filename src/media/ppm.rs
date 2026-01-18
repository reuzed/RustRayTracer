// A PPM image file consists of header then lines of colour values for pixels.

pub fn ppm_header(width: u32, height: u32) -> String {
    format!("P3\n{} {}\n255\n", width, height)
}
