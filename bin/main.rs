use name_sphere::{result::Result, server::Server};

fn main() -> Result<()> {
	let serv = Server::new("0:8080")?;

	serv.listen()
}
