use std::process::Command;
use std::fs;
use std::convert::Into;

pub fn decolor(input_filename: String) {

    // convert PDF to PNG
    if cfg!(target_os = "windows") {
        let _ = Command::new("imagemagick")
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

    fs::remove_dir_all("split");
    fs::create_dir("split");


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
                image::Luma(x) => pixels.push(x[0]),
            }

        }

        profile_pixels(pixels);

        // let img_buf: [u8] = pixels.into();

        // image::ImageEncoder::write_image(&pixels.into(), )
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