use std::sync::Arc;

use resource::*;

pub trait Request {
	fn apply(&self, resources: Arc<ResourceSet>) -> String;
}

pub struct ListRequest {

}

impl ListRequest {
	pub fn new() -> Self {
		return ListRequest {

		}
	}
}

impl Request for ListRequest {
	fn apply(&self, set: Arc<ResourceSet>) -> String {

		let mut s = String::new();
		for r in set.get_resources() {
			s.push_str(r.get_uuid().to_string().as_str());
			s.push_str(r.get_name());
		}
		return s;
	}
}