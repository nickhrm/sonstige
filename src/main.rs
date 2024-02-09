use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::Value;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::{env, fs}; // Import the `blocking` module from `reqwest`
fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // Überprüfe, ob genügend Argumente übergeben wurden
    if args.len() < 2 {
        println!("Bitte gib eine Option an.");
        return;
    }

    // Das erste Argument (args[0]) ist der Name des Programms selbst,
    // daher beginnen wir bei args[1] für die erste echte Option
    let file_path = &args[1];

    // Hier kannst du die Option weiter verarbeiten
    // In diesem Beispiel drucken wir einfach die Option auf der Konsole aus
    println!("Die Option ist: {}", file_path);
    File::create("result.txt").expect("Should have been able to create the file");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    for line in lines {
        match extract_url(line) {
            Some(url) => {
                println!("URL found: {}", url);
                match extract_link_components(url) {
                    Some((category, description)) => {
                        println!("Category: {}", category);
                        println!("Description: {}", description);
                        match get_id(&category, &description) {
                            Some(id) => {
                                println!("ID: {}", id);
                                let og = extract_og_url(line);
                                println!("og_url: {}", &og);
                                write_to_file(og, &id.to_string(), &category).unwrap();
                            }

                            None => println!("No ID found"),
                        }
                    }
                    None => println!("No link components found in URL: {}", url),
                }
            }
            None => println!("No URL found in line: {}", line),
        }
        println!("");
    }
}

//holt URL aus zeile in Datei
fn extract_url(line: &str) -> Option<&str> {
    let start_pattern = "https://stadtmarketing-lehrte.de";

    if let Some(start_index) = line.find(start_pattern) {
        let url_part = &line[start_index..];
        let splitted: Vec<&str> = url_part.split(' ').collect();
        return Some(splitted[0]);
    }

    None // Falls kein passender Link gefunden wurde
}

//holt Kategorie und Titel und start
fn extract_link_components(link: &str) -> Option<(String, String)> {
    // Teile den Link anhand des Schrägstrichs (/) auf
    let parts: Vec<&str> = link.split('/').collect();

    // Überprüfe, ob genügend Teile vorhanden sind
    if parts.len() >= 4 {
        // Extrahiere die relevanten Teile des Links
        let mut category = String::from(parts[3]); // Beispiel: "Rad-%20und%20Wandertouren"
        let title = parts[4..].join("/"); // Beispiel: "Vom%20Rathaus%20Lehrte%20durch%20alle%20Ortsteile%20%7C%20Streckenl%C3%A4nge%20ca.%3A%2058%20km"
        category += "-beitraege"; // Füge die Dateiendung hinzu
                                  // Gib die extrahierten Teile als Tupel zurück
        return Some((category, title));
    } //holt Kategorie und Titel und start des rewrites aus

    None // Falls die Teile nicht ausreichend sind
}

fn get_id(hauptseite: &str, title: &str) -> Option<usize> {
    let url = "https://stadtmarketing-lehrte.de/cms/api/streuobstwiese-beitraege?filters[VorschauTitel][$eqi]=Kaiser%20Wilhelm&fields[0]=id";

    let composed_url = format!(
        "https://stadtmarketing-lehrte.de/cms/api/{}?filters[VorschauTitel][$eqi]={}&fields[0]=id",
        hauptseite, title
    );
    println!("Composed URL: {}", composed_url);

    let bearer: &str = "8ff9069f006859e5d2ec06979a13f027f2ad3005695c081c2dcb53f707f16be002db5b38e2784bb6d93438dd75837fd11002dca05687917fdbf19fdc24c1114444674fd8deb18ccf7c53add152165a08c3027e978399c23b019504d2b54e42455ab7f76819bef9e56edad5f705183b07ee6bab4e5f23f328d1ccc6b921a10a59";

    let client = reqwest::blocking::Client::new();
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", bearer)).unwrap(),
    );
    let response = client.get(composed_url).headers(headers).send();

    match response {
        Ok(response) => match response.text() {
            Ok(text) => {
                println!("Response: {:?}", &text);
                let json: Value = serde_json::from_str(&text).unwrap();
                let id = json["data"][0]["id"].as_u64();
                match id {
                    Some(id) => return Some(id as usize),
                    None => return Some(99999),
                }
            }
            Err(e) => Some(99999),
        },
        Err(e) => None,
    }
}

fn extract_og_url(line: &str) -> &str {
    let splitted: Vec<&str> = line.split(" ").collect();
    return splitted[1];
}

fn write_to_file(og: &str, id: &str, category: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("result.txt")?;

    writeln!(file, "{} {}/{}", og, category, id)?;

    Ok(())
}
