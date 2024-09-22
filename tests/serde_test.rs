// this is written to test the functionality of the serde crate working under mingw cross compiller

#[test]
fn serde_test() {
    use serde_json::Value;

    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = std::fs::read_to_string("./tests/sample_data/john.json")
        .expect("Failed to retreive sample data");

    let data: Value = serde_json::from_str(&data).unwrap();

    assert_eq!("John Doe", data["name"]);
    assert_eq!(43, data["age"]);
    assert_eq!("+44 1234567", data["phones"][0]);
    assert_eq!("+44 2345678", data["phones"][1]);
}
