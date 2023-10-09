use crate::ds_map::EventType;
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

#[test]
fn event_type_conversion() {
    // --- invalid types
    assert_eq!(
        EventType::from(GMLDouble::none()),
        EventType::default()
    );
    assert_eq!(
        EventType::from(59.0),
        EventType::default()
    );
    assert_eq!(
        EventType::from(64.0),
        EventType::default()
    );
    assert_eq!(
        EventType::from(65.0),
        EventType::default()
    );
    assert_eq!(
        EventType::from(77.0),
        EventType::default()
    );

    // --- valid types
    let mut value = 0.0;

    let test_value = |value| {
        assert_eq!(
            EventType::from(value) as i32,
            value as i32,
        );
    };

    test_value(60.0);
    test_value(61.0);
    test_value(62.0);
    test_value(63.0);
    test_value(66.0);
    test_value(67.0);
    test_value(68.0);
    test_value(69.0);
    test_value(70.0);
    test_value(71.0);
    test_value(72.0);
    test_value(73.0);
    test_value(74.0);
    test_value(75.0);
    test_value(76.0);
}
