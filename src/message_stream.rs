use action::*;

#[derive(Debug, Clone)]
pub struct MessageStream {
	recieved: Vec<u8>
}

impl MessageStream {
	pub fn new() -> Self {
		return MessageStream {
			recieved: Vec::new()
		}
	}

	pub fn recv(&mut self, data: &[u8]) {
		self.recieved.extend(data);
	}


	pub fn extract_requests(&mut self) -> Vec<Box<Request>> {
		return vec![Box::new(ListRequest::new())];
	}
}