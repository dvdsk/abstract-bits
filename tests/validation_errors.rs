use abstract_bits::{AbstractBits, abstract_bits};

#[abstract_bits]
#[derive(Debug, PartialEq)]
struct TestOption {
    has_value: bool,
    #[abstract_bits(presence_from = has_value)]
    value: Option<u8>,
}

#[abstract_bits]
#[derive(Debug, PartialEq)]
struct TestVec {
    length: u5,
    #[abstract_bits(length_from = length)]
    data: Vec<u8>,
}

#[abstract_bits]
#[derive(Debug, PartialEq)]
struct ComplexStruct {
    has_source: bool,
    data_len: u3,
    #[abstract_bits(presence_from = has_source)]
    source: Option<u16>,
    #[abstract_bits(length_from = data_len)]
    data: Vec<u8>,
}

#[test]
fn test_option_presence_mismatch_true_but_none() {
    let invalid_struct = TestOption {
        has_value: true, // Says value is present
        value: None,     // But value is None
    };

    // This should fail during serialization
    let result = invalid_struct.to_abstract_bits();
    assert!(
        result.is_err(),
        "Expected serialization to fail when has_value=true but value=None"
    );
}

#[test]
fn test_option_presence_mismatch_false_but_some() {
    let invalid_struct = TestOption {
        has_value: false, // Says value is not present
        value: Some(42),  // But value is Some
    };

    // This should fail during serialization
    let result = invalid_struct.to_abstract_bits();
    assert!(
        result.is_err(),
        "Expected serialization to fail when has_value=false but value=Some"
    );
}

#[test]
fn test_option_valid_combinations() {
    // Valid: has_value=true, value=Some
    let valid1 = TestOption {
        has_value: true,
        value: Some(42),
    };
    assert!(
        valid1.to_abstract_bits().is_ok(),
        "Expected serialization to succeed when has_value=true and value=Some"
    );

    // Valid: has_value=false, value=None
    let valid2 = TestOption {
        has_value: false,
        value: None,
    };
    assert!(
        valid2.to_abstract_bits().is_ok(),
        "Expected serialization to succeed when has_value=false and value=None"
    );
}

#[test]
fn test_vec_length_mismatch_too_short() {
    let invalid_struct = TestVec {
        length: 5,           // Says there are 5 elements
        data: vec![1, 2, 3], // But only 3 elements
    };

    // This should fail during serialization
    let result = invalid_struct.to_abstract_bits();
    assert!(
        result.is_err(),
        "Expected serialization to fail when length=5 but data has 3 elements"
    );
}

#[test]
fn test_vec_length_mismatch_too_long() {
    let invalid_struct = TestVec {
        length: 2,                 // Says there are 2 elements
        data: vec![1, 2, 3, 4, 5], // But has 5 elements
    };

    // This should fail during serialization
    let result = invalid_struct.to_abstract_bits();
    assert!(
        result.is_err(),
        "Expected serialization to fail when length=2 but data has 5 elements"
    );
}

#[test]
fn test_vec_valid_length_match() {
    let valid_struct = TestVec {
        length: 3,
        data: vec![1, 2, 3],
    };

    assert!(
        valid_struct.to_abstract_bits().is_ok(),
        "Expected serialization to succeed when length matches data size"
    );
}

#[test]
fn test_vec_empty_valid() {
    let valid_struct = TestVec {
        length: 0,
        data: vec![],
    };

    assert!(
        valid_struct.to_abstract_bits().is_ok(),
        "Expected serialization to succeed for empty vec with length=0"
    );
}

#[test]
fn test_complex_struct_multiple_mismatches() {
    // Multiple validation errors at once
    let invalid_struct = ComplexStruct {
        has_source: true,       // Says source is present
        data_len: 2,            // Says data has 2 elements
        source: None,           // But source is None - ERROR 1
        data: vec![1, 2, 3, 4], // But data has 4 elements - ERROR 2
    };

    let result = invalid_struct.to_abstract_bits();
    assert!(
        result.is_err(),
        "Expected serialization to fail with multiple validation errors"
    );
}

#[test]
fn test_complex_struct_valid() {
    let valid_struct = ComplexStruct {
        has_source: true,
        data_len: 3,
        source: Some(0x1234),
        data: vec![1, 2, 3],
    };

    assert!(
        valid_struct.to_abstract_bits().is_ok(),
        "Expected serialization to succeed with valid field combinations"
    );
}

#[test]
fn test_complex_struct_valid_no_source() {
    let valid_struct = ComplexStruct {
        has_source: false,
        data_len: 1,
        source: None,
        data: vec![42],
    };

    assert!(
        valid_struct.to_abstract_bits().is_ok(),
        "Expected serialization to succeed with no source"
    );
}

#[test]
fn test_roundtrip_after_validation() {
    // Test that valid structs can roundtrip correctly
    let original = ComplexStruct {
        has_source: true,
        data_len: 2,
        source: Some(0xABCD),
        data: vec![0xFF, 0x00],
    };

    let bytes = original
        .to_abstract_bits()
        .expect("Serialization should succeed");
    let deserialized = ComplexStruct::from_abstract_bits(&bytes)
        .expect("Deserialization should succeed");

    assert_eq!(original, deserialized, "Roundtrip should preserve data");
}

#[test]
fn test_error_message_content() {
    // Test that error messages are informative
    let invalid_option = TestOption {
        has_value: true,
        value: None,
    };

    let result = invalid_option.to_abstract_bits();
    match result {
        Err(abstract_bits::ToBytesError::ValidationError(msg)) => {
            assert!(
                msg.contains("value"),
                "Error message should mention the field name"
            );
            assert!(
                msg.contains("controller is true"),
                "Error message should describe the mismatch"
            );
        }
        _ => panic!("Expected ValidationError"),
    }

    let invalid_vec = TestVec {
        length: 3,
        data: vec![1, 2, 3, 4, 5],
    };

    let result = invalid_vec.to_abstract_bits();
    match result {
        Err(abstract_bits::ToBytesError::ValidationError(msg)) => {
            assert!(
                msg.contains("data"),
                "Error message should mention the field name"
            );
            assert!(
                msg.contains("3"),
                "Error message should mention the expected length"
            );
            assert!(
                msg.contains("5"),
                "Error message should mention the actual length"
            );
        }
        _ => panic!("Expected ValidationError"),
    }
}
