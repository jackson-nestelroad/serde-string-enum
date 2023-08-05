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
        Grass,
        #[string = "Fire"]
        Fire,
        #[string = "Water"]
        Water,
    }

    #[test]
    fn derives_to_string() {
        assert_eq!(Type::Grass.to_string(), "Grass");
        assert_eq!(Type::Fire.to_string(), "Fire");
        assert_eq!(Type::Water.to_string(), "Water");
    }

    #[test]
    fn derives_serialize() {
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
    fn from_str_case_insensitive() {
        assert_eq!(Type::from_str("grass").unwrap(), Type::Grass);
        assert_eq!(Type::from_str("FIRE").unwrap(), Type::Fire);
        assert_eq!(Type::from_str("wAtEr").unwrap(), Type::Water);
    }

    #[test]
    fn deserialize_case_insensitive() {
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
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "L" => Self::Left,
                "R" => Self::Right,
                _ => return Err(format!("invalid rotation {s}")),
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
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match &s[0..1] {
                "S" => Self::Stay,
                "F" => Self::Forward(s[1..].parse::<u8>().map_err(|err| err.to_string())?),
                "R" => Self::Rotate(Rotation::from_str(&s[1..])?),
                _ => return Err(format!("invalid move {s}")),
            })
        }
    }

    #[test]
    fn derives_serialize() {
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
