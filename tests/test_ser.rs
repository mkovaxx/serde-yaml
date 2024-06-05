use indoc::indoc;
use serde::Serialize;
use serde_derive::Serialize;
use serde_yaml::{Serializer, SerializerConfig};

#[test]
fn test_default_unit_variants() {
    #[derive(Serialize)]
    enum Enum {
        Unit,
    }

    let mut buffer = vec![];

    let mut ser = Serializer::new(&mut buffer);
    Enum::Unit.serialize(&mut ser).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    let expected = indoc! {"
        Unit
    "};

    assert_eq!(output, expected);
}

#[test]
fn test_tag_unit_variants() {
    #[derive(Serialize)]
    enum Enum {
        Unit,
    }

    let mut buffer = vec![];
    let mut ser = Serializer::new_with_config(
        &mut buffer,
        SerializerConfig {
            tag_unit_variants: true,
        },
    );
    Enum::Unit.serialize(&mut ser).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    let expected = indoc! {"
        !Unit
    "};

    assert_eq!(output, expected);
}
