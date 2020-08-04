pub trait VecExt { // xml
	type Item;
	fn take_first<P:Fn(&Self::Item)->bool>(&mut self, predicate: P) -> Option<Self::Item>;
}
impl<T> VecExt for Vec<T> {
	type Item = T;
	fn take_first<P:Fn(&Self::Item)->bool>(&mut self, predicate: P) -> Option<Self::Item> {
		Some(self.remove(self.iter().position(predicate)?))
	}
}
