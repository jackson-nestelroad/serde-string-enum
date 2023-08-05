use serde_string_enum::{
    DeserializeLabeledStringEnum,
    SerializeLabeledStringEnum,
};

#[derive(SerializeLabeledStringEnum, DeserializeLabeledStringEnum)]
enum Type {
    #[string = "Grass"]
    Grass,
    Fire,
    Water,
}

fn main() {}
