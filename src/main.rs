use regex::Regex;
use std::{env, fs};
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
                    }
                    None => println!("No link components found in URL: {}", url),
                }
            }
            None => println!("No URL found in line: {}", line),
        }
    }
}

//holt URL aus zeile in Datei
fn extract_url(line: &str) -> Option<&str> {
    // Suchmuster für den Beginn des Links
    let start_pattern = "https://stadtmarketing-lehrte.de";

    // Überprüfe, ob die Zeichenkette das Suchmuster enthält
    if let Some(start_index) = line.find(start_pattern) {
        // Extrahiere den Teil der Zeichenkette ab dem Startmuster
        let url_part = &line[start_index..];

        // Suchmuster für das Ende des Links (in diesem Fall ist das Ende durch das Leerzeichen markiert)
        let end_pattern = " ";

        // Suche nach dem ersten Leerzeichen nach dem Startmuster
        if let Some(end_index) = url_part.find(end_pattern) {
            // Extrahiere den Link bis zum ersten Leerzeichen
            let url = &url_part[..end_index];
            Some(url);
        }
    }

    None // Falls kein passender Link gefunden wurde
}

fn extract_link_components(link: &str) -> Option<(&str, String)> {
    // Teile den Link anhand des Schrägstrichs (/) auf
    let parts: Vec<&str> = link.split('/').collect();

    // Überprüfe, ob genügend Teile vorhanden sind
    if parts.len() >= 4 {
        // Extrahiere die relevanten Teile des Links
        let category = parts[3]; // Beispiel: "Rad-%20und%20Wandertouren"
        let description = parts[4..].join("/"); // Beispiel: "Vom%20Rathaus%20Lehrte%20durch%20alle%20Ortsteile%20%7C%20Streckenl%C3%A4nge%20ca.%3A%2058%20km"

        // Gib die extrahierten Teile als Tupel zurück
        return Some((category, description));
    }

    None // Falls die Teile nicht ausreichend sind
}
