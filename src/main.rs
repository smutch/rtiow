use palette::Srgb;

fn main() -> Result<(), image::ImageError> {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;

    let mut framebuffer = image::RgbImage::new(WIDTH, HEIGHT);

    for jj in 0..HEIGHT {
        for ii in 0..WIDTH {
            let color = Srgb::new(ii as f32 / (WIDTH - 1) as f32, jj as f32 / (HEIGHT -1) as f32, 0.25).into_format().into();
            framebuffer.put_pixel(ii, HEIGHT-jj-1, image::Rgb(color));
        } 
    }

    framebuffer.save("./image.png")?;

    Ok(())

}
