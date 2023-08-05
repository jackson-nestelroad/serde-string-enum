# serde_string_enum

This crate provides a procedural macro to automatically derive [serde](https://serde.rs/)'s `Serialize` and `Deserialize` traits for enum types that should be encoded as a single string.

```toml
[dependencies]
serde = "1.0"
serde_string_enum = "0.1"
unicase = "2.6.0"
```
## Usage
This crate defines two pairs of macros:

- `SerializeLabeledStringEnum` / `DeserializeLabeledStringEnum` - Uses the `#[string = ...]` attribute on each enum variant to perform string conversions.
- `SerializeStringEnum` / `DeserializeStringEnum`  - Uses the enum type's `ToString` and `FromStr` implementations to perform string conversions.

## Examples:
### Labeled Strings
```
use serde_string_enum::{
    DeserializeLabeledStringEnum,
    SerializeLabeledStringEnum,
};

#[derive(Debug, PartialEq, SerializeLabeledStringEnum, DeserializeLabeledStringEnum)]
enum Type {
    #[string = "Grass"]
    Grass,
    #[string = "Fire"]
    Fire,
    #[string = "Water"]
    Water,
}

fn main() -> serde_json::Result<()> {
    let j = serde_json::to_string(&Type::Grass)?;
    assert_eq!(j, "\"Grass\"");
    let t: Type = serde_json::from_str(&j)?;
    assert_eq!(t, Type::Grass);

    // Case-insensitive conversion also works.
    let t: Type = serde_json::from_str("\"fire\"")?;
    assert_eq!(t, Type::Fire);

    Ok(())
}
```

### Enums with Display and FromStr
```
use core::{
    fmt::Display,
    str::FromStr,
};
use serde_string_enum::{
    DeserializeStringEnum,
    SerializeStringEnum,
};

#[derive(Debug, PartialEq, SerializeStringEnum, DeserializeStringEnum)]
enum Move {
    Stay,
    Forward(u8),
    Left(u8),
}

impl Display for Move {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Stay => write!(f, "S"),
            Self::Forward(n) => write!(f, "F{n}"),
            Self::Left(n) => write!(f, "L{n}"),
        }
    }
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s[0..1] {
            "S" => Self::Stay,
            "F" => Self::Forward(s[1..].parse::<u8>().map_err(|err| err.to_string())?),
            "L" => Self::Left(s[1..].parse::<u8>().map_err(|err| err.to_string())?),
            _ => return Err(format!("invalid move {s}")),
        })
    }
}

fn main() -> serde_json::Result<()> {
    let j = serde_json::to_string(&Move::Forward(10))?;
    assert_eq!(j, "\"F10\"");
    let m: Move = serde_json::from_str(&j)?;
    assert_eq!(m, Move::Forward(10));

    let moves: Vec<Move> = serde_json::from_str("[\"S\",\"F2\",\"S\",\"L4\"]")?;
    assert_eq!(
        moves,
        vec![Move::Stay, Move::Forward(2), Move::Stay, Move::Left(4)]
    );

    Ok(())
}
```