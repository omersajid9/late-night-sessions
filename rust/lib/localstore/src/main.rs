use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

// Define the trait
trait Persistable: Serialize + for<'a> Deserialize<'a> {
    fn save(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_vec(self)?;
        let mut file = File::create(filename)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    fn load(filename: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        let mut file = File::open(filename)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let deserialized = serde_json::from_slice(&contents)?;
        Ok(deserialized)
    }
}

// Example struct implementing Persistable
#[derive(Serialize, Deserialize,Debug)]
struct Person {
    name: String,
    age: u32,
    hello: String
}
impl Persistable for Person {}

fn main() -> Result<(), Box<dyn Error>> {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        hello: "TERI MAAAA".to_string()
    };

    // Save the person to a file
    person.save("person.json")?;

    // Load the person from the file
    let loaded_person: Person = Person::load("person.json")?;
    println!("Loaded person: {:?}", loaded_person);

    Ok(())
}