use std::{
	io::{
		BufReader,
		BufRead,
	}, 
	net::{
		TcpListener,
		TcpStream,
	}
};

pub fn connections() {
	let Ok(listener) = TcpListener::bind("127.0.0.1:7878") else {
		eprintln!("could not bind");
		return;
	};

	for c in listener.incoming() {
		let Ok(stream) = c else {
			eprintln!("{:?}", c);
			continue;
		};
		process(stream);
	}
}

fn process(stream: TcpStream) {
	let reader = BufReader::new(stream);
	let c : Vec<String> = reader
		.lines()
		.map(|e| e.unwrap())
		.take_while(|e| 0 != e.len())
		.collect();
	println!("{:?}", c);
}



