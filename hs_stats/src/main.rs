use image::DynamicImage;
use rusty_tesseract::{Args, Image};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

const MY_URL: &str = "https://web.hochschulsport-hannover.de/campusfit/auslastungsgrafik";

fn main() {
    let my_args = Args {
        lang: String::from("eng"),
        dpi: Some(150),
        psm: Some(10),
        oem: Some(1),
        config_variables: HashMap::from([("tessedit_char_whitelist".into(), "1234567890".into())]),
    };

    match fetch_image() {
        Ok(img) => {
            let mut edited_image = img;

            edited_image = edited_image.crop(304, 150, 335, 250);
            edited_image = edited_image.grayscale();
            //edited_image.save("test.png").unwrap();

            let parsed_image = match Image::from_dynamic_image(&edited_image) {
                Ok(img) => img,
                Err(e) => {
                    println!("Fehler beim Parsen des Bildes: {}", e);
                    return;
                }
            };
            let mut output: String = rusty_tesseract::image_to_string(&parsed_image, &my_args).unwrap();
            output = output.chars().filter(|c| !c.is_whitespace()).collect();
            println!("Bild erfolgreich abgerufen. Output: {}", output);
            write_to_csv(&output).unwrap();
        }
        Err(e) => println!("Es ist ein Fehler aufgetreten: {}", e),
    }
}

fn fetch_image() -> Result<DynamicImage, String> {
    let img = reqwest::blocking::get(MY_URL);

    match img {
        Ok(img) => match img.bytes() {
            Ok(img_bytes) => {
                let img = image::load_from_memory(&img_bytes).unwrap();
                Ok(img)
            }
            Err(e) => Err(format!("Fehler beim Abrufen des Bildes: {}", e)),
        },
        Err(e) => Err(format!("Fehler beim Abrufen des Bildes: {}", e)),
    }
}

fn write_to_csv(data: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("output.csv")?;

    writeln!(
        file,
        "{},{}",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
        data
    )?;
    Ok(())
}
