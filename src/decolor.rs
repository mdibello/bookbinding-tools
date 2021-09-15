use std::process::Command;
use std::fs;
use std::convert::Into;

pub fn decolor(input_filename: String) {

    // fs::remove_dir_all("split");
    fs::create_dir("split");

    // convert PDF to PNG
    if cfg!(target_os = "windows") {
        let _ = Command::new("magick")
                        .arg("convert")
                        // .arg("-density")
                        // .arg("300")
                        .arg(input_filename)
                        .arg("-colorspace")
                        .arg("Gray")
                        .arg("-quality")
                        .arg("100")
                        .arg("split/orig-page-%04d.png")
                        .output()
                        .expect("Failed to split PDF and convert to PNG");
    } else {
        let _ = Command::new("convert")
                        // .arg("-density")
                        // .arg("300")
                        .arg(input_filename)
                        .arg("-colorspace")
                        .arg("Gray")
                        .arg("-quality")
                        .arg("100")
                        .arg("split/orig-page-%04d.png")
                        .output()
                        .expect("Failed to split PDF and convert to PNG");
    }

    let pages = fs::read_dir("split").unwrap();
    for page in pages {

        let page_filename = page.unwrap().file_name().into_string().unwrap();
        let img = image::io::Reader::open(format!("split/{}", page_filename))
                                        .expect("Failed to open image")
                                        .decode()
                                        .expect("Failed to decode image");

        let greyscale_img = img.into_luma8();
        // fs::write("output.txt", format!("{:?}", greyscale_img));

        let mut pixels: Vec<u8> = Vec::new();
        for p in greyscale_img.pixels() {
            match p {
                image::Luma(x) => {
                    let pixel = x[0];
                    if pixel < 100 {
                        pixels.push(x[0]);
                    } else {
                        pixels.push(255);
                    }
                },
            }

        }

        profile_pixels(pixels.clone());

        let width = greyscale_img.width();
        let height = greyscale_img.height();

        // image::ImageEncoder::write_image(&pixels.into(), width, height, image::ColorType::L8);
        image::save_buffer_with_format("split/post-page-0000.png", &pixels, width, height, image::ColorType::L8, image::ImageFormat::Png);
    }

}

fn profile_pixels(pixels: Vec<u8>) -> () {
    let mut counts: Vec<u32> = Vec::new();
    for n in 0..=255 {
        counts.push(0);
    }
    for p in pixels {
        counts[p as usize] += 1;
    }
    for n in 0..255 {
        println!("{}: {}", n, counts[n]);
    }
}