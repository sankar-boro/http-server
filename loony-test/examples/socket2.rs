use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write, Error};
use std::str::from_utf8;

const SERVER: Token = Token(0);
#[derive(Debug)]
struct Server {
	poll: Poll,
	events: Events,
	server: TcpListener
}

impl Server {
	fn new() -> Result<Server, Error> {
		let addr = "127.0.0.1:9000".parse().unwrap();
		Ok(Server{
			poll: Poll::new()?,
			events:Events::with_capacity(128),
			server: TcpListener::bind(addr)?,
		})
	}

	pub fn register_listener(mut self) -> Self {
		self.poll.registry().register(&mut self.server, SERVER, Interest::READABLE).unwrap();
		self
	}
}

fn main() -> io::Result<()> {
	let server = Server::new()?;
	let mut server = server.register_listener();
	server.poll.poll(&mut server.events, None)?;
	loop {
		for event in server.events.iter() { 
			println!("Event: {:?}", event);
			match event.token() {
				SERVER => loop {
					let (mut connection, address) = match server.server.accept() {
              Ok((connection, address)) => (connection, address),
					    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // If we get a `WouldBlock` error we know our
                            // listener has no more incoming connections queued,
                            // so we can return to polling and wait for some
                            // more.
                            break;
                        }
                        Err(e) => {
                            // If it was any other kind of error, something went
                            // wrong and we terminate with an error.
                            return Err(e);
                        }
					};

                    println!("Accepted connection from: {}", address);
				},
				token => {}
			}
		}
	}
	Ok(())
}
