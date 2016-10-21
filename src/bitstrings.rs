use std::cmp::{min,Ordering};
use std::mem::size_of;
use std::net::{Ipv4Addr,Ipv6Addr};
use std::ops::{Shl,BitAnd,BitOr,BitXor,Not,Sub};
use std::marker::PhantomData;
use num_traits::{One,Zero};

pub struct BigEndianBits<E> {
	phantom: PhantomData<E>,
}

impl<E: Shl<usize, Output=E>+BitAnd<Output=E>+BitOr<Output=E>+BitXor<Output=E>+Not<Output=E>+Sub<Output=E>+One+Zero+Eq+Copy> BigEndianBits<E> {
	pub fn elembits() -> usize {
		8*size_of::<E>()
	}

	pub fn from_bit(bit: bool) -> E {
		if bit { E::one() } else { E::zero() }
	}

	pub fn mask(ndx: usize, bit: bool) -> E {
		let bits = Self::elembits();
		let bit_ndx = bits - 1 - (ndx % bits);
		Self::from_bit(bit) << bit_ndx
	}

	pub fn get(slice: &[E], ndx: usize) -> bool {
		let mask = Self::mask(ndx, true);
		let slice_ndx = ndx / Self::elembits();
		E::zero() != (slice[slice_ndx] & mask)
	}

	pub fn set(slice: &mut [E], ndx: usize, bit: bool) {
		let mask = Self::mask(ndx, true);
		let value = Self::mask(ndx, bit);
		let slice_ndx = ndx / Self::elembits();
		slice[slice_ndx] = (slice[slice_ndx] & !mask) | value;
	}

	pub fn flip(slice: &mut [E], ndx: usize) {
		let mask = Self::mask(ndx, true);
		let slice_ndx = ndx / Self::elembits();
		slice[slice_ndx] = slice[slice_ndx] ^ mask;
	}

	pub fn zerofrom(slice: &mut [E], ndx: usize) {
		let slice_ndx = ndx / Self::elembits();
		if 0 == ndx % Self::elembits() {
			for i in slice_ndx..slice.len() {
				slice[i] = E::zero();
			}
		}
		else if slice_ndx < slice.len() {
			let mask = Self::mask(ndx - 1, true) - E::one();
			slice[slice_ndx] = slice[slice_ndx] & !mask;
			for i in slice_ndx+1..slice.len() {
				slice[i] = E::zero();
			}
		}
	}
}

pub trait FixedBitString {
	fn get(&self, ndx: usize) -> bool;
	fn set(&mut self, ndx: usize, bit: bool);
	fn flip(&mut self, ndx: usize);
	fn zerofrom(&mut self, ndx: usize);
	fn null() -> Self;
}

impl FixedBitString for Ipv4Addr {
	fn get(&self, ndx: usize) -> bool {
		BigEndianBits::<u8>::get(&self.octets(), ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		let mut o = self.octets();
		BigEndianBits::<u8>::set(&mut o, ndx, bit);
		*self = Ipv4Addr::from(o);
	}

	fn flip(&mut self, ndx: usize) {
		let mut o = self.octets();
		BigEndianBits::<u8>::flip(&mut o, ndx);
		*self = Ipv4Addr::from(o);
	}

	fn zerofrom(&mut self, ndx: usize) {
		let mut o = self.octets();
		BigEndianBits::<u8>::zerofrom(&mut o, ndx);
		*self = Ipv4Addr::from(o);
	}

	fn null() -> Self {
		Ipv4Addr::new(0, 0, 0, 0)
	}
}

impl FixedBitString for Ipv6Addr {
	fn get(&self, ndx: usize) -> bool {
		BigEndianBits::<u8>::get(&self.octets(), ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		let mut o = self.octets();
		BigEndianBits::<u8>::set(&mut o, ndx, bit);
		*self = Ipv6Addr::from(o);
	}

	fn flip(&mut self, ndx: usize) {
		let mut o = self.octets();
		BigEndianBits::<u8>::flip(&mut o, ndx);
		*self = Ipv6Addr::from(o);
	}

	fn zerofrom(&mut self, ndx: usize) {
		let mut o = self.octets();
		BigEndianBits::<u8>::zerofrom(&mut o, ndx);
		*self = Ipv6Addr::from(o);
	}

	fn null() -> Self {
		Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)
	}
}

pub trait BitString: Sized+Clone {
	fn get(&self, ndx: usize) -> bool;
	fn set(&mut self, ndx: usize, bit: bool);
	fn flip(&mut self, ndx: usize);
	fn len(&self) -> usize;
	fn clip(&mut self, len: usize);
	fn append(&mut self, bit: bool);
	fn null() -> Self;

	fn shared_prefix_len(&self, other: &Self) -> usize {
		let max_len = min(self.len(), other.len());
		for i in 0..max_len {
			if self.get(i) != other.get(i) {
				return i
			}
		}
		max_len
	}

	fn shared_prefix(&self, other: &Self) -> Self {
		let mut a = self.clone();
		a.clip(self.shared_prefix_len(other));
		a
	}

	// a < b iff a != b and a is a prefix of b
	fn bitstring_partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let spl = self.shared_prefix_len(other);
		if spl == self.len() {
			if spl == other.len() {
				Some(Ordering::Equal)
			} else {
				Some(Ordering::Less)
			}
		} else if spl == other.len() {
			Some(Ordering::Greater)
		} else {
			None
		}
	}
}

#[derive(Clone,Debug)]
pub struct BitWordString<W: FixedBitString+Sized+Clone> {
	pub bitwords : W,
	pub len : usize,
}

impl<W: FixedBitString+Clone> BitString for BitWordString<W> {
	fn get(&self, ndx: usize) -> bool {
		self.bitwords.get(ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		assert!(ndx < self.len);
		self.bitwords.set(ndx, bit);
	}

	fn flip(&mut self, ndx: usize) {
		assert!(ndx < self.len);
		self.bitwords.flip(ndx);
	}

	fn len(&self) -> usize {
		self.len
	}

	fn clip(&mut self, len: usize) {
		self.bitwords.zerofrom(len);
		self.len = min(self.len, len);
	}

	fn append(&mut self, bit: bool) {
		self.bitwords.set(self.len, bit);
		self.len += 1;
	}

	fn null() -> Self {
		BitWordString{
			bitwords: W::null(),
			len: 0,
		}
	}
}
