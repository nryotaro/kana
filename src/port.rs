pub trait DocumentRepository {
	fn new(url: & str) -> Option<Box<Self>>
	where
		Self: Sized;
	fn close(&self);
}
