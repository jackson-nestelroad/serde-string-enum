use serde_string_enum::{DeserializeStringEnum, SerializeStringEnum};

#[derive(SerializeStringEnum, DeserializeStringEnum)]
struct Type {
    name: String,
}

fn main() {}
