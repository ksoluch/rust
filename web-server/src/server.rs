use std::{
	io::{
		BufRead, BufReader, Write
	}, 
	net::{
		TcpListener,
		TcpStream,
	}
};

struct Server {
	address: &'static str
}

impl Server {
	pub fn new(address: &'static str) -> Self {
		Self {
			address
		}
	}

	pub fn run(&self) -> std::io::Result<()> {
		let listener = TcpListener::bind(self.address)?;

		for connection in listener.incoming() {
			let Ok(stream) = connection else {
				eprintln!("connection failure {:?}", connection);
				continue;
			};
			std::thread::spawn(move || {
				let me = std::thread::current().id();
				println!("new thread {:?}", me);
				Server::process(stream);
			});
		}
		Ok(())
	}

	fn process(mut stream: TcpStream) {
		let reader = BufReader::new(&stream);
		let content : Vec<_> = reader
			.lines()
			.filter_map(|e| e.ok())
			.take_while(|e| 0 != e.len())
			.collect();
		let page;
		if content.into_iter().next().unwrap() == "GET / HTTP/1.1" {
			page = Server::generate_page("./data/hello.html", "HTTP/1.1 200 OK");
		} else {
			page = Server::generate_page("./data/404.html", "HTTP/1.1 404 NOT FOUND");
		}
		stream.write_all(page.as_bytes()).unwrap();
	}

	fn generate_page(file: &str, status_line: &str) -> String {
		let contents = std::fs::read_to_string(file).unwrap();
		let length = contents.len();
		format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
	}

}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn run_server() -> std::io::Result<()> {
		let server = Server::new("127.0.0.1:7878");
		server.run()?;
		Ok(())
	}
}