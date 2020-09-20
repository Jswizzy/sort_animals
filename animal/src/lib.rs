use std::fmt::Debug;
use std::cmp::Ordering;
use animal_derive::Animal;
use serde::{Deserialize, Serialize};

/// Type that has a weight
pub trait HasWeight {
    /// get the weight of an object
    fn weight(&self) -> i32;
}

/// Type that has a color
pub trait HasColor {
    /// get the color of an object
    fn color(&self) -> &str;
}

/// Type that implements [HasColor](HasColor) and [HasWeight](HasWeight) and represents a specific type of animal
#[typetag::serde]
pub trait Animal: HasColor + HasWeight + Debug {}

/// AnimalDAO is a type used to Serialize and Deserialize Animal JSON data in the form of
///
/// ```json
///   {
///     "type": "dog",
///     "weight": 17,
///     "color": "white"
///   }
/// ```
///
/// It is intended to be used as a DAO, data access object, only.
/// Serde handles the Serialization and Deserialization here.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimalDAO {
    #[serde(rename = "type")]
    type_name: String,
    weight: i32,
    color: String,
}

// Animal Types

// used a derive macro to impl traits without having to copy the same code over and over again.
// Had no clue how to use procedural macros in Rust before this.
// Rust is not an inheritance/subtyping-based language, enum might have been a better option but
// enums are homogeneous
#[derive(Animal, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "dog")]
pub struct Dog {
    weight: i32,
    color: String,
}

#[derive(Animal, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Cat {
    weight: i32,
    color: String,
}

#[derive(Animal, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Snake {
    weight: i32,
    color: String,
}

// utility methods

/// Convert an [AnimalDAO](AnimalDAO) into its [Animal](animal::Animal) Type
pub fn animal_dao_to_animal(animal: &AnimalDAO) -> Option<Box<dyn Animal>> {
    match animal.type_name.as_str() {
        "dog" => Some(Box::new(Dog::new(&animal.color, animal.weight))),
        "cat" => Some(Box::new(Cat::new(&animal.color, animal.weight))),
        "snake" => Some(Box::new(Snake::new(&animal.color, animal.weight))),
        _ => None
    }
}

/// given a slice of Animal return a heterogeneous vector of Animal types
pub fn convert_animals_to_types(animals: &[AnimalDAO]) -> Vec<Box<dyn Animal>> {
    animals.iter().filter_map(animal_dao_to_animal).collect()
}

// helpers

pub fn sort_by_color(a: &Box<dyn Animal>, b: &Box<dyn Animal>) -> Ordering {
    a.color().cmp(b.color())
}

pub fn sort_by_weight(a: &Box<dyn Animal>, b: &Box<dyn Animal>) -> Ordering {
    a.weight().cmp(&b.weight())
}