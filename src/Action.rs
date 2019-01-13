use std::sync::Arc;

use Resource::*;

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
	fn apply(&self, resources: Arc<ResourceSet>) -> String {
		return resources.debug();
	}
}