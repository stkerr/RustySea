use rusty_sea::*;

#[test]
fn basic_negation_test_positive() {
	if let Ok(v) = create_bigint_from_string("2") {
		let negative = -v;
		assert!(negative.negative == true);
	} else {
		panic!("Failed to convert from string!");
	}
}

#[test]
fn basic_negation_test_negative() {
	if let Ok(v) = create_bigint_from_string("-2") {
		let negative = -v;
		assert!(negative.negative == false);
	} else {
		panic!("Failed to convert from string!");
	}
}