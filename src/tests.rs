use cidr::Ipv4Cidr;
use std::str::FromStr;
use radixset::RadixSet;
use radixset::def::Definition;

use write_lines::WriteLinesIter;

use std::fmt;

fn format_lines<T, I, F>(f: F) -> String
where
	T: fmt::Display,
	I: Iterator<Item=T>+Clone,
	F: IntoIterator<Item=T,IntoIter=I>
{
	format!("{}", WriteLinesIter::from(f))
}

// tests for compress

#[test]
fn compress_ipv4_empty() {
	let rs : RadixSet<Ipv4Cidr> = Default::default();

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
		)
	);
}

#[test]
fn compress_ipv4_any() {
	let addr = Ipv4Cidr::from_str("0.0.0.0/0").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/0\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
		)
	);
}

#[test]
fn compress_ipv4_single_firstbit_zero() {
	let addr = Ipv4Cidr::from_str("0.0.0.0/1").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/1\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 128.0.0.0/1\n",
		)
	);
}

#[test]
fn compress_ipv4_single_firstbit_one() {
	let addr = Ipv4Cidr::from_str("128.0.0.0/1").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 128.0.0.0/1\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/1\n",
		)
	);
}

#[test]
fn compress_ipv4_single_lastbit_zero() {
	let addr = Ipv4Cidr::from_str("0.0.0.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_single_lastbit_one() {
	let addr = Ipv4Cidr::from_str("0.0.0.1/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.1/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.1/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1a() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/1").unwrap();
	let addr2 = Ipv4Cidr::from_str("128.0.0.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/1\n",
			"include 128.0.0.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 128.0.0.0/1\n",
			"exclude 128.0.0.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1b() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/2").unwrap();
	let addr2 = Ipv4Cidr::from_str("64.0.0.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/2\n",
			"include 64.0.0.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 64.0.0.0/2\n",
			"exclude 64.0.0.0/32\n",
			"include 128.0.0.0/1\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1c() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/3").unwrap();
	let addr2 = Ipv4Cidr::from_str("32.0.0.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/3\n",
			"include 32.0.0.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.0/3\n",
			"exclude 32.0.0.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1d() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/9").unwrap();
	let addr2 = Ipv4Cidr::from_str("0.128.0.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/9\n",
			"include 0.128.0.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.0/9\n",
			"exclude 0.128.0.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1a_flipped() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/32").unwrap();
	let addr2 = Ipv4Cidr::from_str("128.0.0.0/1").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/32\n",
			"include 128.0.0.0/1\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/1\n",
			"exclude 0.0.0.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1b_flipped_1() {
	let addr1 = Ipv4Cidr::from_str("128.0.0.0/2").unwrap();
	let addr2 = Ipv4Cidr::from_str("192.0.0.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 128.0.0.0/2\n",
			"include 192.0.0.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/1\n",
			"include 192.0.0.0/2\n",
			"exclude 192.0.0.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1b_flipped_2() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/32").unwrap();
	let addr2 = Ipv4Cidr::from_str("64.0.0.0/2").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/32\n",
			"include 64.0.0.0/2\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/2\n",
			"exclude 0.0.0.0/32\n",
			"include 128.0.0.0/1\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1c_flipped_1() {
	let addr1 = Ipv4Cidr::from_str("128.0.0.0/3").unwrap();
	let addr2 = Ipv4Cidr::from_str("160.0.0.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 128.0.0.0/3\n",
			"include 160.0.0.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 128.0.0.0/3\n",
			"exclude 160.0.0.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1c_flipped_2() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/32").unwrap();
	let addr2 = Ipv4Cidr::from_str("32.0.0.0/3").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/32\n",
			"include 32.0.0.0/3\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.0/32\n",
			"exclude 32.0.0.0/3\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1d_flipped_1() {
	let addr1 = Ipv4Cidr::from_str("128.0.0.0/9").unwrap();
	let addr2 = Ipv4Cidr::from_str("128.128.0.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 128.0.0.0/9\n",
			"include 128.128.0.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 128.0.0.0/9\n",
			"exclude 128.128.0.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_1d_flipped_2() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/32").unwrap();
	let addr2 = Ipv4Cidr::from_str("0.128.0.0/9").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/32\n",
			"include 0.128.0.0/9\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.0/32\n",
			"exclude 0.128.0.0/9\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_2a() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/1").unwrap();
	let addr2 = Ipv4Cidr::from_str("128.0.0.0/32").unwrap();
	let addr3 = Ipv4Cidr::from_str("128.0.1.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);
	rs.insert(&addr3);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/1\n",
			"include 128.0.0.0/32\n",
			"include 128.0.1.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 128.0.0.0/1\n",
			"exclude 128.0.0.0/32\n",
			"exclude 128.0.1.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_2b() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/2").unwrap();
	let addr2 = Ipv4Cidr::from_str("64.0.0.0/32").unwrap();
	let addr3 = Ipv4Cidr::from_str("64.0.1.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);
	rs.insert(&addr3);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/2\n",
			"include 64.0.0.0/32\n",
			"include 64.0.1.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 64.0.0.0/2\n",
			"exclude 64.0.0.0/32\n",
			"exclude 64.0.1.0/32\n",
			"include 128.0.0.0/1\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_2c() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/3").unwrap();
	let addr2 = Ipv4Cidr::from_str("32.0.0.0/32").unwrap();
	let addr3 = Ipv4Cidr::from_str("32.0.1.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);
	rs.insert(&addr3);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/3\n",
			"include 32.0.0.0/32\n",
			"include 32.0.1.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.0/3\n",
			"exclude 32.0.0.0/32\n",
			"exclude 32.0.1.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_2d() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/9").unwrap();
	let addr2 = Ipv4Cidr::from_str("0.128.0.0/32").unwrap();
	let addr3 = Ipv4Cidr::from_str("0.128.1.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);
	rs.insert(&addr3);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/9\n",
			"include 0.128.0.0/32\n",
			"include 0.128.1.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.0/9\n",
			"exclude 0.128.0.0/32\n",
			"exclude 0.128.1.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_2a_flipped() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/32").unwrap();
	let addr2 = Ipv4Cidr::from_str("0.0.1.0/32").unwrap();
	let addr3 = Ipv4Cidr::from_str("128.0.0.0/1").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);
	rs.insert(&addr3);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/32\n",
			"include 0.0.1.0/32\n",
			"include 128.0.0.0/1\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/1\n",
			"exclude 0.0.0.0/32\n",
			"exclude 0.0.1.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_2b_flipped() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/32").unwrap();
	let addr2 = Ipv4Cidr::from_str("0.0.1.0/32").unwrap();
	let addr3 = Ipv4Cidr::from_str("64.0.0.0/2").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);
	rs.insert(&addr3);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/32\n",
			"include 0.0.1.0/32\n",
			"include 64.0.0.0/2\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/2\n",
			"exclude 0.0.0.0/32\n",
			"exclude 0.0.1.0/32\n",
			"include 128.0.0.0/1\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_2c_flipped() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/32").unwrap();
	let addr2 = Ipv4Cidr::from_str("0.0.1.0/32").unwrap();
	let addr3 = Ipv4Cidr::from_str("32.0.0.0/3").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);
	rs.insert(&addr3);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/32\n",
			"include 0.0.1.0/32\n",
			"include 32.0.0.0/3\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.0/32\n",
			"exclude 0.0.1.0/32\n",
			"exclude 32.0.0.0/3\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_2d_flipped() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/32").unwrap();
	let addr2 = Ipv4Cidr::from_str("0.0.1.0/32").unwrap();
	let addr3 = Ipv4Cidr::from_str("0.128.0.0/9").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);
	rs.insert(&addr3);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/32\n",
			"include 0.0.1.0/32\n",
			"include 0.128.0.0/9\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.0/32\n",
			"exclude 0.0.1.0/32\n",
			"exclude 0.128.0.0/9\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_3() {
	let addr1 = Ipv4Cidr::from_str("0.0.0.0/32").unwrap();
	let addr2 = Ipv4Cidr::from_str("128.0.0.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 0.0.0.0/32\n",
			"include 128.0.0.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 0.0.0.0/32\n",
			"exclude 128.0.0.0/32\n",
		)
	);
}

#[test]
fn compress_ipv4_mixed_4() {
	let addr1 = Ipv4Cidr::from_str("128.0.0.0/32").unwrap();
	let addr2 = Ipv4Cidr::from_str("128.1.0.0/32").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr1);
	rs.insert(&addr2);

	assert_eq!(
		format_lines(Definition::compress(&rs, false)),
		concat!(
			"include 128.0.0.0/32\n",
			"include 128.1.0.0/32\n",
		)
	);

	assert_eq!(
		format_lines(Definition::compress(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
			"exclude 128.0.0.0/32\n",
			"exclude 128.1.0.0/32\n",
		)
	);
}

// tests for "complete" (aggregation is a subset of complete)

#[test]
fn complete_ipv4_empty() {
	let rs : RadixSet<Ipv4Cidr> = Default::default();

	assert_eq!(
		format_lines(Definition::complete(&rs, false)),
		concat!(
			"exclude 0.0.0.0/0\n",
		)
	);

	assert_eq!(
		format_lines(Definition::complete(&rs, true)),
		concat!(
			"include 0.0.0.0/0\n",
		)
	);
}

#[test]
fn complete_ipv4_any() {
	let addr = Ipv4Cidr::from_str("0.0.0.0/0").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr);

	assert_eq!(
		format_lines(Definition::complete(&rs, false)),
		concat!(
			"include 0.0.0.0/0\n",
		)
	);

	assert_eq!(
		format_lines(Definition::complete(&rs, true)),
		concat!(
			"exclude 0.0.0.0/0\n",
		)
	);
}

#[test]
fn complete_ipv4_single_firstbit_zero() {
	let addr = Ipv4Cidr::from_str("0.0.0.0/1").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr);

	assert_eq!(
		format_lines(Definition::complete(&rs, false)),
		concat!(
			"include 0.0.0.0/1\n",
			"exclude 128.0.0.0/1\n",
		)
	);

	assert_eq!(
		format_lines(Definition::complete(&rs, true)),
		concat!(
			"exclude 0.0.0.0/1\n",
			"include 128.0.0.0/1\n",
		)
	);
}

#[test]
fn complete_ipv4_single_firstbit_one() {
	let addr = Ipv4Cidr::from_str("128.0.0.0/1").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr);

	assert_eq!(
		format_lines(Definition::complete(&rs, false)),
		concat!(
			"exclude 0.0.0.0/1\n",
			"include 128.0.0.0/1\n",
		)
	);

	assert_eq!(
		format_lines(Definition::complete(&rs, true)),
		concat!(
			"include 0.0.0.0/1\n",
			"exclude 128.0.0.0/1\n",
		)
	);
}

#[test]
fn complete_ipv4_single_lastbit_zero() {
	let addr = Ipv4Cidr::from_str("0.0.0.0/8").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr);

	assert_eq!(
		format_lines(Definition::complete(&rs, false)),
		concat!(
			"include 0.0.0.0/8\n",
			"exclude 1.0.0.0/8\n",
			"exclude 2.0.0.0/7\n",
			"exclude 4.0.0.0/6\n",
			"exclude 8.0.0.0/5\n",
			"exclude 16.0.0.0/4\n",
			"exclude 32.0.0.0/3\n",
			"exclude 64.0.0.0/2\n",
			"exclude 128.0.0.0/1\n",
		)
	);

	assert_eq!(
		format_lines(Definition::complete(&rs, true)),
		concat!(
			"exclude 0.0.0.0/8\n",
			"include 1.0.0.0/8\n",
			"include 2.0.0.0/7\n",
			"include 4.0.0.0/6\n",
			"include 8.0.0.0/5\n",
			"include 16.0.0.0/4\n",
			"include 32.0.0.0/3\n",
			"include 64.0.0.0/2\n",
			"include 128.0.0.0/1\n",
		)
	);
}

#[test]
fn complete_ipv4_single_lastbit_one() {
	let addr = Ipv4Cidr::from_str("1.0.0.0/8").unwrap();
	let mut rs : RadixSet<Ipv4Cidr> = Default::default();
	rs.insert(&addr);

	assert_eq!(
		format_lines(Definition::complete(&rs, false)),
		concat!(
			"exclude 0.0.0.0/8\n",
			"include 1.0.0.0/8\n",
			"exclude 2.0.0.0/7\n",
			"exclude 4.0.0.0/6\n",
			"exclude 8.0.0.0/5\n",
			"exclude 16.0.0.0/4\n",
			"exclude 32.0.0.0/3\n",
			"exclude 64.0.0.0/2\n",
			"exclude 128.0.0.0/1\n",
		)
	);

	assert_eq!(
		format_lines(Definition::complete(&rs, true)),
		concat!(
			"include 0.0.0.0/8\n",
			"exclude 1.0.0.0/8\n",
			"include 2.0.0.0/7\n",
			"include 4.0.0.0/6\n",
			"include 8.0.0.0/5\n",
			"include 16.0.0.0/4\n",
			"include 32.0.0.0/3\n",
			"include 64.0.0.0/2\n",
			"include 128.0.0.0/1\n",
		)
	);
}
