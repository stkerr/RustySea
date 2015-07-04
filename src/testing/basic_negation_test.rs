use rusty_sea::bigint::utilities::*;

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

#[test]
fn basic_negation_large_integer() {
	assert!((-create_bigint_from_string("-fff33ff33fff3fddd").unwrap()).compare(&create_bigint_from_string("fff33ff33fff3fddd").unwrap()) == 0);
}

#[test]
fn basic_negation_large_integer_backward() {
	assert!((-create_bigint_from_string("fff33ff33fff3fddd").unwrap()).compare(&create_bigint_from_string("-fff33ff33fff3fddd").unwrap()) == 0);
}
