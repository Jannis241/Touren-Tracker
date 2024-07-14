use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;
use std::io::Seek;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
enum Tour_type {
    Leicht,
    Medium,
    Monster,
    Zerstörer,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[derive(Clone)]
enum Person {
    Mama,
    Papa,
    Jesko,
    Jannis,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Tour {
    name: String,
    kilometer: f32,
    höhenmeter: i32,
    tour_type: Option<Tour_type>,
    teilnehmer: Vec<Person>,
    schwierigkeitsBonus: i32,
}

fn write_to_json(tour: &Tour) {
    let json_string = serde_json::to_string_pretty(tour).expect("Failed to serialize tour to JSON");

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
    pub fn new(name: &str, kilometer: f32, höhenmeter: i32, teilnehmer: Vec<Person>, schwierigkeitsBonus: i32) -> Tour {
        let name = name.to_string();
        let mut tour_type: Option<Tour_type> = None;
        if höhenmeter >= 0 && höhenmeter <= 499 {
            tour_type = Some(Tour_type::Leicht);
        }
        if höhenmeter >= 500 && höhenmeter <= 999 {
            tour_type = Some(Tour_type::Medium);
        }
        if höhenmeter >= 1000 && höhenmeter <= 1399 {
            tour_type = Some(Tour_type::Monster);
        }
        if höhenmeter >= 1400 {
            tour_type = Some(Tour_type::Zerstörer);
        }

        let tour = Tour {
            name,
            kilometer,
            höhenmeter,
            tour_type,
            teilnehmer,
            schwierigkeitsBonus,
        };
        write_to_json(&tour);
        tour
    }

    pub fn calculate_money(&self, person: Person) -> i32 {
        let mut money = 0;
        if self.teilnehmer.contains(&person) {
            money += self.schwierigkeitsBonus;

            // tour type money
            match self.tour_type {
                Some(Tour_type::Medium) => money += 3,
                Some(Tour_type::Monster) => money += 5,
                Some(Tour_type::Zerstörer) => money += 10,
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
    let neue_tour = Tour::new("te2s23434242344t", 9.0, 500, vec![Person::Jannis, Person::Jesko], 0);
    let money = neue_tour.calculate_money(Person::Jesko);
}
