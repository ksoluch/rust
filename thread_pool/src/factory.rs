use core::num;
use std::{collections::VecDeque, sync::{Arc, Condvar, Mutex}};

struct Factory<T> {
	workers: Vec<std::thread::JoinHandle<()>>,
	tasks: Arc<(Mutex<VecDeque<T>>,Condvar)>,
}

impl<T: FnOnce() + Send + 'static> Factory<T> {
	fn new(number_of_threads: usize) -> Self {
		let mut workers = Vec::<std::thread::JoinHandle<()>>::with_capacity(number_of_threads);
		let tasks = Arc::new((Mutex::new(VecDeque::<T>::new()), Condvar::new()));
		for i in 0..number_of_threads {
			let internal = tasks.clone();
			workers.push(std::thread::spawn(move || { 
				loop {
					println!("Loop of {:?}", std::thread::current()); 
					let mut queue = internal.0.lock().unwrap();
					while queue.is_empty() {
						queue = internal.1.wait(queue).unwrap();
					}
					let job = queue.pop_front().unwrap();
					job();
				}
			}))
		}
		Self {
			workers,
			tasks,
		}
	}
	fn add(&mut self, job: T) {
		let mut queue = self.tasks.0.lock().unwrap();
		queue.push_back(job);
		self.tasks.1.notify_one();
	}
}

#[cfg(test)]
mod test {
	use std::time::Duration;

use super::Factory;
	#[test]
	fn basic_schedule() {
		let number_of_workers = 5;
		let mut factory= Factory::<Box<dyn FnOnce() + Send>>::new(number_of_workers);
		std::thread::sleep(Duration::from_secs(1));
		for _ in 0..4 {
			factory.add(Box::new(|| {
				println!("Job Loop of {:?}", std::thread::current()); 
			}));
		}
		// factory.start();
		// for _ in 0..4 {
		// 	factory.add(|| { println!("Job!"); });
		// 	factory.add(|| { println!("Job!"); });
		// 	factory.add(|| { println!("Job!"); });
		// 	factory.add(|| { println!("Job!"); });
		//}
	}
}