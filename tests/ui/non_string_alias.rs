use serde_string_enum::{
    DeserializeLabeledStringEnum,
    SerializeLabeledStringEnum,
};

#[derive(SerializeLabeledStringEnum, DeserializeLabeledStringEnum)]
enum Type {
    #[string = "Grass"]
    Grass,
    #[string = "Fire"]
    #[alias = 1234]
    Fire,
    #[string = "Water"]
    Water,
}

fn main() {}
