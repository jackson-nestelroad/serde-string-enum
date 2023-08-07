// Write tests for no_std to test that our library is actually generated no_std-compatible code.
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(test)]
mod labeled_strings {
    use core::str::FromStr;
    use serde_string_enum::{
        DeserializeLabeledStringEnum,
        SerializeLabeledStringEnum,
    };

    #[derive(Debug, PartialEq, SerializeLabeledStringEnum, DeserializeLabeledStringEnum)]
    enum Type {
        #[string = "Grass"]
        #[alias = "Leaf"]
        Grass,
        #[string = "Fire"]
        #[alias = "Flame"]
        #[alias = "Hot"]
        Fire,
        #[string = "Water"]
        Water,
    }

    #[test]
    fn derives_display() {
        extern crate alloc;
        use alloc::fmt::format;

        assert_eq!(format(format_args!("{}", Type::Grass)), "Grass");
        assert_eq!(format(format_args!("{}", Type::Fire)), "Fire");
        assert_eq!(format(format_args!("{}", Type::Water)), "Water");
    }

    #[test]
    #[cfg(feature = "std")]
    fn derives_to_string() {
        extern crate std;
        use std::string::ToString;

        assert_eq!(Type::Grass.to_string(), "Grass");
        assert_eq!(Type::Fire.to_string(), "Fire");
        assert_eq!(Type::Water.to_string(), "Water");
    }

    #[test]
    fn derives_serialize() {
        extern crate alloc;
        use alloc::vec;

        assert_eq!(serde_json::to_string(&Type::Grass).unwrap(), "\"Grass\"");
        assert_eq!(serde_json::to_string(&Type::Fire).unwrap(), "\"Fire\"");
        assert_eq!(serde_json::to_string(&Type::Water).unwrap(), "\"Water\"");

        let types = vec![Type::Grass, Type::Fire, Type::Water];
        assert_eq!(
            serde_json::to_string(&types).unwrap(),
            "[\"Grass\",\"Fire\",\"Water\"]"
        );
    }

    #[test]
    fn derives_from_str() {
        assert_eq!(Type::from_str("Grass").unwrap(), Type::Grass);
        assert_eq!(Type::from_str("Fire").unwrap(), Type::Fire);
        assert_eq!(Type::from_str("Water").unwrap(), Type::Water);
    }

    #[test]
    fn derives_deserialize() {
        extern crate alloc;
        use alloc::{
            vec,
            vec::Vec,
        };

        assert_eq!(
            serde_json::from_str::<Type>("\"Grass\"").unwrap(),
            Type::Grass
        );
        assert_eq!(
            serde_json::from_str::<Type>("\"Fire\"").unwrap(),
            Type::Fire
        );
        assert_eq!(
            serde_json::from_str::<Type>("\"Water\"").unwrap(),
            Type::Water
        );

        assert_eq!(
            serde_json::from_str::<Vec<Type>>("[\"Grass\",\"Fire\",\"Water\"]").unwrap(),
            vec![Type::Grass, Type::Fire, Type::Water],
        );
    }

    #[test]
    #[cfg(feature = "unicase")]
    fn from_str_case_insensitive() {
        assert_eq!(Type::from_str("grass").unwrap(), Type::Grass);
        assert_eq!(Type::from_str("FIRE").unwrap(), Type::Fire);
        assert_eq!(Type::from_str("wAtEr").unwrap(), Type::Water);
    }

    #[test]
    #[cfg(feature = "unicase")]
    fn deserialize_case_insensitive() {
        extern crate alloc;
        use alloc::{
            vec,
            vec::Vec,
        };

        assert_eq!(
            serde_json::from_str::<Type>("\"grass\"").unwrap(),
            Type::Grass
        );
        assert_eq!(
            serde_json::from_str::<Type>("\"FIRE\"").unwrap(),
            Type::Fire
        );
        assert_eq!(
            serde_json::from_str::<Type>("\"wAtEr\"").unwrap(),
            Type::Water
        );

        assert_eq!(
            serde_json::from_str::<Vec<Type>>("[\"grass\",\"fire\",\"WATER\"]").unwrap(),
            vec![Type::Grass, Type::Fire, Type::Water],
        );
    }

    #[test]
    fn from_str_aliases() {
        assert_eq!(Type::from_str("Leaf").unwrap(), Type::Grass);
        assert_eq!(Type::from_str("Flame").unwrap(), Type::Fire);
        assert_eq!(Type::from_str("Hot").unwrap(), Type::Fire);
    }

    #[test]
    #[cfg(feature = "unicase")]
    fn from_str_aliases_case_insensitive() {
        assert_eq!(Type::from_str("LEAF").unwrap(), Type::Grass);
        assert_eq!(Type::from_str("flame").unwrap(), Type::Fire);
        assert_eq!(Type::from_str("HOt").unwrap(), Type::Fire);
    }

    #[test]
    fn deserializes_aliases() {
        assert_eq!(
            serde_json::from_str::<Type>("\"Leaf\"").unwrap(),
            Type::Grass
        );
        assert_eq!(
            serde_json::from_str::<Type>("\"Flame\"").unwrap(),
            Type::Fire
        );
        assert_eq!(serde_json::from_str::<Type>("\"Hot\"").unwrap(), Type::Fire);
    }

    #[test]
    #[cfg(feature = "unicase")]
    fn deserializes_aliases_case_insensitive() {
        assert_eq!(
            serde_json::from_str::<Type>("\"leAF\"").unwrap(),
            Type::Grass
        );
        assert_eq!(
            serde_json::from_str::<Type>("\"FLamE\"").unwrap(),
            Type::Fire
        );
        assert_eq!(serde_json::from_str::<Type>("\"hot\"").unwrap(), Type::Fire);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn invalid_value_string() {
        extern crate alloc;
        use alloc::string::String;

        assert_eq!(
            Type::from_str("bad").err(),
            Some(String::from("invalid Type: bad"))
        )
    }

    #[test]
    #[cfg(not(feature = "alloc"))]
    fn invalid_value_string() {
        assert_eq!(Type::from_str("bad").err(), Some("invalid value"))
    }
}

#[cfg(test)]
mod custom_string_conversion {
    use core::{
        fmt::{
            Display,
            Formatter,
        },
        str::FromStr,
    };

    use serde_string_enum::{
        DeserializeStringEnum,
        SerializeStringEnum,
    };

    #[derive(Debug, PartialEq)]
    enum Rotation {
        Left,
        Right,
    }

    impl Display for Rotation {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::Left => write!(f, "L"),
                Self::Right => write!(f, "R"),
            }
        }
    }

    impl FromStr for Rotation {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "L" => Self::Left,
                "R" => Self::Right,
                _ => return Err("invalid rotation"),
            })
        }
    }

    #[derive(Debug, PartialEq, SerializeStringEnum, DeserializeStringEnum)]
    enum Move {
        Stay,
        Forward(u8),
        Rotate(Rotation),
    }

    impl Display for Move {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::Stay => write!(f, "S"),
                Self::Forward(n) => write!(f, "F{n}"),
                Self::Rotate(r) => write!(f, "R{r}"),
            }
        }
    }

    impl FromStr for Move {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match &s[0..1] {
                "S" => Self::Stay,
                "F" => Self::Forward(s[1..].parse::<u8>().map_err(|_| "invalid forward number")?),
                "R" => Self::Rotate(Rotation::from_str(&s[1..])?),
                _ => return Err("invalid move"),
            })
        }
    }

    #[test]
    fn derives_serialize() {
        extern crate alloc;
        use alloc::vec;

        assert_eq!(serde_json::to_string(&Move::Stay).unwrap(), "\"S\"");
        assert_eq!(serde_json::to_string(&Move::Forward(1)).unwrap(), "\"F1\"");
        assert_eq!(
            serde_json::to_string(&Move::Rotate(Rotation::Left)).unwrap(),
            "\"RL\""
        );

        let moves = vec![
            Move::Stay,
            Move::Forward(20),
            Move::Rotate(Rotation::Right),
            Move::Forward(10),
        ];
        assert_eq!(
            serde_json::to_string(&moves).unwrap(),
            "[\"S\",\"F20\",\"RR\",\"F10\"]"
        );
    }

    #[test]
    fn derives_deserialize() {
        extern crate alloc;
        use alloc::{
            vec,
            vec::Vec,
        };

        assert_eq!(serde_json::from_str::<Move>("\"S\"").unwrap(), Move::Stay);
        assert_eq!(
            serde_json::from_str::<Move>("\"F123\"").unwrap(),
            Move::Forward(123)
        );
        assert_eq!(
            serde_json::from_str::<Move>("\"RR\"").unwrap(),
            Move::Rotate(Rotation::Right)
        );

        assert_eq!(
            serde_json::from_str::<Vec<Move>>("[\"S\",\"F20\",\"RR\",\"F10\"]").unwrap(),
            vec![
                Move::Stay,
                Move::Forward(20),
                Move::Rotate(Rotation::Right),
                Move::Forward(10),
            ],
        );
    }
}
