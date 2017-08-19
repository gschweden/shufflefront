extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    team: String,
}

fn main() {
    println!("\n\n");

    let p = load_all_participants();


    println!("Please call {} at the number {}\n", p["Ben Allison"].name, p["Ben Allison"].team);


    add_new_participant("Bob Cat", "CORE");
}

fn add_new_participant(name : &str, team : &str) {

    let mut all_participants = load_all_participants();
    let participant = Person {name : name, team : team};

    all_participants.insert(name, participant);





}

fn find_all_matches() {

}

fn load_all_participants() -> HashMap<String, Person>
{
    let data = r#"{
        "Ben Allison": {
            "name": "Ben Allison",
            "team": "core-dev"
        },
        "Geraldine Schweden": {
            "name": "Geraldine Schweden",
            "team": "core-dev"
        },
        "Scott": {
            "name": "Scott",
            "team": "sales"
        }
    }"#;
    return serde_json::from_str(data).unwrap();
}

fn get_past_matches_by_participant() {

}

fn save_matches(){

}