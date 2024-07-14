enum Tour_type {
    Leicht,
    Medium,
    Monster,
    Zerstörer,
}
#[derive(PartialEq)]
enum Person {
    Mama,
    Papa,
    Jesko,
    Jannis,
}

struct Tour {
    name: String,
    kilometer: f32,
    höhenmeter: i32,
    tour_type: Option<Tour_type>,
    teilnehmer: Vec<Person>,
    schwierigkeitsBonus: i32,
}


fn writeToJSON(tour: &Tour){

}

impl Tour {
    fn new(name: &str, kilometer: f32, höhenmeter: i32, teilnehmer: Vec<Person>, schwierigkeitsBonus: i32) -> Tour{
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
        if höhenmeter >= 1400{
            tour_type = Some(Tour_type::Zerstörer);
        }
        
        Tour {
            name,
            kilometer,
            höhenmeter,
            tour_type,
            teilnehmer,
            schwierigkeitsBonus,
        }
    }

    fn calculateMoney(&self, person: Person) -> i32{
        let mut money = 0;
        if self.teilnehmer.contains(&person) {
            money += self.schwierigkeitsBonus;

            // tour type money
            match self.tour_type{
                Some(Tour_type::Medium)=> money += 3,
                Some(Tour_type::Monster) => money += 5,
                Some(Tour_type::Zerstörer) => money += 10,
                _ => println!("Tour type not found.."),
            }
    
            if self.kilometer >= 10.0 {
                money += 1;
            }
    
            if self.kilometer >= 15.0 {
                money += 1
            }  
        }

        money   
    }
}


fn test(){
    let neueTour = Tour::new("test", 9.0, 500, vec![Person::Jannis, Person::Jesko], 0);
    let money = neueTour.calculateMoney(Person::Jesko);

    println!("{:?}", money);
}