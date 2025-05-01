#[derive(Debug)]
struct Big {
	items: Vec<u32>,
	name: String,
}
impl Big {
	fn new(name: &str) -> Self {
		Self {
			items: vec![1,2,3,4,5],
			name: name.to_string(),
		}
	}
}
impl Drop for Big {
	fn drop(&mut self) {
		println!("{} Drop", self.name);
	}
}
impl Clone for Big {
	fn clone(&self) -> Self {
		println!("{} Clone", self.name);
		Self { items: self.items.clone(), name: self.name.clone() }
	}
}
#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn check_drop() {
		let b1 = Big::new("A");
		// let b2 = Box::new(Big::new("B"));
		if Big::new("B").items.len() > 0 {

		}
		println!("Random text");

	}
	#[test]
	fn options() {
		let b1 = Big::new("A");
		let b2 = Big::new("B");
		let mut option1 = Some(b1); //b1 is moved into option1
		let mut option2 = Some(Big::new("C"));
		// match option1 {
		// 	Some(ref mut x) => {}, // ref borrows not moves
		// 	// Some(x) => {}, // moves
		// 	None => {}
		// }
		// match option2.as_mut() {
		// 	Some(x) => {}, // ref borrows not moves
		// 	// Some(x) => {}, // moves
		// 	None => {}
		// }
		println!("{:?} {:?}", option1, option2);
		// if let Some(ref mut x) = option1 {
		// 	println!("{:?}", x);
		// 	match option1 {
		// 		Some(x) => {},
		// 		None => {},
		// 	}
		// 	x.items.clear();
		// }
		if let Some(x) = option2.as_mut() {
			println!("{:?}", x);
		}
		// if let Some(x) = option2 { // moves
		// 	println!("{:?}", x);
		// }
		println!("{:?} {:?}", option1, option2);
	}
}