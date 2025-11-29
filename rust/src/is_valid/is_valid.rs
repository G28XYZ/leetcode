fn is_valid(s: String) -> bool {
	let left_closed_chars = ["(", "[", "{"];
	let right_closed_chars = [")", "]", "}"];

	let mut stack = "".to_string();

	for (_, char) in s.chars().enumerate() {
		let left = left_closed_chars.iter().find(|&x| x.chars().next().unwrap() == char);
		let right = right_closed_chars.iter().find(|&x| x.chars().next().unwrap() == char);

		let sym = &char.to_string();
		
		if left != None {
			stack += &sym;
		}
		if right != None {
			if stack != "" {
				let last = stack.chars().last().unwrap();
				let to_int = sym.chars().next().unwrap() as i32;
				if last as i32 == to_int - 1 || last as i32 == to_int - 2 {
					stack.pop();
				} else {
					stack = "err".to_string();
				}
			} else {
				stack = "err".to_string();
			}
		}
	}

	stack.is_empty()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn is_valid_test_1() {
		let res = is_valid(String::from("({})"));

		assert_eq!(res, true);
	}
}