use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read, Seek};
use std::path::Path;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
enum TourType {
    Leicht,
    Medium,
    Monster,
    Zerstörer,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
enum Person {
    Mama,
    Papa,
    Jesko,
    Jannis,
}

#[derive(Serialize, Deserialize, Clone)]
struct Tour {
    name: String,
    kilometer: f32,
    höhenmeter: i32,
    tour_type: Option<TourType>,
    teilnehmer: Vec<Person>,
    schwierigkeitsBonus: i32,
    geld: HashMap<String, i32>,
}

fn write_to_json(tour: &Tour) {
    let path = "./tour_data.json";
    let path = Path::new(path);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .expect("Failed to open file");

    let mut tours: Vec<Tour> = match file.metadata().map(|m| m.len() > 0) {
        Ok(true) => {
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Failed to read file content");
            serde_json::from_str(&content).expect("Failed to deserialize JSON")
        },
        _ => Vec::new(),
    };

    tours.push(tour.clone());

    file.set_len(0).expect("Failed to truncate file");
    file.seek(std::io::SeekFrom::Start(0)).expect("Failed to seek to start of file");

    let json_string = serde_json::to_string_pretty(&tours).expect("Failed to serialize tours to JSON");

    match file.write_all(json_string.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", path.display(), why),
        Ok(_) => println!("Successfully wrote JSON to {}", path.display()),
    }
}

impl Tour {
    pub fn new(name: &str, kilometer: f32, höhenmeter: i32, teilnehmer: Vec<Person>, schwierigkeitsBonus: i32) {
        let name = name.to_string();
        let mut tour_type: Option<TourType> = None;
        if höhenmeter >= 0 && höhenmeter <= 499 {
            tour_type = Some(TourType::Leicht);
        }
        if höhenmeter >= 500 && höhenmeter <= 999 {
            tour_type = Some(TourType::Medium);
        }
        if höhenmeter >= 1000 && höhenmeter <= 1399 {
            tour_type = Some(TourType::Monster);
        }
        if höhenmeter >= 1400 {
            tour_type = Some(TourType::Zerstörer);
        }

        let tour = Tour {
            name,
            kilometer,
            höhenmeter,
            tour_type,
            teilnehmer: teilnehmer.clone(),
            schwierigkeitsBonus,
            geld: HashMap::new(),
        };

        let mut geld = HashMap::new();
        geld.insert("Jannis".to_string(), 0);
        geld.insert("Jesko".to_string(), 0);
        for person in &teilnehmer {
            match person {
                Person::Jannis => {
                    let jannis_geld = tour.calculate_money(Person::Jannis);
                    geld.insert("Jannis".to_string(), jannis_geld);
                },
                Person::Jesko => {
                    let jesko_geld = tour.calculate_money(Person::Jesko);
                    geld.insert("Jesko".to_string(), jesko_geld);
                },
                _ => (),
            }
        }

        let mut tour_with_geld = tour.clone();
        tour_with_geld.geld = geld;
        write_to_json(&tour_with_geld);
    }

    pub fn calculate_money(&self, person: Person) -> i32 {
        let mut money = 0;
        if self.teilnehmer.contains(&person) {
            money += self.schwierigkeitsBonus;

            // tour type money
            match self.tour_type {
                Some(TourType::Medium) => money += 3,
                Some(TourType::Monster) => money += 5,
                Some(TourType::Zerstörer) => money += 10,
                _ => println!("Tour type not found.."),
            }

            if self.kilometer >= 10.0 {
                money += 1;
            }

            if self.kilometer >= 15.0 {
                money += 1;
            }
        }

        money
    }
}

fn main() {
    Tour::new("test tour", 13.5, 1150, vec![Person::Mama, Person::Jesko, Person::Papa, Person::Jannis], 2)
}
