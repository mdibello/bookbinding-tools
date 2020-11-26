use std::process::Command;
use std::fs;

fn decolor(input_filename: String) {

    // convert PDF to PNG
    if cfg!(target_os = "windows") {
        panic!("Cannot run on Windows, needs access to Image Magick");
    }

    fs::create_dir("split");

    let _ = Command::new("convert")
                    .arg(input_filename)
                    .arg("split/page-%04d.png")
                    .output()
                    .expect("Failed to split PDF and convert to PNG");

    let pages = fs::read_dir("split").unwrap();
    for page in pages {
        page_filename = page.unwrap().file_name().into_string().unwrap();
        let mut img = image::io::Reader::open(format!("split/{}", page_filename))?.decode()?.as_luma8()?;
        dbg!(img);
    }
}