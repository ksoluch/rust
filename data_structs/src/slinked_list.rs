struct Node<T> {
	next: Option<Box<Node<T>>>,
	item: T,
}
struct SLinkedList<T> {
	head: Option<Box<Node<T>>>,
}

impl<T> SLinkedList<T> {
	fn new() -> Self {
		Self { head: None} 
	}
	fn insert_back(&mut self, item: T) {

	}
	fn insert_front(&mut self, item: T) {

	}
	fn contains(&self, item: &T) -> bool {
		false
	}
	fn remove(&mut self, item: &T) -> Option<T> {
		None
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn insert_back() {
		let mut list = SLinkedList::<String>::new();
		list.insert_back(String::from("Orange"));
		assert!(list.head.is_some());
		assert_eq!(list.head.as_ref().unwrap().item, String::from("Orange"));
	}
}