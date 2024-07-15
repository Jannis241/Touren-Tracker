use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read, Seek};
use std::path::Path;
use std::collections::HashMap;
use colored::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
enum TourType {
    Leicht,
    Medium,
    Monster,
    Zerstörer,
}


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
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
    pub fn new(name: &str, kilometer: f32, höhenmeter: i32, teilnehmer: Vec<Person>, schwierigkeitsBonus: i32, writetojson: bool) -> Tour {
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
            name: name.clone(),
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

        if writetojson {
            let touren = load_all_touren();

            let mut this_tour_already_exists = false;
            for tour in touren {
                if tour.name == name.clone() {
                    this_tour_already_exists = true;
                    break;
                }
            }

            if !this_tour_already_exists {
                write_to_json(&tour_with_geld);
            }
            else {
                println!("tour {} already exists", name.clone());
            }
        }

        tour_with_geld
    }

    fn calculate_money(&self, person: Person) -> i32 {
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

fn new_lines(num_of_lines: usize) {
    println!("{}", "\n".repeat(num_of_lines));
}

fn get_input(string: &str) -> String {
    print!("{}", string.bold().bright_blue());
    let _ = io::stdout().flush();

    let mut input = "".to_string();

    let _ = io::stdin().read_line(&mut input);
    let input = input.trim().to_lowercase();

    input
}



fn load_all_touren() -> Vec<Tour> {
    let mut touren = vec![];
    
    let file = std::fs::File::open("tour_data.json").expect("couldnt load tour_data.json");
    let json: serde_json::Value = serde_json::from_reader(file).expect("couldnt load tour_data.json");

    let touren_json = json.as_array().unwrap();
    for tour in touren_json {
        let height = tour.get("höhenmeter").unwrap().as_i64().unwrap() as i32;
        let length = tour.get("kilometer").unwrap().as_f64().unwrap() as f32;
        let name = tour.get("name").unwrap().as_str().unwrap();
        let difficulty = tour.get("schwierigkeitsBonus").unwrap().as_i64().unwrap() as i32;
        let teilnehmer = tour.get("teilnehmer").unwrap();
        let mut teilnehmer_enums = vec![];

        for teilnehmer in teilnehmer.as_array().unwrap() {
            teilnehmer_enums.push(
                match teilnehmer.as_str().unwrap().to_lowercase().as_str() {
                    "mama" => Person::Mama,
                    "papa" => Person::Papa,
                    "jesko" => Person::Jesko,
                    "jannis" => Person::Jannis,
                    other => panic!("teilnehmer {} not found", other)
                }
            )
        }
        let tour_type = tour.get("tour_type").unwrap();

        let tour = Tour::new(name, length, height, teilnehmer_enums, difficulty, false);

        touren.push(tour);
    }
    touren
}

impl Debug for Tour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
"\n----------------------------------------------------
Tour: 
    Name: {},
    Kilometer: {},
    Höhenmeter: {},
    Tour Typ: {:?},
    Teilnehmer: {:?},
    Schwierigkeit: {},
                    
    Geld für diese Tour für Jannis: {}
    Geld für diese Tour für Jesko: {}
----------------------------------------------------\n",
                self.name,
                self.kilometer,
                self.höhenmeter,
                self.tour_type,
                self.teilnehmer,
                self.schwierigkeitsBonus,
                self.calculate_money(Person::Jannis),
                self.calculate_money(Person::Jesko)
        )
    }
}

fn main() {
    
    new_lines(10);
    println!("{}", "WILKOMMEN ZUM TOUREN TRACKER".yellow().bold().underline());
    new_lines(1);
        println!("{}", "commands: \n     'new' um eine neue Tour zu erstellen, \n     'Geld', um das Geld für alle Personen zu berechnen,\n     'delete (name)' um eine Tour zu löschen,\n     'list' um alle Touren an zu zeigen, \n     'show (name) um eine bestimmte Tour an zu zeigen'\n     'exit' um den Touren-Tracker zu stoppen".bold().blue());
    loop {
        new_lines(1);
        print!("{}", "<Touren-Tracker> ".bold().bright_blue());
        let _ = io::stdout().flush();

        let mut input = "".to_string();

        let _ = io::stdin().read_line(&mut input);
        let input = input.trim().to_lowercase();
        let arguments = input.split_whitespace().collect::<Vec<&str>>();

        match arguments[0] {
            "new" => {
                new_lines(1);

                let name = get_input("Name der Tour: ".green().bold().to_string().as_str()).to_lowercase();
                let length: f32;
                loop {
                    let länge_start = get_input("Länge der Tour (in Kilometer): ".green().bold().to_string().as_str()).to_lowercase();
                    let länge = länge_start.replace(",", ".");
                    let länge = länge.replace("km", "");
                    let länge = länge.replace("kilometer", "");
                    let länge = länge.trim().parse::<f32>();

                    if länge.is_ok() {
                        length = länge.unwrap();
                        break;
                    }
                    else {
                        println!("{}{} {}{} {}", "ERROR".bold().red().underline(), ":".red().bold(), länge_start.red().bold(), ",".red(), "ist keine Nummer, bitte erneut angeben".red());
                    }
                }
                let height: i32;
                loop {
                    let höhe_start = get_input("Höhenmeter der Tour (in Meter): ".green().bold().to_string().as_str()).to_lowercase();
                    let höhe = höhe_start.replace(",", ".");
                    let höhe = höhe.replace("m", "");
                    let höhe = höhe.replace("meter", "");
                    let höhe = höhe.trim().parse::<i32>();

                    if höhe.is_ok() {
                        height = höhe.unwrap();
                        break;
                    }
                    else {
                        println!("{}{} {}{} {}", "ERROR".bold().red().underline(), ":".red().bold(), höhe_start.red().bold(), ",".red(), "ist keine Nummer, bitte erneut angeben".red());
                    }
                }
                let mut personen_enums = vec![];
                loop {
                    let personen = get_input(&format!("{} {}", "Welche Personen haben".green().bold(), "teilgenommen (getrennt mit Komma): ".green().bold()));
                    let personen = personen.trim().split(",").collect::<Vec<&str>>();
                    
                    let mut should_break = true;
                    for person in personen {
                        let person = person.replace(",", "").trim().to_lowercase();
                        let person_enum = match person.as_str() {
                            "jannis" => Some(Person::Jannis),
                            "jesko" => Some(Person::Jesko),
                            "mama" => Some(Person::Mama),
                            "papa" => Some(Person::Papa),
                            other => {
                                println!("{}{} {}{} {}", "ERROR".bold().red().underline(), ":".red().bold(), person.red().bold(), ",".red(), "ist kein Name, bitte erneut angeben".red());
                                None
                            }
                        };

                        if person_enum.is_none() {
                            should_break = false;
                            break;
                        }
                        else {
                            personen_enums.push(person_enum.unwrap())
                        }
                    }

                    if should_break == false {
                        continue;
                    }
                    else {
                        break;
                    }
                }
                let difficulty: i32;
                loop {
                    let schwierigkeit_start = get_input("Schwierigkeits Bonus (in Euro): ".green().bold().to_string().as_str()).to_lowercase();
                    let schwierigkeit = schwierigkeit_start.replace("€", "");
                    let schwierigkeit = schwierigkeit.trim().parse::<i32>();

                    if schwierigkeit.is_ok() {
                        difficulty = schwierigkeit.unwrap();
                        break;
                    }
                    else {
                        println!("{}{} {}{} {}", "ERROR".bold().red().underline(), ":".red().bold(), schwierigkeit_start.red().bold(), ",".red(), "ist keine nummer, bitte erneut angeben".red());
                    }
                }
                
                Tour::new(name.as_str(), length, height, personen_enums, difficulty, true);
                
                println!("{}{}", "SUCSESS".green().underline().bold(), ", eine neue Tour wurde erstellt".green().bold());
            },
            "geld" => {
                let touren = load_all_touren();

                let mut geld_jannis_insgesamt = 0;
                let mut geld_jesko_insgesamt = 0;

                for tour in touren {
                    let geld_jannis = tour.geld.get("Jannis").unwrap();
                    let geld_jesko = tour.geld.get("Jesko").unwrap();
                    println!("Name: {}, \n   Geld für Jannis: {}, \n   Geld für Jesko: {}\n", tour.name, geld_jannis, geld_jesko);
                    
                    geld_jannis_insgesamt += geld_jannis;
                    geld_jesko_insgesamt += geld_jesko;
                }

                new_lines(1);

                println!("Geld insgesamt für Jannis: {}", geld_jannis_insgesamt);
                println!("Geld insgesamt für Jesko: {}", geld_jesko_insgesamt);
            },
            "list" => {
                let touren = load_all_touren();

                for tour in touren {
                    println!("{:?}", tour);
                }
            },
            "show" => {
                let second_argument_slice: &[&str] = &arguments[1..];
                let mut second_argument: Vec<String> = vec![];
                for arg in second_argument_slice {

                    second_argument.push(arg.replace(",", "").trim().to_string())
                }

                let second_argument = second_argument.join(" ");

                let touren = load_all_touren();

                let mut found_tour = false;

                for tour in touren {
                    if tour.name == second_argument {
                        println!("{:?}", tour);
                        found_tour = true;
                        break;
                    }
                }

                if !found_tour {
                    println!("{}{} {}{} {}", "ERROR".bold().red().underline(), ":".red().bold(), "Tour: ".red(), second_argument.trim().red().bold(), ", nicht gefunden".red());
                }
            },
            "delete" => {
                
            },
            "exit" => break,
            other => println!("{}{} {}, {}", "ERROR".bold().red().underline(), ":".red().bold(), other.red().bold(), "is kein command".red())
        };
    }
    

    
}
