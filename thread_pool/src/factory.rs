use core::num;
use std::{collections::VecDeque, sync::{Arc, Condvar}, thread::JoinHandle, time::Duration, Mutex};

struct Task {
	delay: Duration,
}

struct Output {
	task: Task,
	message: String,
}

struct Factory<T,U> {
	workers: Vec<JoinHandle<()>>,
	tasks: Arc<(Mutex<VecDeque<T>>, Condvar)>,
	outputs: Arc<Mutex<VecDeque<U>>>,
}

impl<T,U> Factory<T,U> {
	fn new(number_of_workers: usize) -> Self {
		let workers = Vec::with_capacity(number_of_workers);
		let tasks = Arc::new((Mutex::new(VecDeque::<T>::new()), Condvar::new()));
		let outputs = Arc::new(Mutex::new(VecDeque::<U>::new()));
		Self {
			workers,
			tasks,
			outputs,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn create_factory() {
		let number_of_workers = 10;
		let factory = Factory::<Task, Output>::new(number_of_workers);
		for n in 0..50 {
			// generate task with a random sleep value to simulate different loads
			let task = generate_task();
			factory.add_work(task);
		}
		// Iterates over result, if next is called when the colleciton is empty but not all tasks were processed it ups the calling thread to sleep.
		let iterator = factory.results();
		let out = iterator.next();
		let out = iterator.next();
		let out = iterator.next();
	}
}