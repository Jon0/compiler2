use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

pub trait Resource {
	fn test(&self); 
}


pub struct ConstantResource {

}


impl ConstantResource {
	pub fn new() -> Self {
		return Self {}
	}
}


impl Resource for ConstantResource {
	fn test(&self) {

	}
}


/// Global collection of resources
#[derive(Clone)]
pub struct ResourceSet {
	r: Vec<Arc<Resource + Send + Sync>>
	//resources: Vec<Arc<Mutex<Resource + Send>>>
}


impl ResourceSet {
	pub fn new() -> Self {
		return ResourceSet {
			r: Vec::new() //Arc::new(Mutex::new(Vec::new()))
			//resources: Vec::new()
		}
	}


	pub fn add(&mut self, i: Arc<Resource + Send + Sync>) {
		self.r.push(i);
	}


	pub fn debug(&self) -> String {
		return "Ok\n".to_string();
	}
}