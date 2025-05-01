#[derive(Debug)]
struct Node<T> {
	next: Option<Box<Node<T>>>,
	item: T,
}
#[derive(Debug)]
struct SLinkedList<T> {
	head: Option<Box<Node<T>>>,
}
impl<T: PartialEq> SLinkedList<T> {
	fn new() -> Self {
		Self { head: None} 
	}
	fn insert_back(&mut self, item: T) {
		if self.head.is_none() {
			self.head = Some(Box::new(Node { next: None, item }));
			return;
		}
		let mut it = self.head.as_mut();
		while it.is_some() {
			let inside = it.unwrap();
			if inside.next.is_none() {
				inside.next = Some(Box::new(Node { next: None, item }));
				break;
			}
			it = inside.next.as_mut();
		}
	}
	fn insert_front(&mut self, item: T) {
		if self.head.is_none() {
			self.head = Some(Box::new(Node { next: None, item }));
		} else {
			self.head = Some(Box::new(Node { next: self.head.take(), item }));
		}
	}
	fn contains(&self, item: &T) -> bool {
		let mut it = self.head.as_ref();
		while let Some(node) = it {
			if node.item == *item {
				return true;
			}
			it = node.next.as_ref();
		}
		false
	}
	fn remove(&mut self, item: &T) -> Option<T> {
		let mut head = self.head.as_mut();
		//special case: head matches
		if let Some(head)	= head {
			if head.item == *item {
				let removed_head = self.head.take();
				self.head = removed_head.unwrap().next;
			}
		}
		// find element whos next contains the item
		let mut it = self.head.as_mut();
		while let Some(parent) = it {
			if let Some(child ) = parent.next.as_mut() {
				if child.item == *item {
					let to_be_removed = parent.next.take().unwrap();
					parent.next = to_be_removed.next;
					return Some(to_be_removed.item);
				}
			}	
			it = parent.next.as_mut();
		}
		None
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn remove() {
		let mut list = SLinkedList::<String>::new();
		list.insert_front(String::from("Berry"));
		list.insert_front(String::from("Apple"));
		list.insert_front(String::from("Orange"));
		println!("{:?}", list);
		let to_remove = String::from("Apple");
		let removed = list.remove(&to_remove);
		println!("{:?}", list);
		assert_eq!(removed, Some(to_remove));
		assert_eq!(list.head.as_ref().unwrap().item, String::from("Orange"));
		assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().item, String::from("Berry"));
		let lookup = String::from("Berry");
		assert!(list.contains(&lookup));
	}
	#[test]
	fn insert_front() {
		let mut list = SLinkedList::<String>::new();
		list.insert_front(String::from("Berry"));
		list.insert_front(String::from("Apple"));
		list.insert_front(String::from("Orange"));
		println!("{:?}", list);
		assert_eq!(list.head.as_ref().unwrap().item, String::from("Orange"));
		assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().item, String::from("Apple"));
		let lookup = String::from("Berry");
		assert!(list.contains(&lookup));
	}
	#[test]
	fn insert_back() {
		let mut list = SLinkedList::<String>::new();
		list.insert_back(String::from("Orange"));
		list.insert_back(String::from("Apple"));
		list.insert_back(String::from("Berry"));
		println!("{:?}", list);
		assert_eq!(list.head.as_ref().unwrap().item, String::from("Orange"));
		assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().item, String::from("Apple"));
		let lookup = String::from("Berry");
		assert!(list.contains(&lookup));
	}
}