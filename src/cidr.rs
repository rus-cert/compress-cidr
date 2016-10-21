use super::bitstrings::{FixedBitString,BitString};
use std::net::{Ipv4Addr,Ipv6Addr,AddrParseError};
use std::cmp::{min,Ordering};
use std::fmt;
use std::str::FromStr;
use std::result::Result;
use std::num::ParseIntError;

pub trait IpAddress: FixedBitString+Clone+Eq+FromStr<Err=AddrParseError>+fmt::Display+fmt::Debug {
}
impl IpAddress for Ipv4Addr {
}
impl IpAddress for Ipv6Addr {
}

pub type Ipv6Cidr = IpCidr<Ipv6Addr>;
pub type Ipv4Cidr = IpCidr<Ipv4Addr>;


#[derive(PartialEq,Clone,Debug)]
pub enum NetworkParseError {
	AddrParseError(AddrParseError),
	NetworkLengthParseError(ParseIntError),
	NetworkLengthTooLongError,
}

#[derive(Clone,PartialEq,Eq,Hash)]
pub struct IpCidr<A: IpAddress> {
	pub address: A,
	pub network_length : u8,
}

impl<A: IpAddress> BitString for IpCidr<A> {
	fn get(&self, ndx: usize) -> bool {
		self.address.get(ndx)
	}

	fn set(&mut self, ndx: usize, bit: bool) {
		assert!(ndx < self.len());
		self.address.set(ndx, bit);
	}

	fn flip(&mut self, ndx: usize) {
		assert!(ndx < self.len());
		self.address.flip(ndx);
	}

	fn len(&self) -> usize {
		self.network_length as usize
	}

	fn clip(&mut self, len: usize) {
		self.address.zerofrom(len);
		self.network_length = min(self.network_length as usize, len) as u8;
	}

	fn append(&mut self, bit: bool) {
		self.address.set(self.network_length as usize, bit);
		self.network_length += 1;
	}

	fn null() -> Self {
		IpCidr{
			address: A::null(),
			network_length: 0,
		}
	}
}

impl<A: IpAddress> fmt::Debug for IpCidr<A> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}/{}", self.address, self.network_length)
	}
}

impl<A: IpAddress> fmt::Display for IpCidr<A> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.address, self.network_length)
	}
}

impl<A: IpAddress> PartialOrd<IpCidr<A>> for IpCidr<A> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.bitstring_partial_cmp(other)
	}
}

impl<A: IpAddress> FromStr for IpCidr<A> {
	type Err = NetworkParseError;
	fn from_str(s: &str) -> Result<IpCidr<A>, NetworkParseError> {
		match s.rfind('/') {
			None => match A::from_str(s) {
				Ok(addr) => Ok(IpCidr{address: addr, network_length: 128}),
				Err(e) => Err(NetworkParseError::AddrParseError(e))
			},
			Some(pos) => {
				let len = match u8::from_str(&s[pos+1..]) {
					Ok(len) if len <= 128 => len,
					Ok(_) => return Err(NetworkParseError::NetworkLengthTooLongError),
					Err(e) => return Err(NetworkParseError::NetworkLengthParseError(e)),
				};
				match A::from_str(&s[0..pos]) {
					Ok(addr) => Ok(IpCidr{address: addr, network_length: len}),
					Err(e) => Err(NetworkParseError::AddrParseError(e))
				}
			}
		}
	}
}
