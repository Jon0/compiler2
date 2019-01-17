use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use uuid::Uuid;


pub trait Resource {
	fn get_uuid(&self) -> Uuid;
	fn get_name(&self) -> &str;
}


///
/// the type should implement Serailise + Deserialise
///
pub struct ConstantResource<T> {
	id: Uuid,
	name: String,
	data: T
}


impl<T> ConstantResource<T> {
	pub fn new(data: T) -> Self {
		return Self {
			id: Uuid::new_v4(),
			name: "Test".to_string(),
			data: data
		}
	}
}


impl<T> Resource for ConstantResource<T> {
	fn get_uuid(&self) -> Uuid {
		return self.id;
	}

	fn get_name(&self) -> &str {
		return self.name.as_str();
	}
}


/// Global collection of resources
#[derive(Clone)]
pub struct ResourceSet {
	resources: Vec<Arc<Resource + Send + Sync>>
}


impl ResourceSet {
	pub fn new() -> Self {
		return ResourceSet {
			resources: Vec::new()
		}
	}


	pub fn add(&mut self, i: Arc<Resource + Send + Sync>) {
		self.resources.push(i);
	}


	pub fn get_resources(&self) -> &[Arc<Resource + Send + Sync>] {
		return &self.resources;
	}
}