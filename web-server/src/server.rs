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
			self.process(stream);
		}
		Ok(())
	}

	fn process(&self, mut stream: TcpStream) {
		let reader = BufReader::new(&stream);
		let content : Vec<_> = reader
			.lines()
			.filter_map(|e| e.ok())
			.take_while(|e| 0 != e.len())
			.collect();
		println!("received: {:?}", content);
		let response = "HTTP/1.1 200 OK\r\n\r\n";
		stream.write_all(response.as_bytes()).unwrap();
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