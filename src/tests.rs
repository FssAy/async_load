use crate::types::*;

#[test]
fn gml_string_build() {
    assert!("text".into_gml().is_ok());
    assert!(String::from("text").into_gml().is_ok());
    assert!(GMLString::none().is_null());
}

#[test]
fn gml_string_read_ascii() {
    const TEXT: &str = "async_load";

    let gml_string = TEXT.into_gml().unwrap();

    assert_eq!(
        gml_string.as_str().unwrap(),
        TEXT
    );
}

#[test]
fn gml_string_read_utf8() {
    const TEXT_SIMPLE: &str = "ąśyńć_łóąd";
    const TEXT_EMOJI: &str = "🍎🍇🍒";
    const TEXT_INVALID: &str = "async\0load";

    let gml_string = TEXT_SIMPLE.into_gml().unwrap();
    assert_eq!(
        gml_string.as_str().unwrap(),
        TEXT_SIMPLE
    );

    let gml_string = TEXT_EMOJI.into_gml().unwrap();
    assert_eq!(
        gml_string.as_str().unwrap(),
        TEXT_EMOJI
    );

    assert!(TEXT_INVALID.into_gml().is_err());

    let gml_string = GMLString::from(TEXT_INVALID as *const str as *const _);
    assert_ne!(
        gml_string.as_str().unwrap(),
        TEXT_INVALID
    );
}
