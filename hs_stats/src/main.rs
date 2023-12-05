use std::collections::HashMap;

use image::DynamicImage;
use rusty_tesseract::{Args, Image};

const MY_URL: &str = "https://web.hochschulsport-hannover.de/campusfit/auslastungsgrafik";

fn main() {
    let my_args = Args {
        lang: String::from("eng"),
        dpi: Some(150),
        psm: Some(3),
        oem: Some(3),
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "1234567890".into(),
        )]),
    };

    match fetch_image() {
        Ok(mut img) => {

            let cropped_image = img.crop(304, 150, 335, 250);

            let parsed_image = match Image::from_dynamic_image(&cropped_image) {
                Ok(img) => img,
                Err(e) => {
                    println!("Fehler beim Parsen des Bildes: {}", e);
                    return;
                }
            };
            let output = rusty_tesseract::image_to_string(&parsed_image, &my_args).unwrap();
            println!("Bild erfolgreich abgerufen. Output: {}", output);
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
