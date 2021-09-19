pub trait DocumentPort {
	fn new(url: & str) -> Option<Box<Self>>
	where
		Self: Sized;
	fn close(&self);
}
