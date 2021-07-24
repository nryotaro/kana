pub trait DocumentRepository {
	fn new(url: &'static str) -> Self;
}
