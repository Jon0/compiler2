extern crate nix;

pub mod AccessMap;

pub struct ConnectionMap {
	name: String
}

impl ConnectionMap {
	pub fn new() -> Self {
		ConnectionMap { name: "Test".to_string() }
	}
}
