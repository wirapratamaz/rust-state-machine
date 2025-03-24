fn main() {

	// safe match operation
	let a: u8 = 255; // hmm this is not a valid u8 value (so dangerous)

	// safe match operation
	let safe_add = a.checked_add(1);
	println!("safe_add: {:?}", safe_add); // will be None (overflow)

	let safe_sub = a.checked_sub(10);
	println!("safe_sub: {:?}", safe_sub); // this is a valid u8 value

	// option handling 
	match safe_add {
		Some(result) => println!("Addition successful: {}", result),
		None => println!("Addition overflowed"),
	}

	// usigng unwrap_or for default values
	let safe_result = a.checked_add(1).unwrap_or(0);
	println!("safe_result: {}", safe_result);

	// function that returns a Result 
	fn divide(a: u32, b: u32) -> Result<u32, &'static str> {
		if b == 0 {
			Err("Division by zero")
		} else {
			Ok(a / b)
		}
	}

	// using match
	match divide(10, 2) {
		Ok(result) => println!("Division successful: {}", result), //5
		Err(error) => println!("Error: {}", error),
	}

	match divide(10, 0) {
		Ok(result) => println!("Division successful: {}", result),
		Err(error) => println!("Error: {}", error), // Division by zero
	}

	// using ? operator with results
	fn complex_calculation(a: u32, b: u32, c: u32) -> Result<u32, &'static str> {
		let division = divide(a, b)?;
		Ok(division * 2)
	}

	println!("Complex calculation: {:?}", complex_calculation(10, 2, 3)); // Ok(10)
	// println!("Complex calculation: {:?}, complex_calculation(10, 0, 3)"; // Err("Division by zero"))
	
	println!("Hello, world!");
}
