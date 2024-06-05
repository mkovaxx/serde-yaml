use indoc::indoc;
use serde::Serialize;
use serde_derive::Serialize;
use serde_yaml::Serializer;

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

    let expected = indoc! { r#"
        Unit
    "# };

    assert_eq!(output, expected);
}

#[test]
fn test_tagged_unit_variants() {
    #[derive(Serialize)]
    enum Enum {
        Unit,
    }

    let mut buffer = vec![];

    let mut ser = Serializer::new_with_settings(&mut buffer, true);
    Enum::Unit.serialize(&mut ser).unwrap();
    let output = String::from_utf8(buffer).unwrap();

    let expected = indoc! { r#"
        !Unit
    "# };

    assert_eq!(output, expected);
}
