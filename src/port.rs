pub trait DocumentRepository {
	fn new(url: &'static str) -> Option<Box<Self>>
	where
		Self: Sized;
	fn close(&self);
}
