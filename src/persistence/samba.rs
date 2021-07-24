mod wrapper;
use crate::port::DocumentRepository;

pub struct SambaClient {}

impl DocumentRepository for SambaClient {
	fn new(url: &'static str) -> SambaClient {
		SambaClient{}
	}
}

pub struct Doge {}

impl DocumentRepository for Doge {
	fn new(url: &'static str) -> Doge {
		Doge{}
	}
}

