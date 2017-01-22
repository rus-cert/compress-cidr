use std::fmt;

pub struct WriteLinesIter<I: Iterator+Clone> {
	iter: I,
}

impl<I: Iterator+Clone> WriteLinesIter<I> {
	pub fn new(iter: I) -> Self {
		WriteLinesIter{iter: iter}
	}
}

impl<T, I: Iterator<Item=T>+Clone, F: IntoIterator<Item=T,IntoIter=I>> From<F> for WriteLinesIter<I> {
	fn from(f: F) -> Self {
		WriteLinesIter::<I>::new(f.into_iter())
	}
}

impl<T: fmt::Display, I: Iterator<Item=T>+Clone> fmt::Display for WriteLinesIter<I> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for x in self.iter.clone() {
			try!(writeln!(f, "{}", x));
		}
		Ok(())
	}
}

impl<T: fmt::Debug, I: Iterator<Item=T>+Clone> fmt::Debug for WriteLinesIter<I> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for x in self.iter.clone() {
			try!(writeln!(f, "{:?}", x));
		}
		Ok(())
	}
}
