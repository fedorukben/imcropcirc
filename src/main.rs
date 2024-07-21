use image::DynamicImage;
use image::GenericImageView;
use image::RgbaImage;
use std::env;
use std::fs::File;
use std::path::Path;

fn imcropcirc(img: DynamicImage) -> RgbaImage {
    let (w,h) = img.dimensions();
    let r = w.min(h) / 2;
    let x0 = w / 2;
    let y0 = h / 2;

    let mut res = RgbaImage::new(w,h);

    for y in 0..h {
        for x in 0..w {
            let dx = x as i32 - x0 as i32;
            let dy = y as i32 - y0 as i32;
            let dist = ((dx * dx + dy * dy) as f64).sqrt();

            if dist < r as f64 {
                let px = img.get_pixel(x,y);
                res.put_pixel(x,y,px);
            } else {
                res.put_pixel(x,y,image::Rgba([0,0,0,0]));
            }
        }
    }
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_image> <output_image>", args[0]);
        return;
    }

    let fin = &args[1];
    let fout = &args[2];

    let img = image::open(fin).expect("Failed to open input image");

    let res = imcropcirc(img);

    res.save_with_format(fout, image::ImageFormat::Png).expect("Failed to save output image");
}
